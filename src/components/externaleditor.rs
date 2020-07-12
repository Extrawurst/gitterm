use crate::{
    components::{
        visibility_blocking, CommandBlocking, CommandInfo, Component,
        DrawableComponent,
    },
    strings,
    ui::{self, style::SharedTheme},
};
use anyhow::{anyhow, Result};
use asyncgit::{sync::utils::repo_work_dir, CWD};
use crossterm::{
    event::Event,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use scopeguard::defer;
use std::{env, io, path::Path, process::Command};
use tui::{
    backend::Backend,
    layout::Rect,
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Text},
    Frame,
};

///
pub struct ExternalEditorComponent {
    visible: bool,
    theme: SharedTheme,
}

impl ExternalEditorComponent {
    ///
    pub fn new(theme: SharedTheme) -> Self {
        Self {
            visible: false,
            theme,
        }
    }

    /// opens file at given `path` in an available editor
    pub fn open_file_in_editor(path: &Path) -> Result<()> {
        let work_dir = repo_work_dir(CWD)?;

        let path = if path.is_relative() {
            Path::new(&work_dir).join(path)
        } else {
            path.into()
        };

        if !path.exists() {
            return Err(anyhow!("file not found: {:?}", path));
        }

        io::stdout().execute(LeaveAlternateScreen)?;
        defer! {
            io::stdout().execute(EnterAlternateScreen).expect("reset terminal");
        }

        let mut editor = env::var("GIT_EDITOR")
            .ok()
            .or_else(|| env::var("VISUAL").ok())
            .or_else(|| env::var("EDITOR").ok())
            .unwrap_or_else(|| String::from("vi"));

        //TODO: check the path.to_str result and return err on None because
        //otherwise this will pretty likely fail in the command stage otherwise
        //and https://github.com/extrawurst/gitui/issues/184 showed how weird
        //'vi' handles opening not existing files
        editor.push_str(&format!(" {}", path.to_string_lossy()));

        let mut editor = editor.split_whitespace();

        let command = editor.next().ok_or_else(|| {
            anyhow!("unable to read editor command")
        })?;

        Command::new(command)
            .current_dir(work_dir)
            .args(editor)
            .status()
            .map_err(|e| anyhow!("\"{}\": {}", command, e))?;

        Ok(())
    }
}

impl DrawableComponent for ExternalEditorComponent {
    fn draw<B: Backend>(
        &self,
        f: &mut Frame<B>,
        _rect: Rect,
    ) -> Result<()> {
        if self.visible {
            let txt =
                vec![Text::Raw(strings::MSG_OPENING_EDITOR.into())];

            let area = ui::centered_rect_absolute(25, 3, f.size());
            f.render_widget(Clear, area);
            f.render_widget(
                Paragraph::new(txt.iter())
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Thick)
                            .title_style(self.theme.title(true))
                            .border_style(self.theme.block(true)),
                    )
                    .style(self.theme.text_danger()),
                area,
            );
        }

        Ok(())
    }
}

impl Component for ExternalEditorComponent {
    fn commands(
        &self,
        out: &mut Vec<CommandInfo>,
        _force_all: bool,
    ) -> CommandBlocking {
        if self.visible {
            out.clear();
        }

        visibility_blocking(self)
    }

    fn event(&mut self, _ev: Event) -> Result<bool> {
        if self.visible {
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
