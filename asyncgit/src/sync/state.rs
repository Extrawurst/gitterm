use crate::{error::Result, sync::utils};
use git2::RepositoryState;
use scopetime::scope_time;

///
#[derive(Debug, PartialEq)]
pub enum RepoState {
	///
	Clean,
	///
	Merge,
	///
	Rebase,
	///
	Other,
}

impl From<RepositoryState> for RepoState {
	fn from(state: RepositoryState) -> Self {
		match state {
			RepositoryState::Clean => Self::Clean,
			RepositoryState::Merge => Self::Merge,
			RepositoryState::RebaseMerge => Self::Rebase,
			_ => Self::Other,
		}
	}
}

///
pub fn repo_state(repo_path: &str) -> Result<RepoState> {
	scope_time!("repo_state");

	let repo = utils::repo(repo_path)?;

	let state = repo.state();

	// dbg!(&state);

	Ok(state.into())
}
