use crate::component::Component;
use crate::event::Event;
use crate::utils::create_id;
use web_view::WebView;

pub struct Panel {
    id: String,
    title: Option<String>,
    collapsible: bool,
    content: Option<Box<dyn Component>>,
}

impl Panel {
    pub fn new() -> Self {
        Panel {
            id: create_id(),
            title: None,
            collapsible: false,
            content: None,
        }
    }

    pub fn set_content(&mut self, content: impl Component + 'static) {
        self.content = Some(Box::new(content))
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
    fn render(&mut self) -> String {
        let content = if self.content.is_some() {
            self.content.as_mut().unwrap().render()
        } else {
            "".to_string()
        };

        format!("<div style=\"overflow: auto;\" id=\"{id}\" class=\"h-100 w-100\" {title} data-collapsible=\"{collapsible}\" data-role=\"panel\">{content}</div>",
                id=self.id,
                content=content ,
                title=optional_attribute("data-title-caption", &self.title),
                collapsible=self.collapsible)
    }

    fn handle_event(&mut self, _webview: &mut WebView<()>, _event: &Event) {}

    fn id(&self) -> &str {
        &*self.id
    }
}
