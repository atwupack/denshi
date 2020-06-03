use crate::event::Event;
use web_view::WebView;
use std::collections::HashMap;
use std::hash::Hash;

pub mod button;
pub mod layout;
pub mod menu;
pub mod panel;
pub mod text;
pub mod tree;

/// Functions every component needs to provide.
pub trait Component {
    /// Render the component as HTML.
    fn render(&mut self) -> String;
    /// Callback for events.
    fn handle_event(&mut self, webview: &mut WebView<()>, event: &Event);
    /// The globally unique ID for teh component.
    fn id(&self) -> String;
}

pub struct ComponentManager<K> {
    components: HashMap<K, Vec<Box<dyn Component>>>,
}

impl<K: Eq + Hash + Clone> ComponentManager<K> {
    pub fn new() -> Self {
        ComponentManager {
            components: HashMap::new(),
        }
    }

    pub fn set_component(&mut self, key: K, comp: impl Component + 'static) {
        self.components.insert(key, vec!(Box::new(comp)));
    }

    pub fn add_component(&mut self, key: K, comp: impl Component + 'static) {
        let comp_vec = self.components.get_mut(&key);
        if comp_vec.is_none() {
            self.components.insert(key.clone(),Vec::new());
        }
        let comp_vec = self.components.get_mut(&key);
        comp_vec.unwrap().push(Box::new(comp));
    }

    pub fn render_component(&mut self, key: &K) -> String {
        let comp_vec = self.components.get_mut(key);
        if comp_vec.is_none() {
            return "".into();
        }
        let mut comp_str = String::new();
        for comp in comp_vec.unwrap() {
            comp_str.push_str(comp.render().as_str())
        }
        comp_str
    }

    pub fn notify_all_components(&mut self, webview: &mut WebView<()>, event: &Event) {
        for value in self.components.values_mut() {
            for comp in value {
                comp.handle_event(webview, event)
            }
        }
    }
}




