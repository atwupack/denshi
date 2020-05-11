use denshi::App;
use denshi::component::button::{Button, Checkbox};
use denshi::component::text::TextField;
use denshi::component::layout::{Form, Page, TabPane, Splitter, Orientation};
use denshi::component::menu::MenuBar;
use denshi::component::panel::Panel;
use denshi::component::tree::{Tree, TreeNode};

struct DemoState {
    text: String,
}

fn build_tree() -> Tree {
    let mut tree = Tree::new();

    let mut comp_node = TreeNode::new("Components");
    comp_node.add_child(TreeNode::new("Buttons"));
    tree.add_root(comp_node);

    let mut cont_node = TreeNode::new("Containers");
    cont_node.add_child(TreeNode::new("Forms"));
    tree.add_root(cont_node);

    let mut layout_node = TreeNode::new("Layouts");
    layout_node.add_child(TreeNode::new("Page Layout"));
    tree.add_root(layout_node);

    tree
}

fn main() {

    let state = DemoState {
        text: "".to_string(),
    };

    // create menu
    let mut menu = MenuBar::new();
    menu.add_entry("File".to_string());
    menu.add_entry("Edit".to_string());
    menu.add_entry("Help".to_string());

    // create tab pane
    let mut tabs = TabPane::new();

    // form tab
    let mut form = Form::new();
    let mut button = Button::new("Test Button".to_owned());
    button.set_click_event(|| {
       dbg!("Clicked");
    });
    let text = TextField::new("Enter Name: ".to_owned());

    let checkbox = Checkbox::new("Checkbox".to_owned());

    //form.add_line(menu);
    form.add_line(text);
    form.add_line(button);
    form.add_line(checkbox);

    tabs.add_tab("Form".to_string(), form);

    // second tab
    let mut left = Panel::new();
    left.set_title("Left".to_string());

    let mut right = Panel::new();
    right.set_title("Right".to_string());

    let split = Splitter::new(Orientation::VERTICAL, left, right);
    tabs.add_tab("Splitter".to_string(), split);

    // create tree
    let tree = build_tree();

    // create split pane
    let main_split = Splitter::new(Orientation::HORIZONTAL, tree, tabs);



    let mut page = Page::new();
    page.set_header(menu);
    page.set_content(main_split);

    let app = App::new("Demo".to_owned(), page);
    app.run();
}
