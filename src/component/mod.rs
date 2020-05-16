use crate::event::Event;
use web_view::WebView;

pub mod button;
pub mod layout;
pub mod menu;
pub mod panel;
pub mod text;
pub mod tree;

pub trait Component {
    fn render(&mut self) -> String;
    fn handle_event(&mut self, webview: &mut WebView<()>, event: &Event);
    fn id(&self) -> &str;
}
