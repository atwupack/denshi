use crate::component::Component;
use crate::event::{Event, EventValue};
use crate::utils::create_id;
use web_view::WebView;
use log::debug;
use std::rc::Rc;
use std::cell::RefCell;
use crate::frp::Behavior;
use enclose::enclose;

/// A text area containing multi line text.
#[derive(Clone)]
pub struct TextArea {
    id: String,
    state: Rc<RefCell<TextAreaState>>,
}

struct TextAreaState {
    text: String,
}

impl TextArea {
    pub fn new() -> Self {
        TextArea {
            id: create_id(),
            state: Rc::new(RefCell::new(TextAreaState {
                text: "".into(),
            })),
        }
    }

    pub fn text(&self) -> Behavior<String> {
        Behavior::from_fn(enclose!((self => sclone) move || {
            sclone.state.borrow().text.clone()
        }))
    }

}

impl Component for TextArea {
    fn render(&mut self) -> String {
        format!(
            r#"<textarea id="{id}" class="w-100 h-100" data-default-value="{text}" data-on-change="fire_value_changed('{id}')" data-on-textarea-create="fire_created" data-role="textarea" >{text}</textarea>"#,
            id = self.id,
            text = self.state.borrow().text,
        )
    }

    fn handle_event(&mut self, _webview: &mut WebView<()>, event: &Event) {
        if event.id == self.id {
            debug!(target: "textarea", "Received event: {:?}", event);
            match &event.value {
                EventValue::ValueChanged(new_value) => self.state.borrow_mut().text=new_value.clone(),
                _ => (),
            }
        }
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}

#[derive(Clone)]
pub enum TextType {
    /// Simple text field.
    Text,
    /// Text field for password input.
    Password { allow_reveal: bool },
    /// Text field for emails.
    Email,
}

impl TextType {
    fn to_attribute(&self) -> String {
        match self {
            TextType::Text => "type=\"text\"".into(),
            TextType::Password { allow_reveal} => format!("type=\"password\" data-reveal-button=\"{reveal}\"", reveal = allow_reveal),
            TextType::Email => "type=\"email\"".into(),
        }
    }
}

/// A single line text field.
#[derive(Clone)]
pub struct TextField {
    id: String,
    text_type: TextType,
    state: Rc<RefCell<TextFiledState>>,
}

struct TextFiledState {
    label: String,
    text: String,
}

impl TextField {
    pub fn new(label: impl Into<String>) -> Self {
        TextField {
            id: create_id(),
            state: Rc::new(RefCell::new(TextFiledState {
                label: label.into(),
                text: "".into(),
            })),
            text_type: TextType::Text,
        }
    }

    pub fn new_with_type(label: impl Into<String>, text_type: TextType) -> Self {
        TextField {
            id: create_id(),
            state: Rc::new(RefCell::new(TextFiledState {
                label: label.into(),
                text: "".into(),
            })),
            text_type,
        }
    }

    pub fn text(&self) -> Behavior<String> {
        Behavior::from_fn(enclose!((self => sclone) move || {
            sclone.state.borrow().text.clone()
        }))
    }
}

impl Component for TextField {
    fn render(&mut self) -> String {
        format!(
            r#"<input id="{id}" value="{value}" oninput="fire_value_changed('{id}')" data-on-clear-click="fire_value_changed('{id}')" {type_attr} data-role="input" data-prepend="{label}"/>"#,
            id = self.id,
            label = self.state.borrow().label,
            value = self.state.borrow().text,
            type_attr = self.text_type.to_attribute(),
        )
    }

    fn handle_event(&mut self, _webview: &mut WebView<()>, event: &Event) {
        if event.id == self.id {
            debug!(target: "textfield", "Received event: {:?}", event);
            match &event.value {
                EventValue::ValueChanged(new_value) => self.state.borrow_mut().text = new_value.clone() ,
                _ => (),
            }
        }
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}
