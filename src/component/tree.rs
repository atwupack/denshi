use crate::component::Component;
use crate::event::Event;
use crate::event::EventValue::*;
use crate::utils::create_id;
use log::warn;
use web_view::WebView;
use std::rc::Rc;
use std::cell::RefCell;

/// Trait to provide date for the tree
pub trait TreeModel<U> {
    /// get the roots of the tree.
    fn roots(&self) -> Vec<U>;
    /// get a node's children
    fn children(&self, parent: &U) -> Vec<U>;
    // get the label to be displayed for a node.
    fn caption(&self, node: &U) -> String;
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
    /// Create a new tree node
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
            children = if self.has_children { "<ul></ul>" } else { "" },
        )
    }

    /// find a child node recursively by its id.
    fn find_node(&self, id: &str) -> Option<&TreeNode<U>> {
        if self.id == id {
            return Some(self);
        }

        for child in &self.nodes {
            let found_child = child.find_node(id);
            if found_child.is_some() {
                return found_child.clone();
            }
        }
        None
    }

    /// return a mutable child node by its id.
    fn find_node_mut(&mut self, id: &str) -> Option<&mut TreeNode<U>> {
        if self.id == id {
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
#[derive(Clone)]
pub struct Tree<U> {
    id: String,
    roots: Rc<RefCell<Vec<TreeNode<U>>>>,
    click_event: Rc<RefCell<Option<Box<dyn Fn(&mut WebView<()>, &U)>>>>,
    model: Rc<dyn TreeModel<U>>,
}

impl<U> Tree<U> {
    pub fn new(model: impl TreeModel<U> + 'static) -> Self {
        Tree {
            id: create_id(),
            roots: Rc::new(RefCell::new(Vec::new())),
            click_event: Rc::new(RefCell::new(None)),
            model: Rc::new(model),
        }
    }

    pub fn set_click_event(&self, event: impl Fn(&mut WebView<()>, &U) + 'static) {
        self.click_event.borrow_mut().replace(Box::new(event));
    }

    fn create_tree_node(&self, node_object: U) -> TreeNode<U> {
        let caption = self.model.caption(&node_object);
        let has_children = self.model.has_children(&node_object);
        TreeNode::new(caption, node_object, has_children)
    }

    fn render_roots(&mut self) -> String {
        for root in self.model.roots() {
            self.roots.borrow_mut().push(self.create_tree_node(root));
        }

        let mut s = String::new();
        for root in &*self.roots.borrow_mut() {
            s.push_str(root.render_node().as_str());
        }
        s
    }

    fn create_children(&mut self, parent_id: &str, webview: &mut WebView<()>) {
        let mut roots = self.roots.borrow_mut();
        if let Some(parent_node) = find_tree_node(roots.as_mut(), parent_id) {
            if parent_node.children_loaded {
                return;
            }

            let clean_js = format!(
                "clear_node('{tree_id}', '{node_id}')",
                tree_id = self.id,
                node_id = parent_id
            );
            let _clean_result = webview.eval(clean_js.as_str());

            let user_object = &parent_node.user_object;
            let new_children = self.model.children(user_object);
            for new_child in new_children {
                let new_node = self.create_tree_node(new_child);
                let js = format!("add_tree_node('{id_tree}', '{id_parent}', '{id_node}', '{caption}', {has_children})",
                                 id_tree=self.id,
                                 id_parent=parent_id,
                                 id_node=new_node.id,
                                 caption=new_node.caption,
                                 has_children=new_node.has_children);
                let result = webview.eval(js.as_str());
                if result.is_ok() {
                    let mut_child = find_tree_node_mut(roots.as_mut(), parent_id).unwrap();
                    mut_child.children_loaded = true;
                    mut_child.nodes.push(new_node);
                }
            }
        }
    }
}

/// find a tree node below the current roots.
fn find_tree_node<'a, U>(roots: &'a Vec<TreeNode<U>>, id: &str) -> Option<&'a TreeNode<U>> {

    for child in roots {
        let found_child = child.find_node(id);
        if found_child.is_some() {
            return found_child;
        }
    }
    None
}

/// find a mutable tree node below the current roots.
fn find_tree_node_mut<'a, U>(roots: &'a mut Vec<TreeNode<U>>, id: &str) -> Option<&'a mut TreeNode<U>> {
    for child in roots {
        let found_child = child.find_node_mut(id);
        if found_child.is_some() {
            return found_child;
        }
    }
    None
}

impl<U: Clone> Component for Tree<U> {
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
                    if let Some(listener) = self.click_event.borrow_mut().as_ref() {
                        let roots = self.roots.borrow_mut();
                        if let Some(child) = find_tree_node(roots.as_ref(), child_id) {
                            listener(webview, &child.user_object);
                        } else {
                            warn!(target: "tree" , "Could not find child with ID {}", child_id);
                        }
                    } else {
                        warn!(target: "tree" , "No listener for tree with ID {}", self.id);
                    }
                }
                NodeExpand(child_id) => {
                    self.create_children(child_id, webview);
                }
                _ => {}
            }
        }
    }

    fn id(&self) -> String {
        self.id.clone()
    }
}
