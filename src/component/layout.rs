use crate::component::Component;
use crate::event::Event;

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