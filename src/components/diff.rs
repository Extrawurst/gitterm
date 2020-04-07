use super::{CommandBlocking, DrawableComponent, EventUpdate};
use crate::{
    components::{CommandInfo, Component},
    queue::{InternalEvent, Queue},
    strings,
};
use asyncgit::{hash, DiffLine, DiffLineType, FileDiff};
use crossterm::event::{Event, KeyCode};
use std::{borrow::Cow, cmp, convert::TryFrom};
use strings::commands;
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    symbols,
    widgets::{Block, Borders, Paragraph, Text, Widget},
    Frame,
};

#[derive(Default)]
struct Current {
    path: String,
    is_stage: bool,
    hash: u64,
}

///
pub struct DiffComponent {
    diff: FileDiff,
    scroll: u16,
    focused: bool,
    current: Current,
    selected_hunk: Option<u16>,
    queue: Queue,
}

impl DiffComponent {
    ///
    pub fn new(queue: Queue) -> Self {
        Self {
            focused: false,
            queue,
            current: Current::default(),
            selected_hunk: None,
            diff: FileDiff::default(),
            scroll: 0,
        }
    }
    ///
    fn can_scroll(&self) -> bool {
        self.diff.lines > 1
    }
    ///
    pub fn current(&self) -> (String, bool) {
        (self.current.path.clone(), self.current.is_stage)
    }
    ///
    pub fn clear(&mut self) {
        self.current = Current::default();
        self.diff = FileDiff::default();
        self.scroll = 0;

        self.selected_hunk =
            Self::find_selected_hunk(&self.diff, self.scroll);
    }
    ///
    pub fn update(
        &mut self,
        path: String,
        is_stage: bool,
        diff: FileDiff,
    ) {
        let hash = hash(&diff);

        if self.current.hash != hash {
            self.current = Current {
                path,
                is_stage,
                hash,
            };
            self.diff = diff;
            self.scroll = 0;

            self.selected_hunk =
                Self::find_selected_hunk(&self.diff, self.scroll);
        }
    }

    fn scroll(&mut self, inc: bool) {
        let old = self.scroll;
        if inc {
            self.scroll = cmp::min(
                self.diff.lines.saturating_sub(1),
                self.scroll.saturating_add(1),
            );
        } else {
            self.scroll = self.scroll.saturating_sub(1);
        }

        if old != self.scroll {
            self.selected_hunk =
                Self::find_selected_hunk(&self.diff, self.scroll);
        }
    }

    fn find_selected_hunk(
        diff: &FileDiff,
        line_selected: u16,
    ) -> Option<u16> {
        let mut line_cursor = 0_u16;
        for (i, hunk) in diff.hunks.iter().enumerate() {
            let hunk_len = u16::try_from(hunk.lines.len()).unwrap();
            let hunk_min = line_cursor;
            let hunk_max = line_cursor + hunk_len;

            let hunk_selected =
                hunk_min <= line_selected && hunk_max > line_selected;

            if hunk_selected {
                return Some(u16::try_from(i).unwrap());
            }

            line_cursor += hunk_len;
        }

        None
    }

    fn get_text(&self, width: u16, height: u16) -> Vec<Text> {
        let selection = self.scroll;
        let height_d2 = height / 2;
        let min = self.scroll.saturating_sub(height_d2);
        let max = min + height;

        let mut res = Vec::new();
        let mut line_cursor = 0_u16;
        let mut lines_added = 0_u16;

        for (i, hunk) in self.diff.hunks.iter().enumerate() {
            let hunk_selected = self
                .selected_hunk
                .map_or(false, |s| s == u16::try_from(i).unwrap());

            if lines_added >= height {
                break;
            }

            let hunk_len = u16::try_from(hunk.lines.len()).unwrap();
            let hunk_min = line_cursor;
            let hunk_max = line_cursor + hunk_len;

            if Self::hunk_visible(hunk_min, hunk_max, min, max) {
                for (i, line) in hunk.lines.iter().enumerate() {
                    if line_cursor >= min {
                        Self::add_line(
                            &mut res,
                            width,
                            line,
                            selection == line_cursor,
                            hunk_selected,
                            i == hunk_len as usize - 1,
                        );
                        lines_added += 1;
                    }

                    line_cursor += 1;
                }
            } else {
                line_cursor += hunk_len;
            }
        }
        res
    }

    fn add_line(
        text: &mut Vec<Text>,
        width: u16,
        line: &DiffLine,
        selected: bool,
        selected_hunk: bool,
        end_of_hunk: bool,
    ) {
        let select_color = Color::Rgb(0, 0, 100);
        let style_default = Style::default().bg(if selected {
            select_color
        } else {
            Color::Reset
        });

        {
            let style = Style::default()
                .bg(if selected || selected_hunk {
                    select_color
                } else {
                    Color::Reset
                })
                .fg(Color::DarkGray);

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

        let style_delete = Style::default()
            .fg(Color::Red)
            .bg(if selected { select_color } else { Color::Reset });
        let style_add = Style::default()
            .fg(Color::Green)
            .bg(if selected { select_color } else { Color::Reset });
        let style_header = Style::default()
            .fg(Color::Rgb(0, 0, 0))
            .bg(if selected {
                select_color
            } else {
                Color::DarkGray
            })
            .modifier(Modifier::BOLD);

        let filled = if selected {
            // selected line
            format!(
                "{:w$}\n",
                line.content.trim_matches('\n'),
                w = width as usize
            )
        } else if line.content.matches('\n').count() == 1 {
            // regular line, no selection (cheapest)
            line.content.clone()
        } else {
            // weird eof missing eol line
            format!("{}\n", line.content.trim_matches('\n'))
        };
        let content = Cow::from(filled);

        text.push(match line.line_type {
            DiffLineType::Delete => {
                Text::Styled(content, style_delete)
            }
            DiffLineType::Add => Text::Styled(content, style_add),
            DiffLineType::Header => {
                Text::Styled(content, style_header)
            }
            _ => Text::Styled(content, style_default),
        });
    }

    fn hunk_visible(
        hunk_min: u16,
        hunk_max: u16,
        min: u16,
        max: u16,
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

    fn add_hunk(&self) {
        if let Some(hunk) = self.selected_hunk {
            let hash = self.diff.hunks
                [usize::try_from(hunk).unwrap()]
            .header_hash;
            self.queue
                .borrow_mut()
                .push_back(InternalEvent::AddHunk(hash));
        }
    }
}

impl DrawableComponent for DiffComponent {
    fn draw<B: Backend>(&self, f: &mut Frame<B>, r: Rect) {
        let mut style_border = Style::default().fg(Color::DarkGray);
        let mut style_title = Style::default();
        if self.focused {
            style_border = style_border.fg(Color::Gray);
            style_title = style_title.modifier(Modifier::BOLD);
        }

        Paragraph::new(self.get_text(r.width, r.height).iter())
            .block(
                Block::default()
                    .title(strings::TITLE_DIFF)
                    .borders(Borders::ALL)
                    .border_style(style_border)
                    .title_style(style_title),
            )
            .alignment(Alignment::Left)
            .render(f, r);
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

        let cmd_text = if self.current.is_stage {
            commands::DIFF_HUNK_ADD
        } else {
            commands::DIFF_HUNK_REMOVE
        };

        out.push(CommandInfo::new(
            cmd_text,
            self.selected_hunk.is_some(),
            self.focused,
        ));

        CommandBlocking::PassingOn
    }

    fn event(&mut self, ev: Event) -> Option<EventUpdate> {
        if self.focused {
            if let Event::Key(e) = ev {
                return match e.code {
                    KeyCode::Down => {
                        self.scroll(true);
                        Some(EventUpdate::None)
                    }
                    KeyCode::Up => {
                        self.scroll(false);
                        Some(EventUpdate::None)
                    }
                    KeyCode::Enter => {
                        self.add_hunk();
                        Some(EventUpdate::None)
                    }
                    _ => None,
                };
            }
        }

        None
    }

    fn focused(&self) -> bool {
        self.focused
    }

    fn focus(&mut self, focus: bool) {
        self.focused = focus
    }
}
