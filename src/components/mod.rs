mod changes;
mod command;
mod commit;
mod diff;
mod help;
mod msg;
mod reset;
mod utils;
pub use changes::ChangesComponent;
pub use command::{CommandInfo, CommandText};
pub use commit::CommitComponent;
pub use diff::DiffComponent;
pub use help::HelpComponent;
pub use msg::MsgComponent;
pub use reset::ResetComponent;
pub use utils::filetree::FileTreeItemKind;

use crossterm::event::Event;
use tui::{
    backend::Backend,
    layout::Alignment,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Text},
    Frame,
};

/// creates accessors for a list of components
///
/// allows generating code to make sure
/// we always enumerate all components in both getter functions
#[macro_export]
macro_rules! accessors {
    ($self:ident, [$($element:ident),+]) => {
        fn components(& $self) -> Vec<&dyn Component> {
            vec![
                $(&$self.$element,)+
            ]
        }

        fn components_mut(&mut $self) -> Vec<&mut dyn Component> {
            vec![
                $(&mut $self.$element,)+
            ]
        }
    };
}

pub fn event_pump(
    ev: Event,
    components: &mut [&mut dyn Component],
) -> bool {
    for c in components {
        if c.event(ev) {
            return true;
        }
    }

    false
}

#[derive(Copy, Clone)]
pub enum ScrollType {
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
}

///
#[derive(PartialEq)]
pub enum CommandBlocking {
    Blocking,
    PassingOn,
}

///
pub fn visibility_blocking<T: Component>(
    comp: &T,
) -> CommandBlocking {
    if comp.is_visible() {
        CommandBlocking::Blocking
    } else {
        CommandBlocking::PassingOn
    }
}

///
pub trait DrawableComponent {
    ///
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, rect: Rect);
}

/// base component trait
pub trait Component {
    ///
    fn commands(
        &self,
        out: &mut Vec<CommandInfo>,
        force_all: bool,
    ) -> CommandBlocking;

    /// returns true if event propagation needs to end (event was consumed)
    fn event(&mut self, ev: Event) -> bool;

    ///
    fn focused(&self) -> bool {
        false
    }
    /// focus/unfocus this component depending on param
    fn focus(&mut self, _focus: bool) {}
    ///
    fn is_visible(&self) -> bool {
        true
    }
    ///
    fn hide(&mut self) {}
    ///
    fn show(&mut self) {}
}

fn dialog_paragraph<'a, 't, T>(
    title: &'a str,
    content: T,
) -> Paragraph<'a, 't, T>
where
    T: Iterator<Item = &'t Text<'t>>,
{
    Paragraph::new(content)
        .block(Block::default().title(title).borders(Borders::ALL))
        .alignment(Alignment::Left)
}
