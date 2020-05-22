use crate::event::Event;
use web_view::WebView;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};

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

/// Reference to a component
#[derive(Clone)]
pub struct CompRef(Rc<RefCell<dyn Component>>);

impl CompRef {
    pub fn new(comp: impl Component + 'static) -> Self {
        CompRef(Rc::new(RefCell::new(comp)))
    }
}

impl Component for CompRef {
    fn render(&mut self) -> String {
        let cell: &RefCell<dyn Component> = &self.0;
        let mut rm = cell.borrow_mut();
        let c = rm.deref_mut();
        c.render()
    }

    fn handle_event(&mut self, webview: &mut WebView<()>, event: &Event) {
        let cell: &RefCell<dyn Component> = &self.0;
        let mut rm = cell.borrow_mut();
        let c = rm.deref_mut();
        c.handle_event(webview, event)
    }

    fn id(&self) -> String {
        let cell: &RefCell<dyn Component> = &self.0;
        let rm = cell.borrow();
        let c = rm.deref();
        c.id().clone()
    }
}



