use uuid::Uuid;
use crate::component::Component;
use crate::event::Event;

pub struct TreeNode {
    caption: String,
}

pub struct Tree {
    id: String,
    roots: Vec<TreeNode>,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            id: format!("id{id}",id=Uuid::new_v4()),
            roots: Vec::new(),
        }
    }
}

impl Component for Tree {
    fn render(&self) -> String {
        format!(r#"<ul id="{id}" data-role="treeview"><li data-caption="Test"></li > </ul>"#, id=self.id)
    }

    fn handle_event(&mut self, event: &Event) {
        println!("Tree created");
    }

    fn id(&self) -> &str {
        &*self.id
    }
}