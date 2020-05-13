use denshi::component::button::{Button, Checkbox};
use denshi::component::layout::{Form, Orientation, Page, Splitter, TabPane};
use denshi::component::menu::MenuBar;
use denshi::component::panel::Panel;
use denshi::component::text::{TextArea, TextField};
use denshi::component::tree::{Tree, TreeNode};
use denshi::App;
use std::error::Error;

#[derive(Debug)]
enum Section {
    Components,
    Buttons,
    Containers,
    Forms,
    Layouts,
    PageLayout,
}

fn build_tree() -> Tree<Section> {
    let mut tree = Tree::new();

    tree.set_click_event(|user_object| {
        dbg!(user_object);
    });

    let mut comp_node = TreeNode::new("Components",  Section::Components);
    comp_node.add_child(TreeNode::new("Buttons", Section::Buttons));
    tree.add_root(comp_node);

    let mut cont_node = TreeNode::new("Containers", Section::Containers);
    cont_node.add_child(TreeNode::new("Forms", Section::Forms));
    tree.add_root(cont_node);

    let mut layout_node = TreeNode::new("Layouts", Section::Layouts);
    layout_node.add_child(TreeNode::new("Page Layout", Section::PageLayout));
    tree.add_root(layout_node);

    tree
}

fn main() -> Result<(), Box<dyn Error>> {
    // create menu
    let mut menu = MenuBar::new();
    menu.add_entry("File");
    menu.add_entry("Edit");
    menu.add_entry("Help");

    // create tab pane
    let mut tabs = TabPane::new();

    // form tab
    let mut form = Form::new();
    let mut button = Button::new("Test Button");
    button.set_click_event(|| {
        dbg!("Clicked");
    });
    let text = TextField::new("Enter Name: ");

    let checkbox = Checkbox::new("Checkbox");

    form.add_line(text);
    form.add_line(button);
    form.add_line(checkbox);

    tabs.add_tab("Form", form);

    // second tab
    let mut left = Panel::new();
    left.set_title("Left");
    left.set_collapsible(true);

    let mut right = Panel::new();
    right.set_title("Right");

    let split = Splitter::new(Orientation::VERTICAL, left, right);
    tabs.add_tab("Splitter", split);

    // text area tab
    let area = TextArea::new();
    tabs.add_tab("Text Area", area);

    // create tree
    let tree = build_tree();

    // create split pane
    let mut main_split = Splitter::new(Orientation::HORIZONTAL, tree, tabs);
    main_split.set_gutter_size(10);

    let mut page = Page::new();
    page.set_header(menu);
    page.set_content(main_split);

    let app = App::new("Demo", page);
    app.run()
}
