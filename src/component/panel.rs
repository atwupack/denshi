use crate::component::Component;
use crate::event::Event;
use crate::utils::create_id;
use web_view::WebView;

pub enum ScrollMode {
    Auto,
    Always,
}

impl ScrollMode {
    fn style(&self) -> &str {
        match self {
            ScrollMode::Auto => "auto",
            ScrollMode::Always => "scroll",
        }
    }
}

pub struct Panel {
    id: String,
    title: Option<String>,
    collapsible: bool,
    content: Box<dyn Component>,
    scroll_mode: ScrollMode,
}

impl Panel {
    pub fn new(content: impl Component + 'static) -> Self {
        Panel {
            id: create_id(),
            title: None,
            collapsible: false,
            content: Box::new(content),
            scroll_mode: ScrollMode::Auto,
        }
    }

    pub fn set_scroll_mode(&mut self, mode: ScrollMode) {
        self.scroll_mode = mode
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
        format!(
            r#"<div style="overflow: {scroll_mode};" id="{id}" class="h-100 w-100" {title} data-collapsible="{collapsible}" data-role="panel">{content}</div>"#,
            id = self.id,
            content = self.content.render(),
            title = optional_attribute("data-title-caption", &self.title),
            collapsible = self.collapsible,
            scroll_mode = self.scroll_mode.style()
        )
    }

    fn handle_event(&mut self, webview: &mut WebView<()>, event: &Event) {
        self.content.handle_event(webview, event)
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}
