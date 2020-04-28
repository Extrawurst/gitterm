//! sync git api (various methods)

use git2::{IndexAddOption, Repository, RepositoryOpenFlags};
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

///
pub fn repo(repo_path: &str) -> Repository {
    let repo = Repository::open_ext(
        repo_path,
        RepositoryOpenFlags::empty(),
        Vec::<&Path>::new(),
    )
    .unwrap();

    if repo.is_bare() {
        panic!("bare repo")
    }

    repo
}

/// this does not run any git hooks
pub fn commit(repo_path: &str, msg: &str) {
    scope_time!("commit");

    let repo = repo(repo_path);

    let signature = repo.signature().unwrap();
    let mut index = repo.index().unwrap();
    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();

    let parents = if let Ok(reference) = repo.head() {
        let parent =
            repo.find_commit(reference.target().unwrap()).unwrap();
        vec![parent]
    } else {
        Vec::new()
    };

    let parents = parents.iter().collect::<Vec<_>>();

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        msg,
        &tree,
        parents.as_slice(),
    )
    .unwrap();
}

/// add a file diff from workingdir to stage (will not add removed files see `stage_addremoved`)
pub fn stage_add_file(repo_path: &str, path: &Path) -> bool {
    scope_time!("stage_add_file");

    let repo = repo(repo_path);

    let mut index = repo.index().unwrap();

    if index.add_path(path).is_ok() {
        index.write().unwrap();
        return true;
    }

    false
}

/// like `stage_add_file` but uses a pattern to match/glob multiple files/folders
pub fn stage_add_all(repo_path: &str, pattern: &str) -> bool {
    scope_time!("stage_add_all");

    let repo = repo(repo_path);

    let mut index = repo.index().unwrap();

    if index
        .add_all(vec![pattern], IndexAddOption::DEFAULT, None)
        .is_ok()
    {
        index.write().unwrap();
        return true;
    }

    false
}

/// stage a removed file
pub fn stage_addremoved(repo_path: &str, path: &Path) -> bool {
    scope_time!("stage_addremoved");

    let repo = repo(repo_path);

    let mut index = repo.index().unwrap();

    if index.remove_path(path).is_ok() {
        index.write().unwrap();
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::{
        status::{get_status, StatusType},
        tests::{get_statuses, repo_init, repo_init_empty},
    };
    use std::{
        fs::{self, remove_file, File},
        io::{Error, Write},
        path::Path,
    };

    #[test]
    fn test_commit() {
        let file_path = Path::new("foo");
        let (_td, repo) = repo_init();
        let root = repo.path().parent().unwrap();
        let repo_path = root.as_os_str().to_str().unwrap();

        File::create(&root.join(file_path))
            .unwrap()
            .write_all(b"test\nfoo")
            .unwrap();

        assert_eq!(get_statuses(repo_path), (1, 0));

        assert_eq!(stage_add_file(repo_path, file_path), true);

        assert_eq!(get_statuses(repo_path), (0, 1));

        commit(repo_path, "commit msg");

        assert_eq!(get_statuses(repo_path), (0, 0));
    }

    #[test]
    fn test_commit_in_empty_repo() {
        let file_path = Path::new("foo");
        let (_td, repo) = repo_init_empty();
        let root = repo.path().parent().unwrap();
        let repo_path = root.as_os_str().to_str().unwrap();

        assert_eq!(get_statuses(repo_path), (0, 0));

        File::create(&root.join(file_path))
            .unwrap()
            .write_all(b"test\nfoo")
            .unwrap();

        assert_eq!(get_statuses(repo_path), (1, 0));

        assert_eq!(stage_add_file(repo_path, file_path), true);

        assert_eq!(get_statuses(repo_path), (0, 1));

        commit(repo_path, "commit msg");

        assert_eq!(get_statuses(repo_path), (0, 0));
    }

    #[test]
    fn test_stage_add_smoke() {
        let file_path = Path::new("foo");
        let (_td, repo) = repo_init_empty();
        let root = repo.path().parent().unwrap();
        let repo_path = root.as_os_str().to_str().unwrap();

        assert_eq!(stage_add_file(repo_path, file_path), false);
    }

    #[test]
    fn test_staging_one_file() {
        let file_path = Path::new("file1.txt");
        let (_td, repo) = repo_init();
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

        assert_eq!(stage_add_file(repo_path, file_path), true);

        assert_eq!(get_statuses(repo_path), (1, 1));
    }

    #[test]
    fn test_staging_folder() -> Result<(), Error> {
        let (_td, repo) = repo_init();
        let root = repo.path().parent().unwrap();
        let repo_path = root.as_os_str().to_str().unwrap();

        let status_count = |s: StatusType| -> usize {
            get_status(repo_path, s).len()
        };

        fs::create_dir_all(&root.join("a/d"))?;
        File::create(&root.join(Path::new("a/d/f1.txt")))?
            .write_all(b"foo")?;
        File::create(&root.join(Path::new("a/d/f2.txt")))?
            .write_all(b"foo")?;
        File::create(&root.join(Path::new("a/f3.txt")))?
            .write_all(b"foo")?;

        assert_eq!(status_count(StatusType::WorkingDir), 3);

        assert_eq!(stage_add_all(repo_path, "a/d"), true);

        assert_eq!(status_count(StatusType::WorkingDir), 1);
        assert_eq!(status_count(StatusType::Stage), 2);

        Ok(())
    }

    #[test]
    fn test_staging_deleted_file() {
        let file_path = Path::new("file1.txt");
        let (_td, repo) = repo_init();
        let root = repo.path().parent().unwrap();
        let repo_path = root.as_os_str().to_str().unwrap();

        let status_count = |s: StatusType| -> usize {
            get_status(repo_path, s).len()
        };

        let full_path = &root.join(file_path);

        File::create(full_path)
            .unwrap()
            .write_all(b"test file1 content")
            .unwrap();

        assert_eq!(stage_add_file(repo_path, file_path), true);

        commit(repo_path, "commit msg");

        // delete the file now
        assert_eq!(remove_file(full_path).is_ok(), true);

        // deleted file in diff now
        assert_eq!(status_count(StatusType::WorkingDir), 1);

        assert_eq!(stage_addremoved(repo_path, file_path), true);

        assert_eq!(status_count(StatusType::WorkingDir), 0);
        assert_eq!(status_count(StatusType::Stage), 1);
    }
}
