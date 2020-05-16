use crate::component::Component;
use crate::event::Event;
use crate::utils::create_id;
use crate::event::EventValue::*;
use web_view::WebView;

/// Trait to provide date for the tree
pub trait TreeModel<U> {
    /// get the roots of the tree.
    fn roots(&self) -> Vec<U>;
    /// get a node's children
    fn children(&self, parent: &U) -> Vec<U>;
    // get the label to be displayed for a node.
    fn label(&self, node: &U) -> &str;
    /// has this nod children
    fn has_children(&self, node: &U) -> bool;
}

/// Internal representation of the current tree structure.
struct TreeNode<U> {
    id: String,
    caption: String,
    nodes: Vec<TreeNode<U>>,
    has_children: bool,
    children_loaded: bool,
    user_object: U,
}

impl<U> TreeNode<U> {
    /// cCreate a new tree node
    fn new(caption: impl Into<String>, user_object: U, has_children: bool) -> Self {
        TreeNode {
            id: create_id(),
            caption: caption.into(),
            nodes: Vec::new(),
            has_children,
            children_loaded: false,
            user_object,
        }
    }

    /// render this tree node as HTML.
    fn render_node(&self) -> String {
        format!(
            r#"<li id="{id}" data-caption="{caption}" data-collapsed="true">{children}</li>"#,
            id = self.id,
            caption = self.caption,
            children = if self.has_children {"<ul></ul>"} else {""},
        )
    }

    /// find a child nod recursively by its id.
    fn find_node(&self, id: &str) -> Option<&TreeNode<U>> {
        if self.id==id {
            return Some(self);
        }

        for child in &self.nodes {
            let found_child = child.find_node(id);
            if found_child.is_some() {
                return found_child;
            }
        }
        None
    }

    /// return a mutable child nod by its id.
    fn find_node_mut(&mut self, id: &str) -> Option<&mut TreeNode<U>> {
        if self.id==id {
            return Some(self);
        }

        for child in &mut self.nodes {
            let found_child = child.find_node_mut(id);
            if found_child.is_some() {
                return found_child;
            }
        }
        None
    }
}

/// Tree widget
pub struct Tree<U> {
    id: String,
    roots: Vec<TreeNode<U>>,
    click_event: Option<Box<dyn Fn(&U)>>,
    model: Box<dyn TreeModel<U>>,
}

impl<U> Tree<U> {
    pub fn new(model: impl TreeModel<U> + 'static) -> Self {
        Tree {
            id: create_id(),
            roots: Vec::new(),
            click_event: None,
            model: Box::new(model),
        }
    }

    pub fn set_click_event(&mut self, event: impl Fn(&U) + 'static) {
        self.click_event = Some(Box::new(event));
    }

    fn create_tree_node(&self, node_object: U) -> TreeNode<U> {
        let caption = self.model.label(&node_object);
        let has_children = self.model.has_children(&node_object);
        TreeNode::new(caption, node_object, has_children)
    }

    fn render_roots(&mut self) -> String {
        for root in self.model.roots() {
            self.roots.push(self.create_tree_node(root));
        }

        let mut s = String::new();
        for root in &self.roots {
            s.push_str(root.render_node().as_str());
        }
        s
    }

    /// find a tree node below the current roots.
    fn find_node(&self, id: &str) -> Option<&TreeNode<U>> {
        for child in &self.roots {
            let found_child = child.find_node(id);
            if found_child.is_some() {
                return found_child;
            }
        }
        None
    }

    /// find a mutable tree node below the current roots.
    fn find_node_mut(&mut self, id: &str) -> Option<&mut TreeNode<U>> {
        for child in &mut self.roots {
            let found_child = child.find_node_mut(id);
            if found_child.is_some() {
                return found_child;
            }
        }
        None
    }

    fn create_children(&mut self, node_id: &str, webview: &mut WebView<()>) {
        let child = self.find_node(node_id);
        if child.is_some() {
            if child.unwrap().children_loaded {
                return;
            }
            let child_node = child.unwrap();
            let user_object = &child_node.user_object;
            let new_children = self.model.children(user_object);
            for new_child in new_children {
                let new_node = self.create_tree_node(new_child);
                let js = format!("add_tree_node('{id_tree}', '{id_parent}', '{id_node}', '{caption}')",
                    id_tree=self.id,
                    id_parent=node_id,
                    id_node=new_node.id,
                    caption=new_node.caption);
                let result = webview.eval(js.as_str());
                if result.is_ok() {
                    let mut_child = self.find_node_mut(node_id).unwrap();
                    mut_child.children_loaded = true;
                    mut_child.nodes.push(new_node);
                }
            }
        }
    }
}

impl<U> Component for Tree<U> {
    fn render(&mut self) -> String {
        let roots = self.render_roots();
        format!(
            r#"<ul id="{id}" data-role="treeview" data-on-expand-node="fire_node_expand" data-on-tree-view-create="fire_created" data-on-node-click="fire_node_clicked">{roots}</ul>"#,
            id = self.id(),
            roots = roots,
        )
    }

    fn handle_event(&mut self, webview: &mut WebView<()>, event: &Event) {
        if event.id == self.id {
            match &event.value {
                ChildClicked(child_id) => {
                    if self.click_event.is_some() {
                        let child = self.find_node(child_id);
                        if child.is_some() {
                            self.click_event.as_ref().unwrap()(&child.unwrap().user_object);
                        }
                    }
                },
                NodeExpand(child_id) => {
                    self.create_children(child_id, webview);
                }
                _ => {}
            }
        }
    }

    fn id(&self) -> &str {
        &*self.id
    }
}
