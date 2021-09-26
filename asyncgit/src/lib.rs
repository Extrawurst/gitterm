//! asyncgit

#![forbid(missing_docs)]
#![deny(
	unused_imports,
	unused_must_use,
	dead_code,
	unstable_name_collisions,
	unused_assignments
)]
#![deny(unstable_name_collisions)]
#![deny(clippy::all, clippy::perf, clippy::nursery, clippy::pedantic)]
#![deny(clippy::filetype_is_file)]
#![deny(clippy::cargo)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::panic)]
#![deny(clippy::match_like_matches_macro)]
#![deny(clippy::needless_update)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::missing_errors_doc)]
//TODO: get this in someday since expect still leads us to crashes sometimes
// #![deny(clippy::expect_used)]

pub mod asyncjob;
mod blame;
pub mod cached;
mod commit_files;
mod diff;
mod error;
mod fetch;
mod progress;
mod push;
mod push_tags;
pub mod remote_progress;
pub mod remote_tags;
mod revlog;
mod status;
pub mod sync;
mod tags;

pub use crate::{
	blame::{AsyncBlame, BlameParams},
	commit_files::{AsyncCommitFiles, CommitFilesParams},
	diff::{AsyncDiff, DiffParams, DiffType},
	error::{Error, Result},
	fetch::{AsyncFetch, FetchRequest},
	progress::ProgressPercent,
	push::{AsyncPush, PushRequest},
	push_tags::{AsyncPushTags, PushTagsRequest},
	remote_progress::{RemoteProgress, RemoteProgressState},
	revlog::{AsyncLog, FetchStatus},
	status::{AsyncStatus, StatusParams},
	sync::{
		diff::{DiffLine, DiffLineType, FileDiff},
		status::{StatusItem, StatusItemType},
	},
	tags::AsyncTags,
};
use std::{
	collections::hash_map::DefaultHasher,
	hash::{Hash, Hasher},
};

/// this type is used to communicate events back through the channel
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AsyncGitNotification {
	/// this indicates that no new state was fetched but that a async process finished
	FinishUnchanged,
	///
	Status,
	///
	Diff,
	///
	Log,
	///
	CommitFiles,
	///
	Tags,
	///
	Push,
	///
	PushTags,
	///
	Fetch,
	///
	Blame,
	///
	RemoteTags,
}

/// current working directory `./`
pub static CWD: &str = "./";

/// helper function to calculate the hash of an arbitrary type that implements the `Hash` trait
pub fn hash<T: Hash + ?Sized>(v: &T) -> u64 {
	let mut hasher = DefaultHasher::new();
	v.hash(&mut hasher);
	hasher.finish()
}

///
#[cfg(feature = "trace-libgit")]
pub fn register_tracing_logging() -> bool {
	fn git_trace(level: git2::TraceLevel, msg: &str) {
		log::info!("[{:?}]: {}", level, msg);
	}
	git2::trace_set(git2::TraceLevel::Trace, git_trace)
}

///
#[cfg(not(feature = "trace-libgit"))]
pub fn register_tracing_logging() -> bool {
	true
}
