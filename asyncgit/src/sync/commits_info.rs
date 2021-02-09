use super::utils::repo;
use crate::error::Result;
use git2::{Commit, Error, Oid};
use scopetime::scope_time;

/// identifies a single commit
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct CommitId(Oid);

impl CommitId {
    /// create new CommitId
    pub fn new(id: Oid) -> Self {
        Self(id)
    }

    ///
    pub(crate) fn get_oid(self) -> Oid {
        self.0
    }

    ///
    pub fn get_short_string(&self) -> String {
        self.to_string().chars().take(7).collect()
    }
}

impl ToString for CommitId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<CommitId> for Oid {
    fn from(id: CommitId) -> Self {
        id.0
    }
}

impl From<Oid> for CommitId {
    fn from(id: Oid) -> Self {
        Self::new(id)
    }
}

///
#[derive(Debug, Clone)]
pub struct CommitInfo {
    ///
    pub message: String,
    ///
    pub time: i64,
    ///
    pub author: String,
    ///
    pub id: CommitId,
}

///
pub fn get_commits_info(
    repo_path: &str,
    ids: &[CommitId],
    message_length_limit: usize,
) -> Result<Vec<CommitInfo>> {
    scope_time!("get_commits_info");

    let repo = repo(repo_path)?;

    let commits = ids
        .iter()
        .map(|id| repo.find_commit((*id).into()))
        .collect::<std::result::Result<Vec<Commit>, Error>>()?
        .into_iter();

    let res = commits
        .map(|c: Commit| {
            let message = get_message(&c, Some(message_length_limit));
            let author = if let Some(name) = c.author().name() {
                String::from(name)
            } else {
                String::from("<unknown>")
            };
            CommitInfo {
                message,
                author,
                time: c.time().seconds(),
                id: CommitId(c.id()),
            }
        })
        .collect::<Vec<_>>();

    Ok(res)
}

///
pub fn get_message(
    c: &Commit,
    message_length_limit: Option<usize>,
) -> String {
    let msg = String::from_utf8_lossy(c.message_bytes());
    let msg = msg.trim_start();

    if let Some(limit) = message_length_limit {
        limit_str(msg, limit).to_string()
    } else {
        msg.to_string()
    }
}

#[inline]
///
pub fn limit_str(s: &str, limit: usize) -> &str {
    if let Some(first) = s.lines().next() {
        let mut limit = limit.min(first.len());
        while !first.is_char_boundary(limit) {
            limit += 1
        }
        &first[0..limit]
    } else {
        ""
    }
}

#[cfg(test)]
mod tests {

    use super::{get_commits_info, limit_str};
    use crate::error::Result;
    use crate::sync::{
        commit, stage_add_file, tests::repo_init_empty,
        utils::get_head_repo,
    };
    use std::{fs::File, io::Write, path::Path};

    #[test]
    fn test_log() -> Result<()> {
        let file_path = Path::new("foo");
        let (_td, repo) = repo_init_empty().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path = root.as_os_str().to_str().unwrap();

        File::create(&root.join(file_path))?.write_all(b"a")?;
        stage_add_file(repo_path, file_path).unwrap();
        let c1 = commit(repo_path, "commit1").unwrap();
        File::create(&root.join(file_path))?.write_all(b"a")?;
        stage_add_file(repo_path, file_path).unwrap();
        let c2 = commit(repo_path, "commit2").unwrap();

        let res =
            get_commits_info(repo_path, &vec![c2, c1], 50).unwrap();

        assert_eq!(res.len(), 2);
        assert_eq!(res[0].message.as_str(), "commit2");
        assert_eq!(res[0].author.as_str(), "name");
        assert_eq!(res[1].message.as_str(), "commit1");

        Ok(())
    }

    #[test]
    fn test_invalid_utf8() -> Result<()> {
        let file_path = Path::new("foo");
        let (_td, repo) = repo_init_empty().unwrap();
        let root = repo.path().parent().unwrap();
        let repo_path = root.as_os_str().to_str().unwrap();

        File::create(&root.join(file_path))?.write_all(b"a")?;
        stage_add_file(repo_path, file_path).unwrap();

        let msg = invalidstring::invalid_utf8("test msg");
        commit(repo_path, msg.as_str()).unwrap();

        let res = get_commits_info(
            repo_path,
            &vec![get_head_repo(&repo).unwrap().into()],
            50,
        )
        .unwrap();

        assert_eq!(res.len(), 1);
        dbg!(&res[0].message);
        assert_eq!(res[0].message.starts_with("test msg"), true);

        Ok(())
    }

    #[test]
    fn test_limit_string_utf8() {
        assert_eq!(limit_str("里里", 1), "里");

        let test_src = "导入按钮由选文件改为选目录，因为整个过程中要用到多个mdb文件，这些文件是在程序里写死的，暂且这么来做，有时间了后 再做调整";
        let test_dst = "导入按钮由选文";
        assert_eq!(limit_str(test_src, 20), test_dst);
    }
}
