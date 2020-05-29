use crate::component::Component;
use crate::event::Event;
use crate::utils::create_id;
use web_view::WebView;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
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

/// A panel with a single content and optional title.
#[derive(Clone)]
pub struct Panel {
    id: String,
    collapsible: bool,
    scroll_mode: ScrollMode,
    state: Rc<RefCell<PanelState>>,
}

struct PanelState {
    title: Option<String>,
    content: Box<dyn Component>,
}

impl Panel {
    pub fn new(content: impl Component + 'static) -> Self {
        Panel {
            id: create_id(),
            state: Rc::new(RefCell::new(PanelState {
                title: None,
                content: Box::new(content),
            })),
            collapsible: false,
            scroll_mode: ScrollMode::Auto,
        }
    }

    pub fn set_scroll_mode(&mut self, mode: ScrollMode) {
        self.scroll_mode = mode
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.state.borrow_mut().title = Some(title.into())
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
        let mut state = self.state.borrow_mut();
        format!(
            r#"<div style="overflow: {scroll_mode};" id="{id}" class="h-100 w-100" {title} data-collapsible="{collapsible}" data-role="panel">{content}</div>"#,
            id = self.id,
            content = state.content.render(),
            title = optional_attribute("data-title-caption", &state.title),
            collapsible = self.collapsible,
            scroll_mode = self.scroll_mode.style()
        )
    }

    fn handle_event(&mut self, webview: &mut WebView<()>, event: &Event) {
        self.state.borrow_mut().content.handle_event(webview, event)
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}
