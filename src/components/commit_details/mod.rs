mod details;

use super::{
    command_pump, event_pump, CommandBlocking, CommandInfo,
    Component, DrawableComponent, FileTreeComponent,
};
use crate::{
    accessors, keys, queue::Queue, strings, ui::style::SharedTheme,
};
use anyhow::Result;
use asyncgit::{
    sync::{CommitId, CommitTags},
    AsyncCommitFiles, AsyncNotification,
};
use crossbeam_channel::Sender;
use crossterm::event::Event;
use details::DetailsComponent;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

pub struct CommitDetailsComponent {
    details: DetailsComponent,
    file_tree: FileTreeComponent,
    git_commit_files: AsyncCommitFiles,
    visible: bool,
}

impl CommitDetailsComponent {
    accessors!(self, [details, file_tree]);

    ///
    pub fn new(
        queue: &Queue,
        sender: &Sender<AsyncNotification>,
        theme: SharedTheme,
    ) -> Self {
        Self {
            details: DetailsComponent::new(theme.clone(), false),
            git_commit_files: AsyncCommitFiles::new(sender),
            file_tree: FileTreeComponent::new(
                "",
                false,
                Some(queue.clone()),
                theme,
            ),
            visible: false,
        }
    }

    fn get_files_title(&self) -> String {
        let files_count = self.file_tree.file_count();

        format!(
            "{} {}",
            strings::commit::DETAILS_FILES_TITLE,
            files_count
        )
    }

    ///
    pub fn set_commit(
        &mut self,
        id: Option<CommitId>,
        tags: Option<CommitTags>,
    ) -> Result<()> {
        self.details.set_commit(id, tags)?;

        if let Some(id) = id {
            if let Some((fetched_id, res)) =
                self.git_commit_files.current()?
            {
                if fetched_id == id {
                    self.file_tree.update(res.as_slice())?;
                    self.file_tree.set_title(self.get_files_title());

                    return Ok(());
                }
            }

            self.file_tree.clear()?;
            self.git_commit_files.fetch(id)?;
        }

        self.file_tree.set_title(self.get_files_title());

        Ok(())
    }

    ///
    pub fn any_work_pending(&self) -> bool {
        self.git_commit_files.is_pending()
    }

    ///
    pub const fn files(&self) -> &FileTreeComponent {
        &self.file_tree
    }
}

impl DrawableComponent for CommitDetailsComponent {
    fn draw<B: Backend>(
        &self,
        f: &mut Frame<B>,
        rect: Rect,
    ) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(60),
                    Constraint::Percentage(40),
                ]
                .as_ref(),
            )
            .split(rect);

        self.details.draw(f, chunks[0])?;
        self.file_tree.draw(f, chunks[1])?;

        Ok(())
    }
}

impl Component for CommitDetailsComponent {
    fn commands(
        &self,
        out: &mut Vec<CommandInfo>,
        force_all: bool,
    ) -> CommandBlocking {
        if self.visible || force_all {
            command_pump(
                out,
                force_all,
                self.components().as_slice(),
            );
        }

        CommandBlocking::PassingOn
    }

    fn event(&mut self, ev: Event) -> Result<bool> {
        if event_pump(ev, self.components_mut().as_mut_slice())? {
            return Ok(true);
        }

        if self.focused() {
            if let Event::Key(e) = ev {
                return match e {
                    keys::FOCUS_BELOW if (self.details.focused()) => {
                        self.details.focus(false);
                        self.file_tree.focus(true);

                        return Ok(true);
                    }
                    keys::FOCUS_ABOVE
                        if (self.file_tree.focused()) =>
                    {
                        self.file_tree.focus(false);
                        self.details.focus(true);

                        return Ok(true);
                    }
                    _ => Ok(false),
                };
            }
        }

        Ok(false)
    }

    fn is_visible(&self) -> bool {
        self.visible
    }
    fn hide(&mut self) {
        self.visible = false;
    }
    fn show(&mut self) -> Result<()> {
        self.visible = true;
        Ok(())
    }

    fn focused(&self) -> bool {
        self.details.focused() || self.file_tree.focused()
    }
    fn focus(&mut self, focus: bool) {
        self.details.focus(false);
        self.file_tree.focus(focus);
        self.file_tree.show_selection(true);
    }
}
