use std::{
	fs::File,
	io::{Read, Write},
	path::PathBuf,
};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ron::{
	self,
	ser::{to_string_pretty, PrettyConfig},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Keys {
	pub tab_status: KeyEvent,
	pub tab_log: KeyEvent,
	pub tab_files: KeyEvent,
	pub tab_stashing: KeyEvent,
	pub tab_stashes: KeyEvent,
	pub tab_toggle: KeyEvent,
	pub tab_toggle_reverse: KeyEvent,
	pub toggle_workarea: KeyEvent,
	pub focus_right: KeyEvent,
	pub focus_left: KeyEvent,
	pub focus_above: KeyEvent,
	pub focus_below: KeyEvent,
	pub exit: KeyEvent,
	pub quit: KeyEvent,
	pub exit_popup: KeyEvent,
	pub open_commit: KeyEvent,
	pub open_commit_editor: KeyEvent,
	pub open_help: KeyEvent,
	pub open_options: KeyEvent,
	pub move_left: KeyEvent,
	pub move_right: KeyEvent,
	pub tree_collapse_recursive: KeyEvent,
	pub tree_expand_recursive: KeyEvent,
	pub home: KeyEvent,
	pub end: KeyEvent,
	pub move_up: KeyEvent,
	pub move_down: KeyEvent,
	pub page_down: KeyEvent,
	pub page_up: KeyEvent,
	pub shift_up: KeyEvent,
	pub shift_down: KeyEvent,
	pub enter: KeyEvent,
	pub blame: KeyEvent,
	pub edit_file: KeyEvent,
	pub status_stage_all: KeyEvent,
	pub status_reset_item: KeyEvent,
	pub status_ignore_file: KeyEvent,
	pub diff_stage_lines: KeyEvent,
	pub diff_reset_lines: KeyEvent,
	pub stashing_save: KeyEvent,
	pub stashing_toggle_untracked: KeyEvent,
	pub stashing_toggle_index: KeyEvent,
	pub stash_apply: KeyEvent,
	pub stash_open: KeyEvent,
	pub stash_drop: KeyEvent,
	pub cmd_bar_toggle: KeyEvent,
	pub log_tag_commit: KeyEvent,
	pub log_mark_commit: KeyEvent,
	pub commit_amend: KeyEvent,
	pub copy: KeyEvent,
	pub create_branch: KeyEvent,
	pub rename_branch: KeyEvent,
	pub select_branch: KeyEvent,
	pub delete_branch: KeyEvent,
	pub merge_branch: KeyEvent,
	pub rebase_branch: KeyEvent,
	pub compare_commits: KeyEvent,
	pub tags: KeyEvent,
	pub delete_tag: KeyEvent,
	pub select_tag: KeyEvent,
	pub push: KeyEvent,
	pub open_file_tree: KeyEvent,
	pub file_find: KeyEvent,
	pub force_push: KeyEvent,
	pub pull: KeyEvent,
	pub abort_merge: KeyEvent,
	pub undo_commit: KeyEvent,
	pub stage_unstage_item: KeyEvent,
}

#[rustfmt::skip]
impl Default for Keys {
	fn default() -> Self {
		Self {
			tab_status: KeyEvent { code: KeyCode::Char('1'), modifiers: KeyModifiers::empty()},
			tab_log: KeyEvent { code: KeyCode::Char('2'), modifiers: KeyModifiers::empty()},
			tab_files: KeyEvent { code: KeyCode::Char('3'), modifiers: KeyModifiers::empty()},
			tab_stashing: KeyEvent { code: KeyCode::Char('4'), modifiers: KeyModifiers::empty()},
			tab_stashes: KeyEvent { code: KeyCode::Char('5'), modifiers: KeyModifiers::empty()},
			tab_toggle: KeyEvent { code: KeyCode::Tab, modifiers: KeyModifiers::empty()},
			tab_toggle_reverse: KeyEvent { code: KeyCode::BackTab, modifiers: KeyModifiers::SHIFT},
			toggle_workarea: KeyEvent { code: KeyCode::Char('w'), modifiers: KeyModifiers::empty()},
			focus_right: KeyEvent { code: KeyCode::Right, modifiers: KeyModifiers::empty()},
			focus_left: KeyEvent { code: KeyCode::Left, modifiers: KeyModifiers::empty()},
			focus_above: KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::empty()},
			focus_below: KeyEvent { code: KeyCode::Down, modifiers: KeyModifiers::empty()},
			exit: KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL},
			quit: KeyEvent { code: KeyCode::Char('q'), modifiers: KeyModifiers::empty()},
			exit_popup: KeyEvent { code: KeyCode::Esc, modifiers: KeyModifiers::empty()},
			open_commit: KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::empty()},
			open_commit_editor: KeyEvent { code: KeyCode::Char('e'), modifiers:KeyModifiers::CONTROL},
			open_help: KeyEvent { code: KeyCode::Char('h'), modifiers: KeyModifiers::empty()},
			open_options: KeyEvent { code: KeyCode::Char('o'), modifiers: KeyModifiers::empty()},
			move_left: KeyEvent { code: KeyCode::Left, modifiers: KeyModifiers::empty()},
			move_right: KeyEvent { code: KeyCode::Right, modifiers: KeyModifiers::empty()},
			tree_collapse_recursive: KeyEvent { code: KeyCode::Left, modifiers: KeyModifiers::SHIFT},
			tree_expand_recursive: KeyEvent { code: KeyCode::Right, modifiers: KeyModifiers::SHIFT},
			home: KeyEvent { code: KeyCode::Home, modifiers: KeyModifiers::empty()},
			end: KeyEvent { code: KeyCode::End, modifiers: KeyModifiers::empty()},
			move_up: KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::empty()},
			move_down: KeyEvent { code: KeyCode::Down, modifiers: KeyModifiers::empty()},
			page_down: KeyEvent { code: KeyCode::PageDown, modifiers: KeyModifiers::empty()},
			page_up: KeyEvent { code: KeyCode::PageUp, modifiers: KeyModifiers::empty()},
			shift_up: KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::SHIFT},
			shift_down: KeyEvent { code: KeyCode::Down, modifiers: KeyModifiers::SHIFT},
			enter: KeyEvent { code: KeyCode::Enter, modifiers: KeyModifiers::empty()},
			blame: KeyEvent { code: KeyCode::Char('B'), modifiers: KeyModifiers::SHIFT},
			edit_file: KeyEvent { code: KeyCode::Char('e'), modifiers: KeyModifiers::empty()},
			status_stage_all: KeyEvent { code: KeyCode::Char('a'), modifiers: KeyModifiers::empty()},
			status_reset_item: KeyEvent { code: KeyCode::Char('D'), modifiers: KeyModifiers::SHIFT},
			diff_reset_lines: KeyEvent { code: KeyCode::Char('d'), modifiers: KeyModifiers::empty()},
			status_ignore_file: KeyEvent { code: KeyCode::Char('i'), modifiers: KeyModifiers::empty()},
			diff_stage_lines: KeyEvent { code: KeyCode::Char('s'), modifiers: KeyModifiers::empty()},
			stashing_save: KeyEvent { code: KeyCode::Char('s'), modifiers: KeyModifiers::empty()},
			stashing_toggle_untracked: KeyEvent { code: KeyCode::Char('u'), modifiers: KeyModifiers::empty()},
			stashing_toggle_index: KeyEvent { code: KeyCode::Char('i'), modifiers: KeyModifiers::empty()},
			stash_apply: KeyEvent { code: KeyCode::Char('a'), modifiers: KeyModifiers::empty()},
			stash_open: KeyEvent { code: KeyCode::Right, modifiers: KeyModifiers::empty()},
			stash_drop: KeyEvent { code: KeyCode::Char('D'), modifiers: KeyModifiers::SHIFT},
			cmd_bar_toggle: KeyEvent { code: KeyCode::Char('.'), modifiers: KeyModifiers::empty()},
			log_tag_commit: KeyEvent { code: KeyCode::Char('t'), modifiers: KeyModifiers::empty()},
			log_mark_commit: KeyEvent { code: KeyCode::Char(' '), modifiers: KeyModifiers::empty()},
			commit_amend: KeyEvent { code: KeyCode::Char('a'), modifiers: KeyModifiers::CONTROL},
			copy: KeyEvent { code: KeyCode::Char('y'), modifiers: KeyModifiers::empty()},
			create_branch: KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::empty()},
			rename_branch: KeyEvent { code: KeyCode::Char('r'), modifiers: KeyModifiers::empty()},
			select_branch: KeyEvent { code: KeyCode::Char('b'), modifiers: KeyModifiers::empty()},
			delete_branch: KeyEvent { code: KeyCode::Char('D'), modifiers: KeyModifiers::SHIFT},
			merge_branch: KeyEvent { code: KeyCode::Char('m'), modifiers: KeyModifiers::empty()},
			rebase_branch: KeyEvent { code: KeyCode::Char('R'), modifiers: KeyModifiers::SHIFT},
			compare_commits: KeyEvent { code: KeyCode::Char('C'), modifiers: KeyModifiers::SHIFT},
			tags: KeyEvent { code: KeyCode::Char('T'), modifiers: KeyModifiers::SHIFT},
			delete_tag: KeyEvent { code: KeyCode::Char('D'), modifiers: KeyModifiers::SHIFT},
			select_tag: KeyEvent { code: KeyCode::Enter, modifiers: KeyModifiers::empty()},
			push: KeyEvent { code: KeyCode::Char('p'), modifiers: KeyModifiers::empty()},
			force_push: KeyEvent { code: KeyCode::Char('P'), modifiers: KeyModifiers::SHIFT},
			undo_commit: KeyEvent { code: KeyCode::Char('U'), modifiers: KeyModifiers::SHIFT},
			pull: KeyEvent { code: KeyCode::Char('f'), modifiers: KeyModifiers::empty()},
			abort_merge: KeyEvent { code: KeyCode::Char('A'), modifiers: KeyModifiers::SHIFT},
			open_file_tree: KeyEvent { code: KeyCode::Char('F'), modifiers: KeyModifiers::SHIFT},
			file_find: KeyEvent { code: KeyCode::Char('f'), modifiers: KeyModifiers::empty()},
			stage_unstage_item: KeyEvent { code: KeyCode::Enter, modifiers: KeyModifiers::empty()},
		}
	}
}

impl Keys {
	pub fn save(&self, file: PathBuf) -> Result<()> {
		let mut file = File::create(file)?;
		let data = to_string_pretty(self, PrettyConfig::default())?;
		file.write_all(data.as_bytes())?;
		Ok(())
	}

	pub fn read_file(config_file: PathBuf) -> Result<Self> {
		let mut f = File::open(config_file)?;
		let mut buffer = Vec::new();
		f.read_to_end(&mut buffer)?;
		Ok(ron::de::from_bytes(&buffer)?)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_load_vim_style_example() {
		assert_eq!(
			Keys::read_file("vim_style_key_config.ron".into())
				.is_ok(),
			true
		);
	}
}
