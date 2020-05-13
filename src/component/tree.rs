use crate::component::Component;
use crate::event::Event;
use crate::utils::create_id;
use crate::event::EventValue::ChildClicked;

pub struct TreeNode<U> {
    id: String,
    caption: String,
    nodes: Vec<TreeNode<U>>,
    user_object: U,
}

impl<U> TreeNode<U> {

    pub fn new(caption: impl Into<String>, user_object: U) -> Self {
        TreeNode {
            id: create_id(),
            caption: caption.into(),
            nodes: Vec::new(),
            user_object,
        }
    }

    pub fn add_child(&mut self, node: TreeNode<U>) {
        self.nodes.push(node)
    }

    pub fn render_node(&self) -> String {
        format!(
            r#"<li id="{id}" data-caption="{caption}" data-collapsed="true">{children}</li>"#,
            id = self.id,
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

    fn find_child_node(&self, id: &str) -> Option<&TreeNode<U>> {
        if self.id==id {
            return Some(self);
        }

        for child in &self.nodes {
            let found_child = child.find_child_node(id);
            if found_child.is_some() {
                return found_child;
            }
        }
        None
    }
}

pub struct Tree<U> {
    id: String,
    roots: Vec<TreeNode<U>>,
    click_event: Option<Box<dyn Fn(&U)>>,
}

impl<U> Tree<U> {
    pub fn new() -> Self {
        Tree {
            id: create_id(),
            roots: Vec::new(),
            click_event: None,
        }
    }

    pub fn set_click_event(&mut self, event: impl Fn(&U) + 'static) {
        self.click_event = Some(Box::new(event));
    }

    pub fn add_root(&mut self, node: TreeNode<U>) {
        self.roots.push(node)
    }

    fn render_roots(&self) -> String {
        let mut s = String::new();
        for root in &self.roots {
            s.push_str(root.render_node().as_str());
        }
        s
    }

    fn find_child_node(&self, id: &str) -> Option<&TreeNode<U>> {
        for child in &self.roots {
            let found_child = child.find_child_node(id);
            if found_child.is_some() {
                return found_child;
            }
        }
        None
    }
}

impl<U> Component for Tree<U> {
    fn render(&self) -> String {
        format!(
            r#"<ul id="{id}" data-role="treeview" data-on-node-click="fire_node_clicked">{roots}</ul>"#,
            id = self.id,
            roots = self.render_roots()
        )
    }

    fn handle_event(&mut self, event: &Event) {
        if event.id == self.id {
            match &event.value {
                ChildClicked(child_id) => {
                    dbg!(child_id);
                    let child = self.find_child_node(child_id);
                    dbg!(&child.unwrap().caption);
                    if self.click_event.is_some() {
                        self.click_event.as_ref().unwrap()(&child.unwrap().user_object);
                    }
                },
                _ => {}
            }
        }
    }

    fn id(&self) -> &str {
        &*self.id
    }
}
