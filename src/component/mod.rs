use crate::event::Event;
use web_view::WebView;

pub mod button;
pub mod layout;
pub mod menu;
pub mod panel;
pub mod text;
pub mod tree;

/// Functions every component needs to provide.
pub trait Component {
    /// Render the component as HTML.
    fn render(&mut self) -> String;
    /// Callback for events.
    fn handle_event(&mut self, webview: &mut WebView<()>, event: &Event);
    /// The globally unique ID for teh component.
    fn id(&self) -> String;
}




