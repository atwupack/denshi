use crate::component::Component;
use crate::event::Event;
use uuid::Uuid;


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
            id: format!("id{id}",id=Uuid::new_v4()),
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

    fn handle_event(&mut self, event: &Event) {
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
    pub fn new(orientation: Orientation, first: impl Component + 'static, second: impl Component + 'static) -> Splitter {
        Splitter {
            id: format!("id{id}",id=Uuid::new_v4()),
            orientation,
            gutter_size: 4,
            first: Box::new(first),
            second : Box::new(second),
        }
    }
}

impl Component for Splitter {
    fn render(&self) -> String {
        format!(r#"<div id="{id}" data-role="splitter" class="h-100" data-split-mode="vertical">
                      <div class"d-flex">{first}</div>
                      <div class="d-flex">{second}</div>
                   </div>"#,
                    id=self.id,
                    first=self.first.render(),
                    second=self.second.render())
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
    components: Vec<Box<dyn Component>>,
}

impl Page {
    pub fn new() -> Self {
        Page {
            id: format!("id{id}",id=Uuid::new_v4()),
            components: Vec::new(),
        }
    }

    pub fn add_component(&mut self, component: impl Component + 'static) {
        self.components.push(Box::new(component));
    }
}

impl Component for Page {
    fn render(&self) -> String {
        let mut components = String::new();
        for comp in &self.components {
            components.push_str(comp.render().as_str());
        }
        format!("{components}", components=components)
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

pub struct Form {
    id: String,
    components: Vec<Box<dyn Component>>,
}

impl Form {
    pub fn new() -> Self {
        Form {
            id: format!("id{id}",id=Uuid::new_v4()),
            components: Vec::new(),
        }
    }

    pub fn add_line(&mut self, component: impl Component + 'static) {
        self.components.push(Box::new(component));
    }

    fn render_lines(&self) -> String {
        let mut lines = String::new();
        for comp in &self.components {
            lines.push_str(format!("<div class=\"form-group\">{line}</div>", line=comp.render()).as_str());
        }
        lines
    }
}

impl Component for Form {
    fn render(&self) -> String {
        format!("<form id=\"{id}\">{lines}</form>", id=self.id, lines=self.render_lines())
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
            id: Uuid::new_v4().to_string(),
            tabs: Vec::new(),
        }
    }

    pub fn add_tab(&mut self, label: String, content: impl Component + 'static) {
        self.tabs.push(Tab {
           label,
            content: Box::new(content),
        });
    }

    fn render_tab_headers(&self) -> String {
        let mut tabs = String::new();
        for tab in &self.tabs {
            tabs.push_str(format!("<li><a href=\"#{id}\">{label}</a></li>", id=tab.content.id(), label=tab.label).as_str());
        }
        tabs
    }

    fn render_tab_content(&self) -> String {
        let mut tabs = String::new();
        tabs.push_str("<div class=\"border bd-default no-border-top p-2 h-100\">");
        for tab in &self.tabs {
            tabs.push_str(tab.content.render().as_str());
        }
        tabs.push_str("</div>");
        tabs
    }
}

impl Component for TabPane {
    fn render(&self) -> String {
        format!(r#"<ul data-role="tabs" data-expand="true">{tabs}</ul>{content}"#, tabs=self.render_tab_headers(), content=self.render_tab_content())
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