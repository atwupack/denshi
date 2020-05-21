use crate::component::Component;
use crate::event::Event;
use crate::utils::create_id;
use log::warn;
use web_view::WebView;

/// Standard button to be pressed
pub struct Button {
    id: String,
    label: String,
    click_event: Option<Box<dyn Fn()>>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Button {
        Button {
            id: create_id(),
            label: label.into(),
            click_event: None,
        }
    }

    pub fn set_click_event(&mut self, event: impl Fn() + 'static) {
        self.click_event = Some(Box::new(event));
    }
}

impl Component for Button {
    fn render(&mut self) -> String {
        format!(
            r#"<a id="{id}" class="button" onclick="fire_clicked('{id}')">{label}</a>"#,
            id = self.id,
            label = self.label
        )
    }

    fn handle_event(&mut self, _webview: &mut WebView<()>, event: &Event) {
        if event.id == self.id {
            if let Some(listener) = &self.click_event {
                listener();
            } else {
                warn!(target: "button" , "No listener for button with ID {}", self.id);
            }
        }
    }

    fn id(&self) -> &str {
        &*self.id
    }
}

pub struct Checkbox {
    id: String,
    label: String,
}

impl Checkbox {
    pub fn new(label: impl Into<String>) -> Self {
        Checkbox {
            id: create_id(),
            label: label.into(),
        }
    }
}

impl Component for Checkbox {
    fn render(&mut self) -> String {
        format!(
            r#"<input id="{id}" type="checkbox" data-role="checkbox" data-on-checkbox-create="fire_created" data-caption="{label}">"#,
            id = self.id,
            label = self.label
        )
    }

    fn handle_event(&mut self, _webview: &mut WebView<()>, event: &Event) {
        if event.id == self.id {}
    }

    fn id(&self) -> &str {
        &*self.id
    }
}
