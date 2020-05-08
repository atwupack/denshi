use crate::event::Event;

pub mod button;
pub mod text;
pub mod layout;
pub mod menu;
pub mod panel;

pub trait Component {
    fn render(&self) -> String;
    fn handle_event(&mut self, event: &Event);
    fn id(&self) -> &str;
}
