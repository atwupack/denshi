use crate::component::Component;
use crate::event::Event;
use crate::utils::create_id;

pub struct BorderLayout {
    id: String,
    top: Option<Box<dyn Component>>,
    bottom: Option<Box<dyn Component>>,
    center: Option<Box<dyn Component>>,
    left: Option<Box<dyn Component>>,
    right: Option<Box<dyn Component>>,
}

impl BorderLayout {
    pub fn new() -> Self {
        BorderLayout {
            id: create_id(),
            top: None,
            bottom: None,
            center: None,
            left: None,
            right: None,
        }
    }
}

impl Component for BorderLayout {
    fn render(&self) -> String {
        unimplemented!()
    }

    fn handle_event(&mut self, _event: &Event) {
        unimplemented!()
    }

    fn id(&self) -> &str {
        &*self.id
    }
}

pub enum Orientation {
    VERTICAL,
    HORIZONTAL,
}

pub struct Splitter {
    id: String,
    orientation: Orientation,
    gutter_size: u8,
    first: Box<dyn Component>,
    second: Box<dyn Component>,
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
            gutter_size: 4,
            first: Box::new(first),
            second: Box::new(second),
        }
    }

    pub fn set_gutter_size(&mut self, new_size: u8) {
        self.gutter_size = new_size
    }
}

impl Component for Splitter {
    fn render(&self) -> String {
        let split_mode = match self.orientation {
            Orientation::HORIZONTAL => "data-split-mode=\"horizontal\"",
            Orientation::VERTICAL => "data-split-mode=\"vertical\"",
        };

        format!(
            r#"<div id="{id}" data-gutter-size="{gutter}" data-role="splitter" class="h-100" {split_mode}>
                      <div class="d-flex flex-justify-start flex-align-start">{first}</div>
                      <div class="d-flex flex-column flex-justify-start flex-align-start">{second}</div>
                   </div>"#,
            id = self.id,
            first = self.first.render(),
            second = self.second.render(),
            split_mode = split_mode,
            gutter = self.gutter_size
        )
    }

    fn handle_event(&mut self, event: &Event) {
        self.first.handle_event(event);
        self.second.handle_event(event);
    }

    fn id(&self) -> &str {
        &*self.id
    }
}

pub struct Page {
    id: String,
    header: Option<Box<dyn Component>>,
    content: Option<Box<dyn Component>>,
    footer: Option<Box<dyn Component>>,
}

impl Page {
    pub fn new() -> Self {
        Page {
            id: create_id(),
            header: None,
            content: None,
            footer: None,
        }
    }

    pub fn set_content(&mut self, content: impl Component + 'static) {
        self.content = Some(Box::new(content))
    }

    pub fn set_header(&mut self, header: impl Component + 'static) {
        self.header = Some(Box::new(header))
    }

    pub fn set_footer(&mut self, footer: impl Component + 'static) {
        self.header = Some(Box::new(footer))
    }
}

impl Component for Page {
    fn render(&self) -> String {
        let mut components = String::new();
        components.push_str(
            "<div class=\"noselect h-100 container-fluid d-flex flex-column flex-align-stretch\">",
        );

        if self.header.is_some() {
            components.push_str(&format!(
                "<header>{header}</header>",
                header = self.header.as_ref().unwrap().render()
            ));
        }

        if self.content.is_some() {
            components.push_str(&format!(
                "<div class=\"h-100\">{content}</div>",
                content = self.content.as_ref().unwrap().render()
            ));
        }

        if self.footer.is_some() {
            components.push_str(&format!(
                "<footer>{footer}</footer>",
                footer = self.footer.as_ref().unwrap().render()
            ));
        }

        components.push_str("</div>");
        components
    }

    fn handle_event(&mut self, event: &Event) {
        if self.header.is_some() {
            self.header.as_mut().unwrap().handle_event(event);
        }
        if self.content.is_some() {
            self.content.as_mut().unwrap().handle_event(event);
        }
        if self.footer.is_some() {
            self.footer.as_mut().unwrap().handle_event(event);
        }
    }

    fn id(&self) -> &str {
        &*self.id
    }
}

pub struct Form {
    id: String,
    components: Vec<Box<dyn Component>>,
}

impl Form {
    pub fn new() -> Self {
        Form {
            id: create_id(),
            components: Vec::new(),
        }
    }

    pub fn add_line(&mut self, component: impl Component + 'static) {
        self.components.push(Box::new(component));
    }

    fn render_lines(&self) -> String {
        let mut lines = String::new();
        for comp in &self.components {
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
    fn render(&self) -> String {
        format!(
            "<form id=\"{id}\">{lines}</form>",
            id = self.id,
            lines = self.render_lines()
        )
    }

    fn handle_event(&mut self, event: &Event) {
        for comp in &mut self.components {
            comp.handle_event(event);
        }
    }

    fn id(&self) -> &str {
        &*self.id
    }
}

pub struct TabPane {
    id: String,
    tabs: Vec<Tab>,
}

struct Tab {
    label: String,
    content: Box<dyn Component>,
}

impl TabPane {
    pub fn new() -> Self {
        TabPane {
            id: create_id(),
            tabs: Vec::new(),
        }
    }

    pub fn add_tab(&mut self, label: impl Into<String>, content: impl Component + 'static) {
        self.tabs.push(Tab {
            label: label.into(),
            content: Box::new(content),
        });
    }

    fn render_tab_headers(&self) -> String {
        let mut tabs = String::new();
        for tab in &self.tabs {
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

    fn render_tab_content(&self) -> String {
        let mut tabs = String::new();
        tabs.push_str("<div class=\"border bd-default no-border-top p-2 w-100 h-100\">");
        for tab in &self.tabs {
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
    fn render(&self) -> String {
        format!(
            r#"<ul data-role="tabs" data-expand="true">{tabs}</ul>{content}"#,
            tabs = self.render_tab_headers(),
            content = self.render_tab_content()
        )
    }

    fn handle_event(&mut self, event: &Event) {
        for tab in &mut self.tabs {
            tab.content.handle_event(event);
        }
    }

    fn id(&self) -> &str {
        &*self.id
    }
}
