use crate::component::Component;
use uuid::Uuid;
use crate::event::Event;

pub struct Button {
    id: String,
    label: String,
}

impl Button {
    pub fn new(label: String) -> Button {
        Button {
            id: Uuid::new_v4().to_string(),
            label,
        }
    }
}

impl Component for Button {
    fn render(&self) -> String {
        format!(r#"<button id="{id}" class="button" onclick="fire_click('{id}')">{label}</button>"#, id=self.id, label=self.label)
    }

    fn handle_event(&mut self, event: &Event) {
        if event.id == self.id {
            dbg!(event);
        }
    }
}