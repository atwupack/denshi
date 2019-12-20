use crate::component::Component;
use crate::event::Event;

pub enum Orientation {
    VERTICAL,
    HORIZONTAL,
}

pub struct Splitter {
    orientation: Orientation,
    gutter_size: u8,
    first: Box<dyn Component>,
    second: Box<dyn Component>,
}

impl Splitter {
    pub fn new(orientation: Orientation, first: impl Component + 'static, second: impl Component + 'static) -> Splitter {
        Splitter {
            orientation,
            gutter_size: 4,
            first: Box::new(first),
            second : Box::new(second),
        }
    }
}

impl Component for Splitter {
    fn render(&self) -> String {
        format!(r#"<div data-role="splitter" class="h-100">
                      <div>{first}</div>
                      <div>{second}</div>
                   </div>"#,
                    first=self.first.render(),
                    second=self.second.render())
    }

    fn handle_event(&mut self, event: &Event) {
        self.first.handle_event(event);
        self.second.handle_event(event);
    }
}


pub struct Form {
    components: Vec<Box<dyn Component>>,
}

impl Form {
    pub fn new() -> Self {
        Form {
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
        format!("<form>{lines}</form>", lines=self.render_lines())
    }

    fn handle_event(&mut self, event: &Event) {
        for comp in &mut self.components {
            comp.handle_event(event);
        }
    }
}