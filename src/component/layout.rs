use crate::component::{Component, ComponentManager};
use crate::event::Event;
use crate::utils::create_id;
use web_view::WebView;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub enum Orientation {
    VERTICAL,
    HORIZONTAL,
}

/// A split pane with a horizontal or vertical divider.
#[derive(Clone)]
pub struct Splitter {
    id: String,
    orientation: Orientation,
    state: Rc<RefCell<SplitterState>>,
}

#[derive(Eq, PartialEq, Hash, Clone)]
enum SplitterPosition {
    First,
    Second,
}

struct SplitterState {
    comps: ComponentManager<SplitterPosition>,
    gutter_size: u8,
}

impl Splitter {
    pub fn new(
        orientation: Orientation,
        first: impl Component + 'static,
        second: impl Component + 'static,
    ) -> Splitter {
        let mut comps = ComponentManager::new();
        comps.set_component(SplitterPosition::First, first);
        comps.set_component(SplitterPosition::Second, second);
        Splitter {
            id: create_id(),
            orientation,
            state: Rc::new(RefCell::new(SplitterState {
                gutter_size: 4,
                comps,
            })),
        }
    }

    pub fn set_gutter_size(&mut self, new_size: u8) {
        self.state.borrow_mut().gutter_size = new_size
    }
}

impl Component for Splitter {
    fn render(&mut self) -> String {
        let split_mode = match self.orientation {
            Orientation::HORIZONTAL => "data-split-mode=\"horizontal\"",
            Orientation::VERTICAL => "data-split-mode=\"vertical\"",
        };

        let mut state = self.state.borrow_mut();

        format!(
            r#"<div id="{id}" data-gutter-size="{gutter}" data-role="splitter" class="h-100" {split_mode}>
                      <div>{first}</div>
                      <div>{second}</div>
                   </div>"#,
            id = self.id,
            first = state.comps.render_component(&SplitterPosition::First),
            second = state.comps.render_component(&SplitterPosition::Second),
            split_mode = split_mode,
            gutter = state.gutter_size,
        )
    }

    fn handle_event(&mut self, webview: &mut WebView<()>, event: &Event) {
        self.state.borrow_mut().comps.notify_all_components(webview, event);
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}

/// Simple page layout with a header, content and a footer.
pub struct Page {
    id: String,
    state: Rc<RefCell<PageState>>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum PagePosition {
    Header,
    Footer,
    Content,
}

struct PageState {
    comps: ComponentManager<PagePosition>,
}

impl Page {
    pub fn new() -> Self {
        Page {
            id: create_id(),
            state: Rc::new(RefCell::new(PageState{
                comps: ComponentManager::new(),
            })),
        }
    }

    pub fn set_content(&mut self, content: impl Component + 'static) {
        self.state.borrow_mut().comps.set_component(PagePosition::Content, content);
    }

    pub fn set_header(&mut self, header: impl Component + 'static) {
        self.state.borrow_mut().comps.set_component(PagePosition::Header, header);
    }

    pub fn set_footer(&mut self, footer: impl Component + 'static) {
        self.state.borrow_mut().comps.set_component(PagePosition::Footer, footer);
    }
}

impl Component for Page {
    fn render(&mut self) -> String {
        let mut components = String::new();
        components.push_str(
            "<div class=\"noselect h-100 container-fluid d-flex flex-column flex-align-stretch\">",
        );

        let mut state = self.state.borrow_mut();

        components.push_str(&state.comps.render_component_with(&PagePosition::Header, |comp_str| {
            format!("<header>{header}</header>", header = comp_str)
        }));
        components.push_str(&state.comps.render_component_with(&PagePosition::Content, |comp_str| {
            format!("<div class=\"h-100\">{content}</div>", content = comp_str)
        }));
        components.push_str(&state.comps.render_component_with(&PagePosition::Footer, |comp_str| {
            format!("<footer>{footer}</footer>", footer = comp_str)
        }));

        components.push_str("</div>");
        components
    }

    fn handle_event(&mut self, webview: &mut WebView<()>, event: &Event) {

        let mut state = self.state.borrow_mut();

        state.comps.notify_all_components(webview, event);
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}

/// A form with several lines of input fields.
#[derive(Clone)]
pub struct Form {
    id: String,
    state: Rc<RefCell<FormState>>,
}

struct FormState {
    comps: ComponentManager<()>,
}

impl Form {
    pub fn new() -> Self {
        Form {
            id: create_id(),
            state: Rc::new(RefCell::new(FormState {
                comps: ComponentManager::new(),
            })),
        }
    }

    pub fn add_line(&mut self, component: impl Component + 'static) {
        self.state.borrow_mut().comps.add_component((), component)
    }

    fn render_lines(&mut self) -> String {
        let mut lines = String::new();
        let mut state = self.state.borrow_mut();

        lines.push_str(&state.comps.render_component_with(&(), |comp_str| {
            format!(
                "<div class=\"form-group\">{line}</div>",
                line = comp_str
            )
        }));
        lines
    }
}

impl Component for Form {
    fn render(&mut self) -> String {
        let lines = self.render_lines();
        format!(
            "<form id=\"{id}\">{lines}</form>",
            id = self.id(),
            lines = lines,
        )
    }

    fn handle_event(&mut self, webview: &mut WebView<()>, event: &Event) {
        self.state.borrow_mut().comps.notify_all_components(webview, event);
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}

#[derive(Clone)]
/// A tab pane with several tab to switch between.
pub struct TabPane {
    id: String,
    tabs: Rc<RefCell<Vec<Tab>>>,
}

struct Tab {
    label: String,
    content: Box<dyn Component>,
}

impl TabPane {
    pub fn new() -> Self {
        TabPane {
            id: create_id(),
            tabs: Default::default(),
        }
    }

    pub fn add_tab(&mut self, label: impl Into<String>, content: impl Component + 'static) {
        self.tabs.borrow_mut().push(Tab {
            label: label.into(),
            content: Box::new(content),
        });
    }

    fn render_tab_headers(&self) -> String {
        let mut tabs = String::new();
        for tab in &*self.tabs.borrow() {
            tabs.push_str(
                format!(
                    "<li><a href=\"#{id}tab\">{label}</a></li>",
                    id = tab.content.id(),
                    label = tab.label
                )
                .as_str(),
            );
        }
        tabs
    }

    fn render_tab_content(&mut self) -> String {
        let mut tabs = String::new();
        tabs.push_str("<div class=\"border bd-default no-border-top p-2 w-100 h-100\">");
        for tab in &mut *self.tabs.borrow_mut() {
            tabs.push_str(
                format!(
                    "<div class=\"w-100 h-100\" id=\"{id}tab\">",
                    id = tab.content.id()
                )
                .as_str(),
            );
            tabs.push_str(tab.content.render().as_str());
            tabs.push_str("</div>");
        }
        tabs.push_str("</div>");
        tabs
    }
}

impl Component for TabPane {
    fn render(&mut self) -> String {
        format!(
            r#"<ul data-role="tabs" data-expand="true">{tabs}</ul>{content}"#,
            tabs = self.render_tab_headers(),
            content = self.render_tab_content()
        )
    }

    fn handle_event(&mut self, webview: &mut WebView<()>, event: &Event) {
        for tab in &mut *self.tabs.borrow_mut() {
            tab.content.handle_event(webview, event);
        }
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}
