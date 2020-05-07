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
            id: format!("id{id}",id=Uuid::new_v4()),
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
        format!(r#"<a id="{id}" class="button" onclick="fire_clicked('{id}')">{label}</a>"#, id=self.id, label=self.label)
    }

    fn handle_event(&mut self, event: &Event) {
        if event.id == self.id {
            match &self.click_event {
                Some(listener) => listener(),
                None => ()
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
    checked: bool,
}

impl Checkbox {
    pub fn new(label: String) -> Self {
        Checkbox {
            id: format!("id{id}",id=Uuid::new_v4()),
            label,
            checked: false,
        }
    }
}

impl Component for Checkbox {
    fn render(&self) -> String {
        format!(r#"<input id="{id}" type="checkbox" data-role="checkbox" data-caption="{label}" />"#, id=self.id, label=self.label)
    }

    fn handle_event(&mut self, event: &Event) {
        if event.id == self.id {
            dbg!("Clicked");
        }
    }

    fn id(&self) -> &str {
        &*self.id
    }
}