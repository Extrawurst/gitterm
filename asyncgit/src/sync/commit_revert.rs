use super::{CommitId, RepoPath};
use crate::{
	error::Result,
	sync::{repository::repo, utils::read_file},
};
use scopetime::scope_time;

const GIT_REVERT_HEAD_FILE: &str = "REVERT_HEAD";

///
pub fn revert_commit(
	repo_path: &RepoPath,
	commit: CommitId,
) -> Result<()> {
	scope_time!("revert");

	let repo = repo(repo_path)?;

	let commit = repo.find_commit(commit.into())?;

	repo.revert(&commit, None)?;

	Ok(())
}

///
pub fn revert_head(repo_path: &RepoPath) -> Result<CommitId> {
	scope_time!("revert_head");

	let path = repo(repo_path)?.path().join(GIT_REVERT_HEAD_FILE);

	let file_content = read_file(&path)?;

	let id = git2::Oid::from_str(&file_content)?;

	Ok(id.into())
}

///
pub fn abort_revert(repo_path: &RepoPath) -> Result<()> {
	scope_time!("abort_revert");

	//TODO: revert all changes in index and workdir

	std::fs::remove_file(
		repo(repo_path)?.path().join(GIT_REVERT_HEAD_FILE),
	)?;

	Ok(())
}
