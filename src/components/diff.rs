use super::{CommandBlocking, DrawableComponent, ScrollType};
use crate::{
    components::{CommandInfo, Component},
    keys,
    queue::{InternalEvent, Queue},
    strings,
    ui::{calc_scroll_top, style::Theme},
};
use asyncgit::{hash, DiffLine, DiffLineType, FileDiff};
use crossterm::event::Event;
use std::{borrow::Cow, cmp};
use strings::commands;
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    symbols,
    widgets::{Block, Borders, Paragraph, Text},
    Frame,
};

use anyhow::Result;

#[derive(Default)]
struct Current {
    path: String,
    is_stage: bool,
    hash: u64,
}

///
pub struct DiffComponent {
    diff: FileDiff,
    selection: usize,
    selected_hunk: Option<usize>,
    current_size: (u16, u16),
    focused: bool,
    current: Current,
    scroll_top: usize,
    queue: Option<Queue>,
    theme: Theme,
}

impl DiffComponent {
    ///
    pub fn new(queue: Option<Queue>, theme: &Theme) -> Self {
        Self {
            focused: false,
            queue,
            current: Current::default(),
            selected_hunk: None,
            diff: FileDiff::default(),
            current_size: (0, 0),
            selection: 0,
            scroll_top: 0,
            theme: *theme,
        }
    }
    ///
    const fn can_scroll(&self) -> bool {
        self.diff.lines > 1
    }
    ///
    pub fn current(&self) -> (String, bool) {
        (self.current.path.clone(), self.current.is_stage)
    }
    ///
    pub fn clear(&mut self) -> Result<()> {
        self.current = Current::default();
        self.diff = FileDiff::default();
        self.scroll_top = 0;
        self.selection = 0;
        self.selected_hunk = None;

        Ok(())
    }
    ///
    pub fn update(
        &mut self,
        path: String,
        is_stage: bool,
        diff: FileDiff,
    ) -> Result<()> {
        let hash = hash(&diff);

        if self.current.hash != hash {
            self.current = Current {
                path,
                is_stage,
                hash,
            };
            self.diff = diff;
            self.scroll_top = 0;
            self.selection = 0;

            self.selected_hunk =
                Self::find_selected_hunk(&self.diff, self.selection)?;
        }

        Ok(())
    }

    fn move_selection(
        &mut self,
        move_type: ScrollType,
    ) -> Result<()> {
        let old = self.selection;

        let max = self.diff.lines.saturating_sub(1) as usize;

        self.selection = match move_type {
            ScrollType::Down => old.saturating_add(1),
            ScrollType::Up => old.saturating_sub(1),
            ScrollType::Home => 0,
            ScrollType::End => max,
            ScrollType::PageDown => self.selection.saturating_add(
                self.current_size.1.saturating_sub(1) as usize,
            ),
            ScrollType::PageUp => self.selection.saturating_sub(
                self.current_size.1.saturating_sub(1) as usize,
            ),
        };

        self.selection = cmp::min(max, self.selection);

        if old != self.selection {
            self.selected_hunk =
                Self::find_selected_hunk(&self.diff, self.selection)?;
        }

        Ok(())
    }

    fn find_selected_hunk(
        diff: &FileDiff,
        line_selected: usize,
    ) -> Result<Option<usize>> {
        let mut line_cursor = 0_usize;
        for (i, hunk) in diff.hunks.iter().enumerate() {
            let hunk_len = hunk.lines.len();
            let hunk_min = line_cursor;
            let hunk_max = line_cursor + hunk_len;

            let hunk_selected =
                hunk_min <= line_selected && hunk_max > line_selected;

            if hunk_selected {
                return Ok(Some(i));
            }

            line_cursor += hunk_len;
        }

        Ok(None)
    }

    fn get_text(&self, width: u16, height: u16) -> Result<Vec<Text>> {
        let selection = self.selection;

        let min = self.scroll_top;
        let max = min + height as usize;

        let mut res = Vec::new();
        let mut line_cursor = 0_usize;
        let mut lines_added = 0_usize;

        for (i, hunk) in self.diff.hunks.iter().enumerate() {
            let hunk_selected =
                self.selected_hunk.map_or(false, |s| s == i);

            if lines_added >= height as usize {
                break;
            }

            let hunk_len = hunk.lines.len();
            let hunk_min = line_cursor;
            let hunk_max = line_cursor + hunk_len;

            if Self::hunk_visible(hunk_min, hunk_max, min, max) {
                for (i, line) in hunk.lines.iter().enumerate() {
                    if line_cursor >= min && line_cursor <= max {
                        Self::add_line(
                            &mut res,
                            width,
                            line,
                            selection == line_cursor,
                            hunk_selected,
                            i == hunk_len as usize - 1,
                            self.theme,
                        );
                        lines_added += 1;
                    }

                    line_cursor += 1;
                }
            } else {
                line_cursor += hunk_len;
            }
        }

        Ok(res)
    }

    fn add_line(
        text: &mut Vec<Text>,
        width: u16,
        line: &DiffLine,
        selected: bool,
        selected_hunk: bool,
        end_of_hunk: bool,
        theme: Theme,
    ) {
        {
            let style = theme.diff_hunk_marker(selected_hunk);

            if end_of_hunk {
                text.push(Text::Styled(
                    Cow::from(symbols::line::BOTTOM_LEFT),
                    style,
                ));
            } else {
                text.push(match line.line_type {
                    DiffLineType::Header => Text::Styled(
                        Cow::from(symbols::line::TOP_LEFT),
                        style,
                    ),
                    _ => Text::Styled(
                        Cow::from(symbols::line::VERTICAL),
                        style,
                    ),
                });
            }
        }

        let trimmed =
            line.content.trim_matches(|c| c == '\n' || c == '\r');

        let filled = if selected {
            // selected line
            format!("{:w$}\n", trimmed, w = width as usize)
        } else {
            // weird eof missing eol line
            format!("{}\n", trimmed)
        };
        //TODO: allow customize tabsize
        let content = Cow::from(filled.replace("\t", "  "));

        text.push(Text::Styled(
            content,
            theme.diff_line(line.line_type, selected),
        ));
    }

    fn hunk_visible(
        hunk_min: usize,
        hunk_max: usize,
        min: usize,
        max: usize,
    ) -> bool {
        // full overlap
        if hunk_min <= min && hunk_max >= max {
            return true;
        }

        // partly overlap
        if (hunk_min >= min && hunk_min <= max)
            || (hunk_max >= min && hunk_max <= max)
        {
            return true;
        }

        false
    }

    fn add_hunk(&self) -> Result<()> {
        if let Some(hunk) = self.selected_hunk {
            let hash = self.diff.hunks[hunk].header_hash;
            self.queue
                .as_ref()
                .expect("try using queue in immutable diff")
                .borrow_mut()
                .push_back(InternalEvent::AddHunk(hash));
        }

        Ok(())
    }

    fn is_immutable(&self) -> bool {
        self.queue.is_none()
    }
}

impl DrawableComponent for DiffComponent {
    fn draw<B: Backend>(
        &mut self,
        f: &mut Frame<B>,
        r: Rect,
    ) -> Result<()> {
        self.current_size =
            (r.width.saturating_sub(2), r.height.saturating_sub(2));

        self.scroll_top = calc_scroll_top(
            self.scroll_top,
            self.current_size.1 as usize,
            self.selection,
        );

        let title =
            format!("{}{}", strings::TITLE_DIFF, self.current.path);
        f.render_widget(
            Paragraph::new(
                self.get_text(r.width, self.current_size.1)?.iter(),
            )
            .block(
                Block::default()
                    .title(title.as_str())
                    .borders(Borders::ALL)
                    .border_style(self.theme.block(self.focused))
                    .title_style(self.theme.title(self.focused)),
            )
            .alignment(Alignment::Left),
            r,
        );

        Ok(())
    }
}

impl Component for DiffComponent {
    fn commands(
        &self,
        out: &mut Vec<CommandInfo>,
        _force_all: bool,
    ) -> CommandBlocking {
        out.push(CommandInfo::new(
            commands::SCROLL,
            self.can_scroll(),
            self.focused,
        ));

        out.push(
            CommandInfo::new(
                commands::DIFF_HOME_END,
                self.can_scroll(),
                self.focused,
            )
            .hidden(),
        );

        if !self.is_immutable() {
            out.push(CommandInfo::new(
                commands::DIFF_HUNK_REMOVE,
                self.selected_hunk.is_some(),
                self.focused && self.current.is_stage,
            ));
            out.push(CommandInfo::new(
                commands::DIFF_HUNK_ADD,
                self.selected_hunk.is_some(),
                self.focused && !self.current.is_stage,
            ));
        }

        CommandBlocking::PassingOn
    }

    fn event(&mut self, ev: Event) -> Result<bool> {
        if self.focused {
            if let Event::Key(e) = ev {
                return match e {
                    keys::MOVE_DOWN => {
                        self.move_selection(ScrollType::Down)?;
                        Ok(true)
                    }
                    keys::SHIFT_DOWN | keys::END => {
                        self.move_selection(ScrollType::End)?;
                        Ok(true)
                    }
                    keys::HOME | keys::SHIFT_UP => {
                        self.move_selection(ScrollType::Home)?;
                        Ok(true)
                    }
                    keys::MOVE_UP => {
                        self.move_selection(ScrollType::Up)?;
                        Ok(true)
                    }
                    keys::PAGE_UP => {
                        self.move_selection(ScrollType::PageUp)?;
                        Ok(true)
                    }
                    keys::PAGE_DOWN => {
                        self.move_selection(ScrollType::PageDown)?;
                        Ok(true)
                    }
                    keys::ENTER if !self.is_immutable() => {
                        self.add_hunk()?;
                        Ok(true)
                    }
                    _ => Ok(false),
                };
            }
        }

        Ok(false)
    }

    fn focused(&self) -> bool {
        self.focused
    }
    fn focus(&mut self, focus: bool) {
        self.focused = focus
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lineendings() {
        let mut text = Vec::new();
        DiffComponent::add_line(
            &mut text,
            10,
            &DiffLine {
                content: String::from("line 1\r\n"),
                line_type: DiffLineType::None,
            },
            false,
            false,
            false,
            Theme::default(),
        );

        assert_eq!(text.len(), 2);

        if let Text::Styled(c, _) = &text[1] {
            assert_eq!(c, "line 1\n");
        } else {
            panic!("err")
        }
    }
}
