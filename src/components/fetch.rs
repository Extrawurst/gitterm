use crate::{
    components::{
        cred::CredComponent, visibility_blocking, CommandBlocking,
        CommandInfo, Component, DrawableComponent,
    },
    keys::SharedKeyConfig,
    queue::{InternalEvent, Queue},
    strings,
    ui::{self, style::SharedTheme},
};
use anyhow::Result;
use asyncgit::{
    sync::{
        self,
        cred::{
            extract_username_password, need_username_password,
            BasicAuthCredential,
        },
        get_default_remote,
    },
    AsyncFetch, AsyncNotification, FetchRequest, PushProgress, CWD,
};
use crossbeam_channel::Sender;
use crossterm::event::Event;
use tui::{
    backend::Backend,
    layout::Rect,
    text::Span,
    widgets::{Block, BorderType, Borders, Clear, Gauge},
    Frame,
};

///
pub struct FetchComponent {
    visible: bool,
    git_fetch: AsyncFetch,
    progress: Option<PushProgress>,
    pending: bool,
    branch: String,
    queue: Queue,
    theme: SharedTheme,
    key_config: SharedKeyConfig,
    input_cred: CredComponent,
}

impl FetchComponent {
    ///
    pub fn new(
        queue: &Queue,
        sender: &Sender<AsyncNotification>,
        theme: SharedTheme,
        key_config: SharedKeyConfig,
    ) -> Self {
        Self {
            queue: queue.clone(),
            pending: false,
            visible: false,
            branch: String::new(),
            git_fetch: AsyncFetch::new(sender),
            progress: None,
            input_cred: CredComponent::new(
                theme.clone(),
                key_config.clone(),
            ),
            theme,
            key_config,
        }
    }

    ///
    pub fn fetch(&mut self, branch: String) -> Result<()> {
        self.branch = branch;
        self.show()?;
        if need_username_password()? {
            let cred =
                extract_username_password().unwrap_or_else(|_| {
                    BasicAuthCredential::new(None, None)
                });
            if cred.is_complete() {
                self.fetch_from_remote(Some(cred))
            } else {
                self.input_cred.set_cred(cred);
                self.input_cred.show()
            }
        } else {
            self.fetch_from_remote(None)
        }
    }

    fn fetch_from_remote(
        &mut self,
        cred: Option<BasicAuthCredential>,
    ) -> Result<()> {
        self.pending = true;
        self.progress = None;
        self.git_fetch.request(FetchRequest {
            remote: get_default_remote(CWD)?,
            branch: self.branch.clone(),
            basic_credential: cred,
        })?;
        Ok(())
    }

    ///
    pub fn update_git(
        &mut self,
        ev: AsyncNotification,
    ) -> Result<()> {
        if self.is_visible() {
            if let AsyncNotification::Fetch = ev {
                self.update()?;
            }
        }

        Ok(())
    }

    ///
    fn update(&mut self) -> Result<()> {
        self.pending = self.git_fetch.is_pending()?;
        // self.progress = self.git_fetch.progress()?;

        if !self.pending {
            if let Some((_bytes, err)) =
                self.git_fetch.last_result()?
            {
                if err.is_empty() {
                    let merge_res =
                        sync::branch_merge_upstream_fastforward(
                            CWD,
                            &self.branch,
                        );
                    if let Err(err) = merge_res {
                        self.queue.borrow_mut().push_back(
                            InternalEvent::ShowErrorMsg(format!(
                                "merge failed:\n{}",
                                err
                            )),
                        );
                    }
                } else {
                    self.queue.borrow_mut().push_back(
                        InternalEvent::ShowErrorMsg(format!(
                            "fetch failed:\n{}",
                            err
                        )),
                    );
                }
            }
            self.hide();
        }

        Ok(())
    }

    fn get_progress(&self) -> (String, u8) {
        self.progress.as_ref().map_or(
            (strings::PUSH_POPUP_PROGRESS_NONE.into(), 0),
            |progress| (String::from("Fetching"), progress.progress),
        )
    }
}

impl DrawableComponent for FetchComponent {
    fn draw<B: Backend>(
        &self,
        f: &mut Frame<B>,
        rect: Rect,
    ) -> Result<()> {
        if self.visible {
            let (state, progress) = self.get_progress();

            let area = ui::centered_rect_absolute(30, 3, f.size());

            f.render_widget(Clear, area);
            f.render_widget(
                Gauge::default()
                    .label(state.as_str())
                    .block(
                        Block::default()
                            .title(Span::styled(
                                strings::FETCH_POPUP_MSG,
                                self.theme.title(true),
                            ))
                            .borders(Borders::ALL)
                            .border_type(BorderType::Thick)
                            .border_style(self.theme.block(true)),
                    )
                    .gauge_style(self.theme.push_gauge())
                    .percent(u16::from(progress)),
                area,
            );
            self.input_cred.draw(f, rect)?;
        }

        Ok(())
    }
}

impl Component for FetchComponent {
    fn commands(
        &self,
        out: &mut Vec<CommandInfo>,
        force_all: bool,
    ) -> CommandBlocking {
        if self.is_visible() {
            out.clear();
        }

        if self.input_cred.is_visible() {
            self.input_cred.commands(out, force_all)
        } else {
            out.push(CommandInfo::new(
                strings::commands::close_msg(&self.key_config),
                !self.pending,
                self.visible,
            ));
            visibility_blocking(self)
        }
    }

    fn event(&mut self, ev: Event) -> Result<bool> {
        if self.visible {
            if let Event::Key(e) = ev {
                if e == self.key_config.exit_popup {
                    self.hide();
                }
                if self.input_cred.event(ev)? {
                    return Ok(true);
                } else if e == self.key_config.enter {
                    if self.input_cred.is_visible()
                        && self.input_cred.get_cred().is_complete()
                    {
                        self.fetch_from_remote(Some(
                            self.input_cred.get_cred().clone(),
                        ))?;
                        self.input_cred.hide();
                    } else {
                        self.hide();
                    }
                }
            }
            return Ok(true);
        }
        Ok(false)
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn hide(&mut self) {
        self.visible = false
    }

    fn show(&mut self) -> Result<()> {
        self.visible = true;

        Ok(())
    }
}
