use crate::component::menu::MenuItem::Entry;
use crate::component::Component;
use crate::event::Event;
use crate::utils::create_id;
use web_view::WebView;

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
            id: create_id(),
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, label: impl Into<String>) {
        self.entries.push(Entry(label.into()));
    }

    fn render_items(&self) -> String {
        let mut items = String::new();
        for entry in &self.entries {
            match entry {
                Entry(label) => items.push_str(
                    format!("<li><a href=\"#\">{label}</a></li>", label = label).as_str(),
                ),
            }
        }
        items
    }
}

impl Component for MenuBar {
    fn render(&mut self) -> String {
        format!(
            r#"<ul class="h-menu">{items}</ul>"#,
            items = self.render_items()
        )
    }

    fn handle_event(&mut self, _webview: &mut WebView<()>, _event: &Event) {}

    fn id(&self) -> String {
        self.id.clone()
    }
}

pub struct Menu {}
