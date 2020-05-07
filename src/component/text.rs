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
            id: format!("id{id}",id=Uuid::new_v4()),
            label,
        }
    }
}

impl Component for TextField {
    fn render(&self) -> String {
        format!(r#"<input id="{id}" oninput="fire_value_changed('{id}')" data-on-clear-click="fire_value_changed('{id}')" type="text" data-role="input" data-prepend="{label}"/>"#, id=self.id, label=self.label)
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