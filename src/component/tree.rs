use crate::component::Component;
use crate::event::Event;
use crate::utils::create_id;

pub struct TreeNode {
    caption: String,
    nodes: Vec<TreeNode>,
}

impl TreeNode {
    pub fn new(caption: impl Into<String>) -> Self {
        TreeNode {
            caption: caption.into(),
            nodes: Vec::new(),
        }
    }

    pub fn add_child(&mut self, node: TreeNode) {
        self.nodes.push(node)
    }

    pub fn render_node(&self) -> String {
        format!(
            "<li data-caption=\"{caption}\" data-collapsed=\"true\">{children}</li>",
            caption = self.caption,
            children = self.render_children()
        )
    }

    fn render_children(&self) -> String {
        if self.nodes.is_empty() {
            "".to_string()
        } else {
            let mut s = "<ul>".to_string();
            for node in &self.nodes {
                s.push_str(node.render_node().as_str());
            }
            s.push_str("</ul>");
            s
        }
    }
}

pub struct Tree {
    id: String,
    roots: Vec<TreeNode>,
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            id: create_id(),
            roots: Vec::new(),
        }
    }

    pub fn add_root(&mut self, node: TreeNode) {
        self.roots.push(node)
    }

    fn render_roots(&self) -> String {
        let mut s = String::new();
        for root in &self.roots {
            s.push_str(root.render_node().as_str());
        }
        s
    }
}

impl Component for Tree {
    fn render(&self) -> String {
        format!(
            r#"<ul id="{id}" data-role="treeview">{roots}</ul>"#,
            id = self.id,
            roots = self.render_roots()
        )
    }

    fn handle_event(&mut self, _event: &Event) {
        println!("Tree created");
    }

    fn id(&self) -> &str {
        &*self.id
    }
}
