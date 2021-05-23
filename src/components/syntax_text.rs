use super::{
    CommandBlocking, CommandInfo, Component, DrawableComponent,
    EventState,
};
use crate::{
    keys::SharedKeyConfig,
    ui::{
        self, style::SharedTheme, AsyncSyntaxJob, ParagraphState,
        ScrollPos, StatefulParagraph,
    },
};
use anyhow::Result;
use async_utils::AsyncSingleJob;
use asyncgit::{
    sync::{self, TreeFile},
    AsyncNotification, CWD,
};
use crossbeam_channel::Sender;
use crossterm::event::Event;
use itertools::Either;
use std::{cell::Cell, convert::From, path::Path};
use tui::{
    backend::Backend,
    layout::Rect,
    text::Text,
    widgets::{Block, Borders, Wrap},
    Frame,
};

pub struct SyntaxTextComponent {
    current_file: Option<(String, Either<ui::SyntaxText, String>)>,
    async_highlighting:
        AsyncSingleJob<AsyncSyntaxJob, AsyncNotification>,
    key_config: SharedKeyConfig,
    scroll_top: Cell<u16>,
    focused: bool,
    theme: SharedTheme,
}

impl SyntaxTextComponent {
    ///
    pub fn new(
        sender: &Sender<AsyncNotification>,
        key_config: SharedKeyConfig,
        theme: SharedTheme,
    ) -> Self {
        Self {
            async_highlighting: AsyncSingleJob::new(
                sender.clone(),
                AsyncNotification::SyntaxHighlighting,
            ),
            current_file: None,
            scroll_top: Cell::new(0),
            focused: false,
            key_config,
            theme,
        }
    }

    ///
    pub fn update(&mut self, ev: AsyncNotification) {
        if ev == AsyncNotification::SyntaxHighlighting {
            if let Some(job) = self.async_highlighting.get_last() {
                if let Some((path, content)) =
                    self.current_file.as_mut()
                {
                    if let Some(syntax) = (*job.text).clone() {
                        if syntax.path() == Path::new(path) {
                            *content = Either::Left(syntax);
                        }
                    }
                }
            }
        }
    }

    ///
    pub fn any_work_pending(&self) -> bool {
        self.async_highlighting.is_pending()
    }

    ///
    pub fn clear(&mut self) {
        self.current_file = None;
    }

    ///
    pub fn load_file(&mut self, path: String, item: &TreeFile) {
        let already_loaded = self
            .current_file
            .as_ref()
            .map(|(current_file, _)| current_file == &path)
            .unwrap_or_default();

        if !already_loaded {
            //TODO: fetch file content async aswell
            match sync::tree_file_content(CWD, item) {
                Ok(content) => {
                    self.async_highlighting.spawn(
                        AsyncSyntaxJob::new(
                            content.clone(),
                            path.clone(),
                        ),
                    );

                    self.current_file =
                        Some((path, Either::Right(content)))
                }
                Err(e) => {
                    self.current_file = Some((
                        path,
                        Either::Right(format!(
                            "error loading file: {}",
                            e
                        )),
                    ))
                }
            }
        }
    }
}

impl DrawableComponent for SyntaxTextComponent {
    fn draw<B: Backend>(
        &self,
        f: &mut Frame<B>,
        area: Rect,
    ) -> Result<()> {
        let text = self.current_file.as_ref().map_or_else(
            || Text::from(""),
            |(_, content)| match content {
                Either::Left(syn) => syn.into(),
                Either::Right(s) => Text::from(s.as_str()),
            },
        );

        let content = StatefulParagraph::new(text)
            .wrap(Wrap { trim: false })
            .block(
                Block::default()
                    .title(
                        self.current_file
                            .as_ref()
                            .map(|(name, _)| name.clone())
                            .unwrap_or_default(),
                    )
                    .borders(Borders::ALL)
                    .border_style(self.theme.title(self.focused())),
            );

        let mut state = ParagraphState::default();
        state.set_scroll(ScrollPos::new(0, self.scroll_top.get()));

        f.render_stateful_widget(content, area, &mut state);

        self.scroll_top.set(
            self.scroll_top
                .get()
                .min(state.lines().saturating_sub(area.height)),
        );

        Ok(())
    }
}

impl Component for SyntaxTextComponent {
    fn commands(
        &self,
        _out: &mut Vec<CommandInfo>,
        _force_all: bool,
    ) -> CommandBlocking {
        //TODO: scrolling
        CommandBlocking::PassingOn
    }

    fn event(
        &mut self,
        event: crossterm::event::Event,
    ) -> Result<EventState> {
        if let Event::Key(key) = event {
            if key == self.key_config.move_down {
                self.scroll_top
                    .set(self.scroll_top.get().saturating_add(1));
            } else if key == self.key_config.move_up {
                self.scroll_top
                    .set(self.scroll_top.get().saturating_sub(1));
            }
        }

        Ok(EventState::NotConsumed)
    }

    ///
    fn focused(&self) -> bool {
        self.focused
    }

    /// focus/unfocus this component depending on param
    fn focus(&mut self, focus: bool) {
        self.focused = focus
    }
}
