//! sync git api (various methods)

use super::CommitId;
use crate::error::{Error, Result};
use git2::{IndexAddOption, Oid, Repository, RepositoryOpenFlags};
use scopetime::scope_time;
use std::path::Path;

///
pub fn is_repo(repo_path: &str) -> bool {
    Repository::open_ext(
        repo_path,
        RepositoryOpenFlags::empty(),
        Vec::<&Path>::new(),
    )
    .is_ok()
}

/// checks if the git repo at path `repo_path` is a bare repo
pub fn is_bare_repo(repo_path: &str) -> Result<bool> {
    let repo = Repository::open_ext(
        repo_path,
        RepositoryOpenFlags::empty(),
        Vec::<&Path>::new(),
    )?;

    Ok(repo.is_bare())
}

///
pub fn repo(repo_path: &str) -> Result<Repository> {
    let repo = Repository::open_ext(
        repo_path,
        RepositoryOpenFlags::empty(),
        Vec::<&Path>::new(),
    )?;

    if repo.is_bare() {
        return Err(Error::Generic("bare repo".to_string()));
    }

    Ok(repo)
}

///
pub fn work_dir(repo: &Repository) -> &Path {
    repo.workdir().expect("unable to query workdir")
}

///
pub fn get_head(repo_path: &str) -> Result<CommitId> {
    let repo = repo(repo_path)?;
    get_head_repo(&repo)
}

///
pub fn get_head_repo(repo: &Repository) -> Result<CommitId> {
    scope_time!("get_head_repo");

    let head = repo.head()?.target();

    if let Some(head_id) = head {
        Ok(CommitId::new(head_id))
    } else {
        Err(Error::NoHead)
    }
}

/// Wrap Repository::signature to allow unknown user.name.
///
/// See <https://github.com/extrawurst/gitui/issues/79>.
pub fn signature_allow_undefined_name(
    repo: &Repository,
) -> std::result::Result<git2::Signature<'static>, git2::Error> {
    match repo.signature() {
        Err(e) if e.code() == git2::ErrorCode::NotFound => {
            let config = repo.config()?;
            git2::Signature::now(
                config.get_str("user.name").unwrap_or("unknown"),
                config.get_str("user.email")?,
            )
        }

        v => v,
    }
}

/// ditto
pub fn commit_new(repo_path: &str, msg: &str) -> Result<CommitId> {
    commit(repo_path, msg).map(CommitId::new)
}

/// this does not run any git hooks
pub fn commit(repo_path: &str, msg: &str) -> Result<Oid> {
    scope_time!("commit");

    let repo = repo(repo_path)?;

    let signature = signature_allow_undefined_name(&repo)?;
    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let parents = if let Ok(id) = get_head(repo_path) {
        vec![repo.find_commit(id.into())?]
    } else {
        Vec::new()
    };

    let parents = parents.iter().collect::<Vec<_>>();

    Ok(repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        msg,
        &tree,
        parents.as_slice(),
    )?)
}

/// add a file diff from workingdir to stage (will not add removed files see `stage_addremoved`)
pub fn stage_add_file(repo_path: &str, path: &Path) -> Result<()> {
    scope_time!("stage_add_file");

    let repo = repo(repo_path)?;

    let mut index = repo.index()?;

    index.add_path(path)?;
    index.write()?;

    Ok(())
}

/// like `stage_add_file` but uses a pattern to match/glob multiple files/folders
pub fn stage_add_all(repo_path: &str, pattern: &str) -> Result<()> {
    scope_time!("stage_add_all");

    let repo = repo(repo_path)?;

    let mut index = repo.index()?;

    index.add_all(vec![pattern], IndexAddOption::DEFAULT, None)?;
    index.write()?;

    Ok(())
}

/// stage a removed file
pub fn stage_addremoved(repo_path: &str, path: &Path) -> Result<()> {
    scope_time!("stage_addremoved");

    let repo = repo(repo_path)?;

    let mut index = repo.index()?;

    index.remove_path(path)?;
    index.write()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::{
        status::{get_status, StatusType},
        tests::{
            debug_cmd_print, get_statuses, repo_init, repo_init_empty,
        },
    };
    use std::{
        fs::{self, remove_file, File},
        io::Write,
        path::Path,
    };

    #[test]
    fn test_commit() {
        let file_path = Path::new("foo");
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path = root.as_os_str().to_str().unwrap();

        File::create(&root.join(file_path))
            .unwrap()
            .write_all(b"test\nfoo")
            .unwrap();

        assert_eq!(get_statuses(repo_path), (1, 0));

        stage_add_file(repo_path, file_path).unwrap();

        assert_eq!(get_statuses(repo_path), (0, 1));

        commit(repo_path, "commit msg").unwrap();

        assert_eq!(get_statuses(repo_path), (0, 0));
    }

    #[test]
    fn test_commit_in_empty_repo() {
        let file_path = Path::new("foo");
        let (_td, repo) = repo_init_empty().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path = root.as_os_str().to_str().unwrap();

        assert_eq!(get_statuses(repo_path), (0, 0));

        File::create(&root.join(file_path))
            .unwrap()
            .write_all(b"test\nfoo")
            .unwrap();

        assert_eq!(get_statuses(repo_path), (1, 0));

        stage_add_file(repo_path, file_path).unwrap();

        assert_eq!(get_statuses(repo_path), (0, 1));

        commit(repo_path, "commit msg").unwrap();

        assert_eq!(get_statuses(repo_path), (0, 0));
    }

    #[test]
    fn test_stage_add_smoke() {
        let file_path = Path::new("foo");
        let (_td, repo) = repo_init_empty().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path = root.as_os_str().to_str().unwrap();

        assert_eq!(
            stage_add_file(repo_path, file_path).is_ok(),
            false
        );
    }

    #[test]
    fn test_staging_one_file() {
        let file_path = Path::new("file1.txt");
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path = root.as_os_str().to_str().unwrap();

        File::create(&root.join(file_path))
            .unwrap()
            .write_all(b"test file1 content")
            .unwrap();

        File::create(&root.join(Path::new("file2.txt")))
            .unwrap()
            .write_all(b"test file2 content")
            .unwrap();

        assert_eq!(get_statuses(repo_path), (2, 0));

        stage_add_file(repo_path, file_path).unwrap();

        assert_eq!(get_statuses(repo_path), (1, 1));
    }

    #[test]
    fn test_staging_folder() -> Result<()> {
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path = root.as_os_str().to_str().unwrap();

        let status_count = |s: StatusType| -> usize {
            get_status(repo_path, s, true).unwrap().len()
        };

        fs::create_dir_all(&root.join("a/d"))?;
        File::create(&root.join(Path::new("a/d/f1.txt")))?
            .write_all(b"foo")?;
        File::create(&root.join(Path::new("a/d/f2.txt")))?
            .write_all(b"foo")?;
        File::create(&root.join(Path::new("a/f3.txt")))?
            .write_all(b"foo")?;

        assert_eq!(status_count(StatusType::WorkingDir), 3);

        stage_add_all(repo_path, "a/d").unwrap();

        assert_eq!(status_count(StatusType::WorkingDir), 1);
        assert_eq!(status_count(StatusType::Stage), 2);

        Ok(())
    }

    #[test]
    fn test_staging_deleted_file() {
        let file_path = Path::new("file1.txt");
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path = root.as_os_str().to_str().unwrap();

        let status_count = |s: StatusType| -> usize {
            get_status(repo_path, s, true).unwrap().len()
        };

        let full_path = &root.join(file_path);

        File::create(full_path)
            .unwrap()
            .write_all(b"test file1 content")
            .unwrap();

        stage_add_file(repo_path, file_path).unwrap();

        commit(repo_path, "commit msg").unwrap();

        // delete the file now
        assert_eq!(remove_file(full_path).is_ok(), true);

        // deleted file in diff now
        assert_eq!(status_count(StatusType::WorkingDir), 1);

        stage_addremoved(repo_path, file_path).unwrap();

        assert_eq!(status_count(StatusType::WorkingDir), 0);
        assert_eq!(status_count(StatusType::Stage), 1);
    }

    // see https://github.com/extrawurst/gitui/issues/108
    #[test]
    fn test_staging_sub_git_folder() -> Result<()> {
        let (_td, repo) = repo_init().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path = root.as_os_str().to_str().unwrap();

        let status_count = |s: StatusType| -> usize {
            get_status(repo_path, s, true).unwrap().len()
        };

        let sub = &root.join("sub");

        fs::create_dir_all(sub)?;

        debug_cmd_print(sub.to_str().unwrap(), "git init subgit");

        File::create(sub.join("subgit/foo.txt"))
            .unwrap()
            .write_all(b"content")
            .unwrap();

        assert_eq!(status_count(StatusType::WorkingDir), 1);

        //expect to fail
        assert!(stage_add_all(repo_path, "sub").is_err());

        Ok(())
    }

    #[test]
    fn test_head_empty() -> Result<()> {
        let (_td, repo) = repo_init_empty()?;
        let root = repo.path().parent().unwrap();
        let repo_path = root.as_os_str().to_str().unwrap();

        assert_eq!(get_head(repo_path).is_ok(), false);

        Ok(())
    }

    #[test]
    fn test_head() -> Result<()> {
        let (_td, repo) = repo_init()?;
        let root = repo.path().parent().unwrap();
        let repo_path = root.as_os_str().to_str().unwrap();

        assert_eq!(get_head(repo_path).is_ok(), true);

        Ok(())
    }
}
