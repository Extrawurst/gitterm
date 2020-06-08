use crate::{
    components::{
        visibility_blocking, CommandBlocking, CommandInfo,
        CommitDetailsComponent, CommitList, Component,
        DrawableComponent,
    },
    keys,
    queue::{InternalEvent, Queue},
    strings,
    ui::style::Theme,
};
use anyhow::Result;
use asyncgit::{sync, AsyncLog, AsyncNotification, FetchStatus, CWD};
use crossbeam_channel::Sender;
use crossterm::event::Event;
use strings::commands;
use sync::CommitId;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

const SLICE_SIZE: usize = 1200;

///
pub struct Revlog {
    commit_details: CommitDetailsComponent,
    list: CommitList,
    git_log: AsyncLog,
    queue: Queue,
    visible: bool,
}

impl Revlog {
    ///
    pub fn new(
        queue: &Queue,
        sender: &Sender<AsyncNotification>,
        theme: &Theme,
    ) -> Self {
        Self {
            queue: queue.clone(),
            commit_details: CommitDetailsComponent::new(
                sender, theme,
            ),
            list: CommitList::new(strings::LOG_TITLE, theme),
            git_log: AsyncLog::new(sender),
            visible: false,
        }
    }

    ///
    pub fn any_work_pending(&self) -> bool {
        self.git_log.is_pending()
            || self.commit_details.any_work_pending()
    }

    ///
    pub fn update(&mut self) -> Result<()> {
        if self.visible {
            let log_changed =
                self.git_log.fetch()? == FetchStatus::Started;

            self.list.set_count_total(self.git_log.count()?);

            let selection = self.list.selection();
            let selection_max = self.list.selection_max();
            if self.list.items().needs_data(selection, selection_max)
                || log_changed
            {
                self.fetch_commits()?;
            }

            if !self.list.has_tags() || log_changed {
                self.list.set_tags(sync::get_tags(CWD)?);
            }

            if self.commit_details.is_visible() {
                self.commit_details.set_commit(
                    self.selected_commit(),
                    self.list.tags().expect("tags"),
                )?;
            }
        }

        Ok(())
    }

    ///
    pub fn update_git(
        &mut self,
        ev: AsyncNotification,
    ) -> Result<()> {
        if self.visible {
            match ev {
                AsyncNotification::CommitFiles
                | AsyncNotification::Log => self.update()?,
                _ => (),
            }
        }

        Ok(())
    }

    fn fetch_commits(&mut self) -> Result<()> {
        let want_min =
            self.list.selection().saturating_sub(SLICE_SIZE / 2);

        let commits = sync::get_commits_info(
            CWD,
            &self.git_log.get_slice(want_min, SLICE_SIZE)?,
            self.list.current_size().0.into(),
        );

        if let Ok(commits) = commits {
            self.list.items().set_items(want_min, commits);
        }

        Ok(())
    }

    fn selected_commit(&self) -> Option<CommitId> {
        self.list.selected_entry().map(|e| e.id)
    }
}

impl DrawableComponent for Revlog {
    fn draw<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        area: Rect,
    ) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(60),
                    Constraint::Percentage(40),
                ]
                .as_ref(),
            )
            .split(area);

        if self.commit_details.is_visible() {
            self.list.draw(f, chunks[0])?;
            self.commit_details.draw(f, chunks[1])?;
        } else {
            self.list.draw(f, area)?;
        }

        Ok(())
    }
}

impl Component for Revlog {
    fn event(&mut self, ev: Event) -> Result<bool> {
        if self.visible {
            let event_used = self.list.event(ev)?;

            if event_used {
                self.update()?;
                return Ok(true);
            } else if let Event::Key(keys::LOG_COMMIT_DETAILS) = ev {
                self.commit_details.toggle_visible()?;
                self.update()?;
                return Ok(true);
            } else if let Event::Key(keys::FOCUS_RIGHT) = ev {
                return if let Some(id) = self.selected_commit() {
                    self.queue
                        .borrow_mut()
                        .push_back(InternalEvent::InspectCommit(id));
                    Ok(true)
                } else {
                    Ok(false)
                };
            }
        }

        Ok(false)
    }

    fn commands(
        &self,
        out: &mut Vec<CommandInfo>,
        force_all: bool,
    ) -> CommandBlocking {
        if self.visible || force_all {
            self.list.commands(out, force_all);
        }

        out.push(CommandInfo::new(
            commands::LOG_DETAILS_TOGGLE,
            true,
            self.visible,
        ));

        out.push(CommandInfo::new(
            commands::LOG_DETAILS_OPEN,
            true,
            (self.visible && self.commit_details.is_visible())
                || force_all,
        ));

        visibility_blocking(self)
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn hide(&mut self) {
        self.visible = false;
        self.git_log.set_background();
    }

    fn show(&mut self) -> Result<()> {
        self.visible = true;
        self.list.clear();
        self.update()?;

        Ok(())
    }
}
