use uuid::Uuid;
use crate::component::Component;
use crate::event::Event;
use crate::component::menu::MenuItem::Entry;

enum MenuItem {
    Entry(String),
}

pub struct MenuBar {
    id: String,
    entries: Vec<MenuItem>,
}

impl MenuBar {
    pub fn new() -> Self {
        MenuBar {
            id: format!("id{id}",id=Uuid::new_v4()),
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, label: String) {
        self.entries.push(Entry(label));
    }

    fn render_items(&self) -> String {
        let mut items = String::new();
        for entry in &self.entries {
            match entry {
                Entry(label) => items.push_str(format!("<li><a href=\"#\">{label}</a></li>", label = label).as_str()),
            }
        }
        items
    }
}

impl Component for MenuBar {
    fn render(&self) -> String {
        format!(r#"<ul class="h-menu">{items}</ul>"#, items=self.render_items())
    }

    fn handle_event(&mut self, event: &Event) {

    }

    fn id(&self) -> &str {
        &*self.id
    }
}

pub struct Menu {

}