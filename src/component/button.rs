use crate::component::Component;
use uuid::Uuid;
use crate::event::Event;

pub struct Button {
    id: String,
    label: String,
    click_event: Option<Box<dyn Fn()>>,
}

impl Button {
    pub fn new(label: String) -> Button {
        Button {
            id: Uuid::new_v4().to_string(),
            label,
            click_event: None,
        }
    }

    pub fn set_click_event(&mut self, event: impl Fn() + 'static) {
        self.click_event = Some(Box::new(event));
    }
}

impl Component for Button {
    fn render(&self) -> String {
        format!(r#"<button id="{id}" class="button" onclick="fire_clicked('{id}')">{label}</button>"#, id=self.id, label=self.label)
    }

    fn handle_event(&mut self, event: &Event) {
        if event.id == self.id {
            match &self.click_event {
                Some(listener) => listener(),
                None => ()
            }
        }
    }
}