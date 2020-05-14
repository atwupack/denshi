use crate::component::Component;
use crate::event::Event;
use crate::utils::create_id;

pub struct TextArea {
    id: String,
}

impl TextArea {
    pub fn new() -> Self {
        TextArea { id: create_id() }
    }
}

impl Component for TextArea {
    fn render(&self) -> String {
        format!(
            r#"<textarea id="{id}" class="w-100 h-100" data-on-change="fire_value_changed('{id}')" data-on-textarea-create="fire_created" data-role="textarea" ></textarea>"#,
            id = self.id
        )
    }

    fn handle_event(&mut self, event: &Event) {
        if event.id == self.id {
            dbg!(event);
        }
    }

    fn id(&self) -> &str {
        &*self.id
    }
}

pub struct TextField {
    id: String,
    label: String,
}

impl TextField {
    pub fn new(label: impl Into<String>) -> Self {
        TextField {
            id: create_id(),
            label: label.into(),
        }
    }
}

impl Component for TextField {
    fn render(&self) -> String {
        format!(
            r#"<input id="{id}" oninput="fire_value_changed('{id}')" data-on-clear-click="fire_value_changed('{id}')" type="text" data-role="input" data-prepend="{label}"/>"#,
            id = self.id,
            label = self.label
        )
    }

    fn handle_event(&mut self, event: &Event) {
        if event.id == self.id {
            dbg!(event);
        }
    }

    fn id(&self) -> &str {
        &*self.id
    }
}
