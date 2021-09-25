use super::{
	textinput::TextInputComponent, visibility_blocking,
	CommandBlocking, CommandInfo, Component, DrawableComponent,
	EventState, ExternalEditorComponent,
};
use crate::{
	keys::SharedKeyConfig,
	queue::{InternalEvent, NeedsUpdate, Queue},
	strings, try_or_popup,
	ui::style::SharedTheme,
};
use anyhow::Result;
use asyncgit::{
	cached,
	sync::{
		self, get_config_string, CommitId, HookResult, RepoState,
	},
	CWD,
};
use crossterm::event::Event;
use easy_cast::Cast;
use std::{
	fs::{read_to_string, File},
	io::{Read, Write},
};
use tui::{
	backend::Backend,
	layout::{Alignment, Rect},
	widgets::Paragraph,
	Frame,
};

enum Mode {
	Normal,
	Amend(CommitId),
	Merge(Vec<CommitId>),
}

pub struct CommitComponent {
	input: TextInputComponent,
	mode: Mode,
	queue: Queue,
	key_config: SharedKeyConfig,
	git_branch_name: cached::BranchName,
	commit_template: Option<String>,
	theme: SharedTheme,
}

const FIRST_LINE_LIMIT: usize = 50;

impl CommitComponent {
	///
	pub fn new(
		queue: Queue,
		theme: SharedTheme,
		key_config: SharedKeyConfig,
	) -> Self {
		Self {
			queue,
			mode: Mode::Normal,

			input: TextInputComponent::new(
				theme.clone(),
				key_config.clone(),
				"",
				&strings::commit_msg(&key_config),
				true,
			),
			key_config,
			git_branch_name: cached::BranchName::new(CWD),
			commit_template: None,
			theme,
		}
	}

	///
	pub fn update(&mut self) {
		self.git_branch_name.lookup().ok();
	}

	fn draw_branch_name<B: Backend>(&self, f: &mut Frame<B>) {
		if let Some(name) = self.git_branch_name.last() {
			let w = Paragraph::new(format!("{{{}}}", name))
				.alignment(Alignment::Right);

			let rect = {
				let mut rect = self.input.get_area();
				rect.height = 1;
				rect.width = rect.width.saturating_sub(1);
				rect
			};

			f.render_widget(w, rect);
		}
	}

	fn draw_warnings<B: Backend>(&self, f: &mut Frame<B>) {
		let first_line = self
			.input
			.get_text()
			.lines()
			.next()
			.map(str::len)
			.unwrap_or_default();

		if first_line > FIRST_LINE_LIMIT {
			let msg = strings::commit_first_line_warning(first_line);
			let msg_length: u16 = msg.len().cast();
			let w =
				Paragraph::new(msg).style(self.theme.text_danger());

			let rect = {
				let mut rect = self.input.get_area();
				rect.y += rect.height.saturating_sub(1);
				rect.height = 1;
				let offset =
					rect.width.saturating_sub(msg_length + 1);
				rect.width = rect.width.saturating_sub(offset + 1);
				rect.x += offset;

				rect
			};

			f.render_widget(w, rect);
		}
	}

	pub fn show_editor(&mut self) -> Result<()> {
		let file_path = sync::repo_dir(CWD)?.join("COMMIT_EDITMSG");

		{
			let mut file = File::create(&file_path)?;
			file.write_fmt(format_args!(
				"{}\n",
				self.input.get_text()
			))?;
			file.write_all(
				strings::commit_editor_msg(&self.key_config)
					.as_bytes(),
			)?;
		}

		ExternalEditorComponent::open_file_in_editor(&file_path)?;

		let mut message = String::new();

		let mut file = File::open(&file_path)?;
		file.read_to_string(&mut message)?;
		drop(file);
		std::fs::remove_file(&file_path)?;

		let message: String = message
			.lines()
			.flat_map(|l| {
				if l.starts_with('#') {
					vec![]
				} else {
					vec![l, "\n"]
				}
			})
			.collect();

		let message = message.trim().to_string();

		self.input.set_text(message);
		self.input.show()?;

		Ok(())
	}

	fn commit(&mut self) -> Result<()> {
		let msg = self.input.get_text().to_string();
		self.input.clear();
		self.commit_with_msg(msg)
	}

	fn commit_with_msg(&mut self, msg: String) -> Result<()> {
		if let HookResult::NotOk(e) = sync::hooks_pre_commit(CWD)? {
			log::error!("pre-commit hook error: {}", e);
			self.queue.push(InternalEvent::ShowErrorMsg(format!(
				"pre-commit hook error:\n{}",
				e
			)));
			return Ok(());
		}
		let mut msg = msg;
		if let HookResult::NotOk(e) =
			sync::hooks_commit_msg(CWD, &mut msg)?
		{
			log::error!("commit-msg hook error: {}", e);
			self.queue.push(InternalEvent::ShowErrorMsg(format!(
				"commit-msg hook error:\n{}",
				e
			)));
			return Ok(());
		}

		let res = match &self.mode {
			Mode::Normal => sync::commit(CWD, &msg),
			Mode::Amend(amend) => sync::amend(CWD, *amend, &msg),
			Mode::Merge(ids) => sync::merge_commit(CWD, &msg, ids),
		};

		if let Err(e) = res {
			log::error!("commit error: {}", &e);
			self.queue.push(InternalEvent::ShowErrorMsg(format!(
				"commit failed:\n{}",
				&e
			)));
			return Ok(());
		}

		if let HookResult::NotOk(e) = sync::hooks_post_commit(CWD)? {
			log::error!("post-commit hook error: {}", e);
			self.queue.push(InternalEvent::ShowErrorMsg(format!(
				"post-commit hook error:\n{}",
				e
			)));
		}

		self.hide();

		self.queue.push(InternalEvent::Update(NeedsUpdate::ALL));

		Ok(())
	}

	fn can_commit(&self) -> bool {
		!self.is_empty() && self.is_changed()
	}

	fn can_amend(&self) -> bool {
		matches!(self.mode, Mode::Normal)
			&& sync::get_head(CWD).is_ok()
			&& (self.is_empty() || !self.is_changed())
	}

	fn is_empty(&self) -> bool {
		self.input.get_text().is_empty()
	}

	fn is_changed(&self) -> bool {
		Some(self.input.get_text().trim())
			!= self.commit_template.as_ref().map(|s| s.trim())
	}

	fn amend(&mut self) -> Result<()> {
		if self.can_amend() {
			let id = sync::get_head(CWD)?;
			self.mode = Mode::Amend(id);

			let details = sync::get_commit_details(CWD, id)?;

			self.input.set_title(strings::commit_title_amend());

			if let Some(msg) = details.message {
				self.input.set_text(msg.combine());
			}
		}

		Ok(())
	}
}

impl DrawableComponent for CommitComponent {
	fn draw<B: Backend>(
		&self,
		f: &mut Frame<B>,
		rect: Rect,
	) -> Result<()> {
		if self.is_visible() {
			self.input.draw(f, rect)?;
			self.draw_branch_name(f);
			self.draw_warnings(f);
		}

		Ok(())
	}
}

impl Component for CommitComponent {
	fn commands(
		&self,
		out: &mut Vec<CommandInfo>,
		force_all: bool,
	) -> CommandBlocking {
		self.input.commands(out, force_all);

		if self.is_visible() || force_all {
			out.push(CommandInfo::new(
				strings::commands::commit_enter(&self.key_config),
				self.can_commit(),
				true,
			));

			out.push(CommandInfo::new(
				strings::commands::commit_amend(&self.key_config),
				self.can_amend(),
				true,
			));

			out.push(CommandInfo::new(
				strings::commands::commit_open_editor(
					&self.key_config,
				),
				true,
				true,
			));
		}

		visibility_blocking(self)
	}

	fn event(&mut self, ev: Event) -> Result<EventState> {
		if self.is_visible() {
			if self.input.event(ev)?.is_consumed() {
				return Ok(EventState::Consumed);
			}

			if let Event::Key(e) = ev {
				if e == self.key_config.enter && self.can_commit() {
					try_or_popup!(
						self,
						"commit error:",
						self.commit()
					);
				} else if e == self.key_config.commit_amend
					&& self.can_amend()
				{
					self.amend()?;
				} else if e == self.key_config.open_commit_editor {
					self.queue.push(
						InternalEvent::OpenExternalEditor(None),
					);
					self.hide();
				} else {
				}
				// stop key event propagation
				return Ok(EventState::Consumed);
			}
		}

		Ok(EventState::NotConsumed)
	}

	fn is_visible(&self) -> bool {
		self.input.is_visible()
	}

	fn hide(&mut self) {
		self.input.hide();
	}

	fn show(&mut self) -> Result<()> {
		//only clear text if it was not a normal commit dlg before, so to preserve old commit msg that was edited
		if !matches!(self.mode, Mode::Normal) {
			self.input.clear();
		}

		self.mode = Mode::Normal;

		self.mode = if sync::repo_state(CWD)? == RepoState::Merge {
			let ids = sync::mergehead_ids(CWD)?;
			self.input.set_title(strings::commit_title_merge());
			self.input.set_text(sync::merge_msg(CWD)?);
			Mode::Merge(ids)
		} else {
			self.commit_template =
				get_config_string(CWD, "commit.template")
					.ok()
					.flatten()
					.and_then(|path| read_to_string(path).ok());

			if self.is_empty() {
				if let Some(s) = &self.commit_template {
					self.input.set_text(s.clone());
				}
			}

			self.input.set_title(strings::commit_title());
			Mode::Normal
		};

		self.input.show()?;

		Ok(())
	}
}
