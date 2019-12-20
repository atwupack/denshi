use uuid::Uuid;
use crate::component::Component;
use crate::event::Event;

pub struct TextField {
    id: String,
    label: String,
}

impl TextField {
    pub fn new(label: String) -> Self {
        TextField {
            id: Uuid::new_v4().to_string(),
            label,
        }
    }
}

impl Component for TextField {
    fn render(&self) -> String {
        format!(r#"<input id="{id}" type="text" data-role="input" data-prepend="{label}"/>"#, id=self.id, label=self.label)
    }

    fn handle_event(&mut self, event: &Event) {
        // do nothing
    }
}