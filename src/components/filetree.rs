use super::{
    utils::{
        filetree::{FileTreeItem, FileTreeItemKind},
        statustree::{MoveSelection, StatusTree},
    },
    CommandBlocking, DrawableComponent,
};
use crate::{
    components::{CommandInfo, Component},
    keys::SharedKeyConfig,
    queue::{InternalEvent, NeedsUpdate, Queue},
    strings::{self, commands, order},
    ui,
    ui::style::SharedTheme,
};
use anyhow::Result;
use asyncgit::{hash, StatusItem, StatusItemType};
use crossterm::event::Event;
use std::{borrow::Cow, cell::Cell, convert::From, path::Path};
use tui::{backend::Backend, layout::Rect, widgets::Text, Frame};

///
pub struct FileTreeComponent {
    title: String,
    tree: StatusTree,
    pending: bool,
    current_hash: u64,
    focused: bool,
    show_selection: bool,
    queue: Option<Queue>,
    theme: SharedTheme,
    key_config: SharedKeyConfig,
    scroll_top: Cell<usize>,
}

impl FileTreeComponent {
    ///
    pub fn new(
        title: &str,
        focus: bool,
        queue: Option<Queue>,
        theme: SharedTheme,
        key_config: SharedKeyConfig,
    ) -> Self {
        Self {
            title: title.to_string(),
            tree: StatusTree::default(),
            current_hash: 0,
            focused: focus,
            show_selection: focus,
            queue,
            theme,
            key_config,
            scroll_top: Cell::new(0),
            pending: true,
        }
    }

    ///
    pub fn update(&mut self, list: &[StatusItem]) -> Result<()> {
        self.pending = false;
        let new_hash = hash(list);
        if self.current_hash != new_hash {
            self.tree.update(list)?;
            self.current_hash = new_hash;
        }

        Ok(())
    }

    ///
    pub fn selection(&self) -> Option<FileTreeItem> {
        self.tree.selected_item()
    }

    ///
    pub fn selection_file(&self) -> Option<StatusItem> {
        self.tree.selected_item().and_then(|f| {
            if let FileTreeItemKind::File(f) = f.kind {
                Some(f)
            } else {
                None
            }
        })
    }

    ///
    pub fn show_selection(&mut self, show: bool) {
        self.show_selection = show;
    }

    /// returns true if list is empty
    pub fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    ///
    pub const fn file_count(&self) -> usize {
        self.tree.tree.file_count()
    }

    ///
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    ///
    pub fn clear(&mut self) -> Result<()> {
        self.current_hash = 0;
        self.pending = true;
        self.tree.update(&[])
    }

    ///
    pub fn is_file_seleted(&self) -> bool {
        if let Some(item) = self.tree.selected_item() {
            match item.kind {
                FileTreeItemKind::File(_) => true,
                FileTreeItemKind::Path(..) => false,
            }
        } else {
            false
        }
    }

    fn move_selection(&mut self, dir: MoveSelection) -> bool {
        let changed = self.tree.move_selection(dir);

        if changed {
            if let Some(ref queue) = self.queue {
                queue.borrow_mut().push_back(InternalEvent::Update(
                    NeedsUpdate::DIFF,
                ));
            }
        }

        changed
    }

    fn item_to_text<'b>(
        item: &FileTreeItem,
        width: u16,
        selected: bool,
        theme: &'b SharedTheme,
    ) -> Option<Text<'b>> {
        let indent_str = if item.info.indent == 0 {
            String::from("")
        } else {
            format!("{:w$}", " ", w = (item.info.indent as usize) * 2)
        };

        if !item.info.visible {
            return None;
        }

        match &item.kind {
            FileTreeItemKind::File(status_item) => {
                let status_char =
                    Self::item_status_char(status_item.status);
                let file = Path::new(&status_item.path)
                    .file_name()
                    .and_then(std::ffi::OsStr::to_str)
                    .expect("invalid path.");

                let txt = if selected {
                    format!(
                        "{} {}{:w$}",
                        status_char,
                        indent_str,
                        file,
                        w = width as usize
                    )
                } else {
                    format!("{} {}{}", status_char, indent_str, file)
                };

                Some(Text::Styled(
                    Cow::from(txt),
                    theme.item(status_item.status, selected),
                ))
            }

            FileTreeItemKind::Path(path_collapsed) => {
                let collapse_char =
                    if path_collapsed.0 { '▸' } else { '▾' };

                let txt = if selected {
                    format!(
                        "  {}{}{:w$}",
                        indent_str,
                        collapse_char,
                        item.info.path,
                        w = width as usize
                    )
                } else {
                    format!(
                        "  {}{}{}",
                        indent_str, collapse_char, item.info.path,
                    )
                };

                Some(Text::Styled(
                    Cow::from(txt),
                    theme.text(true, selected),
                ))
            }
        }
    }

    fn item_status_char(item_type: StatusItemType) -> char {
        match item_type {
            StatusItemType::Modified => 'M',
            StatusItemType::New => '+',
            StatusItemType::Deleted => '-',
            StatusItemType::Renamed => 'R',
            StatusItemType::Typechange => ' ',
        }
    }
}

impl DrawableComponent for FileTreeComponent {
    fn draw<B: Backend>(
        &self,
        f: &mut Frame<B>,
        r: Rect,
    ) -> Result<()> {
        if self.pending {
            let items = vec![Text::Styled(
                Cow::from(strings::LOADING_TEXT),
                self.theme.text(false, false),
            )];

            ui::draw_list(
                f,
                r,
                self.title.as_str(),
                items.into_iter(),
                None,
                self.focused,
                &self.theme,
            );
        } else {
            let selection_offset =
                self.tree.tree.items().iter().enumerate().fold(
                    0,
                    |acc, (idx, e)| {
                        let visible = e.info.visible;
                        let index_above_select =
                            idx < self.tree.selection.unwrap_or(0);

                        if !visible && index_above_select {
                            acc + 1
                        } else {
                            acc
                        }
                    },
                );

            let select = self
                .tree
                .selection
                .map(|idx| idx.saturating_sub(selection_offset))
                .unwrap_or_default();
            let tree_height = r.height.saturating_sub(2) as usize;

            self.scroll_top.set(ui::calc_scroll_top(
                self.scroll_top.get(),
                tree_height,
                select,
            ));

            let items = self
                .tree
                .tree
                .items()
                .iter()
                .enumerate()
                .filter_map(|(idx, e)| {
                    Self::item_to_text(
                        e,
                        r.width,
                        self.show_selection
                            && self
                                .tree
                                .selection
                                .map_or(false, |e| e == idx),
                        &self.theme,
                    )
                })
                .skip(self.scroll_top.get());

            ui::draw_list(
                f,
                r,
                self.title.as_str(),
                items,
                Some(select),
                self.focused,
                &self.theme,
            );
        }

        Ok(())
    }
}

impl Component for FileTreeComponent {
    fn commands(
        &self,
        out: &mut Vec<CommandInfo>,
        force_all: bool,
    ) -> CommandBlocking {
        out.push(
            CommandInfo::new(
                commands::NAVIGATE_TREE,
                !self.is_empty(),
                self.focused || force_all,
            )
            .order(order::NAV),
        );

        CommandBlocking::PassingOn
    }

    fn event(&mut self, ev: Event) -> Result<bool> {
        if self.focused {
            if let Event::Key(e) = ev {
                return if e == self.key_config.move_down {
                    Ok(self.move_selection(MoveSelection::Down))
                } else if e == self.key_config.move_up {
                    Ok(self.move_selection(MoveSelection::Up))
                } else if e == self.key_config.home
                    || e == self.key_config.shift_up
                {
                    Ok(self.move_selection(MoveSelection::Home))
                } else if e == self.key_config.end
                    || e == self.key_config.shift_down
                {
                    Ok(self.move_selection(MoveSelection::End))
                } else if e == self.key_config.move_left {
                    Ok(self.move_selection(MoveSelection::Left))
                } else if e == self.key_config.move_right {
                    Ok(self.move_selection(MoveSelection::Right))
                } else {
                    Ok(false)
                };
            }
        }

        Ok(false)
    }

    fn focused(&self) -> bool {
        self.focused
    }
    fn focus(&mut self, focus: bool) {
        self.focused = focus;
        self.show_selection(focus);
    }
}
