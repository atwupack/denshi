use crate::component::Component;
use crate::event::Event;
use crate::utils::create_id;
use log::warn;
use web_view::WebView;
use std::rc::Rc;
use std::cell::RefCell;

/// Standard button to be pressed
#[derive(Clone)]
pub struct Button {
    id: String,
    state: Rc<RefCell<ButtonState>>,
    click_event: Option<Rc<RefCell<dyn Fn()>>>,
}

struct ButtonState {
    label: String,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Button {
        Button {
            id: create_id(),
            state: Rc::new(RefCell::new(ButtonState {
                label: label.into(),
            })) ,
            click_event: None,
        }
    }

    pub fn set_click_event(&mut self, event: impl Fn() + 'static) {
        self.click_event = Some(Rc::new(RefCell::new(event)));
    }
}

impl Component for Button {
    fn render(&mut self) -> String {
        format!(
            r#"<a id="{id}" class="button" onclick="fire_clicked('{id}')">{label}</a>"#,
            id = self.id,
            label = self.state.borrow().label,
        )
    }

    fn handle_event(&mut self, _webview: &mut WebView<()>, event: &Event) {
        if event.id == self.id {
            if let Some(listener) = &self.click_event {
                (listener.borrow())()
            } else {
                warn!(target: "button" , "No listener for button with ID {}", self.id);
            }
        }
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}

/// A check box with a label.
#[derive(Clone)]
pub struct Checkbox {
    id: String,
    state: Rc<RefCell<CheckboxState>>
}

struct CheckboxState {
    label: String,
}

impl Checkbox {
    pub fn new(label: impl Into<String>) -> Self {
        Checkbox {
            id: create_id(),
            state: Rc::new(RefCell::new(CheckboxState {
                label: label.into(),
            })),
        }
    }
}

impl Component for Checkbox {
    fn render(&mut self) -> String {
        format!(
            r#"<input id="{id}" type="checkbox" data-role="checkbox" data-on-checkbox-create="fire_created" data-caption="{label}">"#,
            id = self.id,
            label = self.state.borrow().label
        )
    }

    fn handle_event(&mut self, _webview: &mut WebView<()>, event: &Event) {
        if event.id == self.id {}
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}
