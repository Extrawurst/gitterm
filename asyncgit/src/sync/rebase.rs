use git2::{BranchType, Repository};
use scopetime::scope_time;

use crate::{
	error::{Error, Result},
	sync::utils,
};

use super::CommitId;

/// rebase current HEAD on `branch`
pub fn rebase_branch(
	repo_path: &str,
	branch: &str,
) -> Result<RebaseState> {
	scope_time!("rebase_branch");

	let repo = utils::repo(repo_path)?;

	rebase_branch_repo(&repo, branch)
}

fn rebase_branch_repo(
	repo: &Repository,
	branch_name: &str,
) -> Result<RebaseState> {
	let branch = repo.find_branch(branch_name, BranchType::Local)?;

	let annotated =
		repo.reference_to_annotated_commit(&branch.into_reference())?;

	rebase(repo, &annotated)
}

/// rebase attempt which aborts and undo's rebase if any conflict appears
pub fn conflict_free_rebase(
	repo: &git2::Repository,
	commit: &git2::AnnotatedCommit,
) -> Result<CommitId> {
	let mut rebase = repo.rebase(None, Some(commit), None, None)?;
	let signature =
		crate::sync::commit::signature_allow_undefined_name(repo)?;
	let mut last_commit = None;
	while let Some(op) = rebase.next() {
		let _op = op?;

		if repo.index()?.has_conflicts() {
			rebase.abort()?;
			return Err(Error::RebaseConflict);
		}

		let c = rebase.commit(None, &signature, None)?;

		last_commit = Some(CommitId::from(c));
	}

	if repo.index()?.has_conflicts() {
		rebase.abort()?;
		return Err(Error::RebaseConflict);
	}

	rebase.finish(Some(&signature))?;

	last_commit.ok_or_else(|| {
		Error::Generic(String::from("no commit rebased"))
	})
}

///
#[derive(PartialEq, Debug)]
pub enum RebaseState {
	///
	Finished,
	///
	Conflicted,
}

/// rebase
pub fn rebase(
	repo: &git2::Repository,
	commit: &git2::AnnotatedCommit,
) -> Result<RebaseState> {
	let mut rebase = repo.rebase(None, Some(commit), None, None)?;
	let signature =
		crate::sync::commit::signature_allow_undefined_name(repo)?;

	while let Some(op) = rebase.next() {
		let _op = op?;
		// dbg!(op.id());

		if repo.index()?.has_conflicts() {
			return Ok(RebaseState::Conflicted);
		}

		rebase.commit(None, &signature, None)?;
	}

	if repo.index()?.has_conflicts() {
		return Ok(RebaseState::Conflicted);
	}

	rebase.finish(Some(&signature))?;

	Ok(RebaseState::Finished)
}

/// continue pending rebase
pub fn continue_rebase(
	repo: &git2::Repository,
) -> Result<RebaseState> {
	let mut rebase = repo.open_rebase(None)?;
	let signature =
		crate::sync::commit::signature_allow_undefined_name(repo)?;

	if repo.index()?.has_conflicts() {
		return Ok(RebaseState::Conflicted);
	}

	// try commit current rebase step
	if !repo.index()?.is_empty() {
		rebase.commit(None, &signature, None)?;
	}

	while let Some(op) = rebase.next() {
		let _op = op?;
		// dbg!(op.id());

		if repo.index()?.has_conflicts() {
			return Ok(RebaseState::Conflicted);
		}

		rebase.commit(None, &signature, None)?;
	}

	if repo.index()?.has_conflicts() {
		return Ok(RebaseState::Conflicted);
	}

	rebase.finish(Some(&signature))?;

	Ok(RebaseState::Finished)
}

///
#[derive(PartialEq, Debug)]
pub struct RebaseProgress {
	///
	pub steps: usize,
	///
	pub current: usize,
	///
	pub current_commit: Option<CommitId>,
}

///
pub fn get_rebase_progress(
	repo: &git2::Repository,
) -> Result<RebaseProgress> {
	let mut rebase = repo.open_rebase(None)?;

	let current_commit: Option<CommitId> = rebase
		.operation_current()
		.and_then(|idx| rebase.nth(idx))
		.map(|op| op.id().into());

	let progress = RebaseProgress {
		steps: rebase.len(),
		current: rebase.operation_current().unwrap_or_default(),
		current_commit,
	};

	Ok(progress)
}

///
pub fn abort_rebase(repo: &git2::Repository) -> Result<()> {
	let mut rebase = repo.open_rebase(None)?;

	rebase.abort()?;

	Ok(())
}

#[cfg(test)]
mod test_conflict_free_rebase {
	use crate::sync::{
		checkout_branch, create_branch,
		rebase::{rebase_branch, RebaseState},
		repo_state,
		tests::{repo_init, write_commit_file},
		utils, CommitId, RepoState,
	};
	use git2::{BranchType, Repository};

	use super::conflict_free_rebase;

	fn parent_ids(repo: &Repository, c: CommitId) -> Vec<CommitId> {
		let foo = repo
			.find_commit(c.into())
			.unwrap()
			.parent_ids()
			.map(|id| CommitId::from(id))
			.collect();

		foo
	}

	///
	fn test_rebase_branch_repo(
		repo_path: &str,
		branch_name: &str,
	) -> CommitId {
		let repo = utils::repo(repo_path).unwrap();

		let branch =
			repo.find_branch(branch_name, BranchType::Local).unwrap();

		let annotated = repo
			.reference_to_annotated_commit(&branch.into_reference())
			.unwrap();

		conflict_free_rebase(&repo, &annotated).unwrap()
	}

	#[test]
	fn test_smoke() {
		let (_td, repo) = repo_init().unwrap();
		let root = repo.path().parent().unwrap();
		let repo_path = root.as_os_str().to_str().unwrap();

		let c1 =
			write_commit_file(&repo, "test1.txt", "test", "commit1");

		create_branch(repo_path, "foo").unwrap();

		let c2 =
			write_commit_file(&repo, "test2.txt", "test", "commit2");

		assert_eq!(parent_ids(&repo, c2), vec![c1]);

		checkout_branch(repo_path, "refs/heads/master").unwrap();

		let c3 =
			write_commit_file(&repo, "test3.txt", "test", "commit3");

		checkout_branch(repo_path, "refs/heads/foo").unwrap();

		let r = test_rebase_branch_repo(repo_path, "master");

		assert_eq!(parent_ids(&repo, r), vec![c3]);
	}

	#[test]
	fn test_conflict() {
		let (_td, repo) = repo_init().unwrap();
		let root = repo.path().parent().unwrap();
		let repo_path = root.as_os_str().to_str().unwrap();

		write_commit_file(&repo, "test.txt", "test1", "commit1");

		create_branch(repo_path, "foo").unwrap();

		write_commit_file(&repo, "test.txt", "test2", "commit2");

		checkout_branch(repo_path, "refs/heads/master").unwrap();

		write_commit_file(&repo, "test.txt", "test3", "commit3");

		checkout_branch(repo_path, "refs/heads/foo").unwrap();

		let res = rebase_branch(repo_path, "master");

		assert!(matches!(res.unwrap(), RebaseState::Conflicted));

		assert_eq!(repo_state(repo_path).unwrap(), RepoState::Rebase);
	}
}

#[cfg(test)]
mod test_rebase {
	use crate::sync::{
		checkout_branch, create_branch,
		rebase::{
			abort_rebase, get_rebase_progress, RebaseProgress,
			RebaseState,
		},
		rebase_branch, repo_state,
		tests::{repo_init, write_commit_file},
		RepoState,
	};

	#[test]
	fn test_conflicted_abort() {
		let (_td, repo) = repo_init().unwrap();
		let root = repo.path().parent().unwrap();
		let repo_path = root.as_os_str().to_str().unwrap();

		write_commit_file(&repo, "test.txt", "test1", "commit1");

		create_branch(repo_path, "foo").unwrap();

		let c =
			write_commit_file(&repo, "test.txt", "test2", "commit2");

		checkout_branch(repo_path, "refs/heads/master").unwrap();

		write_commit_file(&repo, "test.txt", "test3", "commit3");

		checkout_branch(repo_path, "refs/heads/foo").unwrap();

		assert!(get_rebase_progress(&repo).is_err());

		// rebase

		let r = rebase_branch(repo_path, "master").unwrap();

		assert_eq!(r, RebaseState::Conflicted);
		assert_eq!(repo_state(repo_path).unwrap(), RepoState::Rebase);
		assert_eq!(
			get_rebase_progress(&repo).unwrap(),
			RebaseProgress {
				current: 0,
				steps: 1,
				current_commit: Some(c)
			}
		);

		// abort

		abort_rebase(&repo).unwrap();

		assert_eq!(repo_state(repo_path).unwrap(), RepoState::Clean);
	}
}
