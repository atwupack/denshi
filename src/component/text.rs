use crate::component::Component;
use crate::event::{Event, EventValue};
use crate::utils::create_id;
use web_view::WebView;
use log::debug;

pub struct TextArea {
    id: String,
    text: String,
}

impl TextArea {
    pub fn new() -> Self {
        TextArea {
            id: create_id(),
            text: "".into(),
        }
    }
}

impl Component for TextArea {
    fn render(&mut self) -> String {
        format!(
            r#"<textarea id="{id}" class="w-100 h-100" data-default-value="{text}" data-on-change="fire_value_changed('{id}')" data-on-textarea-create="fire_created" data-role="textarea" >{text}</textarea>"#,
            id = self.id,
            text = self.text,
        )
    }

    fn handle_event(&mut self, _webview: &mut WebView<()>, event: &Event) {
        if event.id == self.id {
            debug!(target: "textarea", "Received event: {:?}", event);
            match &event.value {
                EventValue::ValueChanged(new_value) => self.text=new_value.clone(),
                _ => (),
            }
        }
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}

pub enum TextType {
    /// Simple text field.
    Text,
    /// Text field for password input.
    Password,
    /// Text field for emails.
    Email,
}

pub struct TextField {
    id: String,
    label: String,
    text: String,
    text_type: TextType,
}

impl TextField {
    pub fn new(label: impl Into<String>) -> Self {
        TextField {
            id: create_id(),
            label: label.into(),
            text: "".into(),
            text_type: TextType::Text,
        }
    }

    pub fn new_with_type(label: impl Into<String>, text_type: TextType) -> Self {
        TextField {
            id: create_id(),
            label: label.into(),
            text: "".into(),
            text_type,
        }
    }
}

impl Component for TextField {
    fn render(&mut self) -> String {
        format!(
            r#"<input id="{id}" value="{value}" oninput="fire_value_changed('{id}')" data-on-clear-click="fire_value_changed('{id}')" type="text" data-role="input" data-prepend="{label}"/>"#,
            id = self.id,
            label = self.label,
            value = self.text,
        )
    }

    fn handle_event(&mut self, _webview: &mut WebView<()>, event: &Event) {
        if event.id == self.id {
            debug!(target: "textfield", "Received event: {:?}", event);
            match &event.value {
                EventValue::ValueChanged(new_value) => self.text = new_value.clone() ,
                _ => (),
            }
        }
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}
