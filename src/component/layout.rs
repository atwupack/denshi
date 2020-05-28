use crate::component::Component;
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

struct SplitterState {
    first: Box<dyn Component>,
    second: Box<dyn Component>,
    gutter_size: u8,
}

impl Splitter {
    pub fn new(
        orientation: Orientation,
        first: impl Component + 'static,
        second: impl Component + 'static,
    ) -> Splitter {
        Splitter {
            id: create_id(),
            orientation,
            state: Rc::new(RefCell::new(SplitterState {
                gutter_size: 4,
                first: Box::new(first),
                second: Box::new(second),

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
            first = state.first.render(),
            second = state.second.render(),
            split_mode = split_mode,
            gutter = state.gutter_size,
        )
    }

    fn handle_event(&mut self, webview: &mut WebView<()>, event: &Event) {
        self.state.borrow_mut().first.handle_event(webview, event);
        self.state.borrow_mut().second.handle_event(webview, event);
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

struct PageState {
    header: Option<Box<dyn Component>>,
    content: Option<Box<dyn Component>>,
    footer: Option<Box<dyn Component>>,

}

impl Page {
    pub fn new() -> Self {
        Page {
            id: create_id(),
            state: Rc::new(RefCell::new(PageState{
                header: None,
                content: None,
                footer: None,
            })),
        }
    }

    pub fn set_content(&mut self, content: impl Component + 'static) {
        self.state.borrow_mut().content = Some(Box::new(content))
    }

    pub fn set_header(&mut self, header: impl Component + 'static) {
        self.state.borrow_mut().header = Some(Box::new(header))
    }

    pub fn set_footer(&mut self, footer: impl Component + 'static) {
        self.state.borrow_mut().footer = Some(Box::new(footer))
    }
}

impl Component for Page {
    fn render(&mut self) -> String {
        let mut components = String::new();
        components.push_str(
            "<div class=\"noselect h-100 container-fluid d-flex flex-column flex-align-stretch\">",
        );

        let mut state = self.state.borrow_mut();

        if state.header.is_some() {
            components.push_str(&format!(
                "<header>{header}</header>",
                header = state.header.as_mut().unwrap().render()
            ));
        }

        if state.content.is_some() {
            components.push_str(&format!(
                "<div class=\"h-100\">{content}</div>",
                content = state.content.as_mut().unwrap().render()
            ));
        }

        if state.footer.is_some() {
            components.push_str(&format!(
                "<footer>{footer}</footer>",
                footer = state.footer.as_mut().unwrap().render()
            ));
        }

        components.push_str("</div>");
        components
    }

    fn handle_event(&mut self, webview: &mut WebView<()>, event: &Event) {

        let mut state = self.state.borrow_mut();

        if state.header.is_some() {
            state.header.as_mut().unwrap().handle_event(webview, event);
        }
        if state.content.is_some() {
            state.content.as_mut().unwrap().handle_event(webview, event);
        }
        if state.footer.is_some() {
            state.footer.as_mut().unwrap().handle_event(webview, event);
        }
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
    components: Vec<Box<dyn Component>>,
}

impl Form {
    pub fn new() -> Self {
        Form {
            id: create_id(),
            state: Rc::new(RefCell::new(FormState {
                components: Vec::new(),
            })),
        }
    }

    pub fn add_line(&mut self, component: impl Component + 'static) {
        self.state.borrow_mut().components.push(Box::new(component));
    }

    fn render_lines(&mut self) -> String {
        let mut lines = String::new();
        let mut state = self.state.borrow_mut();

        for comp in &mut state.components {
            lines.push_str(
                format!(
                    "<div class=\"form-group\">{line}</div>",
                    line = comp.render()
                )
                .as_str(),
            );
        }
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
        for comp in &mut self.state.borrow_mut().components {
            comp.handle_event(webview, event);
        }
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
            tabs: Rc::new(RefCell::new(Vec::new())),
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
