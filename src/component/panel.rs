use crate::component::Component;
use crate::event::Event;
use crate::utils::create_id;

pub struct Panel {
    id: String,
    components: Vec<Box<dyn Component>>,
    title: Option<String>,
    collapsible: bool,
}

impl Panel {
    pub fn new() -> Self {
        Panel {
            id: create_id(),
            components: Vec::new(),
            title: None,
            collapsible: false,
        }
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = Some(title.into())
    }

    pub fn set_collapsible(&mut self, collapsible: bool) {
        self.collapsible = collapsible
    }
}

fn optional_attribute(attribute: &str, value: &Option<String>) -> String {
    match value {
        Some(s) => format!("{attr}=\"{value}\"", attr = attribute, value = s),
        None => "".to_string(),
    }
}

impl Component for Panel {
    fn render(&self) -> String {
        format!("<div id=\"{id}\" class=\"h-100 w-100\" {title} data-collapsible=\"{collapsible}\" data-role=\"panel\">{content}</div>",
                id=self.id, content="",
                title=optional_attribute("data-title-caption", &self.title),
                collapsible=self.collapsible)
    }

    fn handle_event(&mut self, _event: &Event) {}

    fn id(&self) -> &str {
        &*self.id
    }
}
