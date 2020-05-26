#![windows_subsystem = "windows"]
use crate::Section::{Buttons, Components, Containers, Forms, Layouts, PageLayout};
use denshi::component::button::{Button, Checkbox};
use denshi::component::layout::{Form, Orientation, Page, Splitter, TabPane};
use denshi::component::menu::MenuBar;
use denshi::component::panel::Panel;
use denshi::component::text::{TextArea, TextField};
use denshi::component::tree::{Tree, TreeModel};
use denshi::App;
use log::{debug, LevelFilter};
use simplelog::{Config, SimpleLogger};
use std::error::Error;
use denshi::component::CompRef;
use std::rc::Rc;
use std::cell::RefCell;


#[derive(Debug)]
enum Section {
    Components,
    Buttons,
    Containers,
    Forms,
    Layouts,
    PageLayout,
}

struct SectionTree {}

impl TreeModel<Section> for SectionTree {
    fn roots(&self) -> Vec<Section> {
        vec![Components, Containers, Layouts]
    }

    fn children(&self, parent: &Section) -> Vec<Section> {
        match parent {
            Components => vec![Buttons],
            Containers => vec![Forms],
            Layouts => vec![PageLayout],
            _ => Vec::new(),
        }
    }

    fn caption(&self, node: &Section) -> String {
        match node {
            Components => "Components".into(),
            Buttons => "Buttons".into(),
            Containers => "Containers".into(),
            Forms => "Forms".into(),
            Layouts => "Layouts".into(),
            PageLayout => "Page Layout".into(),
        }
    }

    fn has_children(&self, node: &Section) -> bool {
        match node {
            Components | Containers | Layouts => true,
            _ => false,
        }
    }
}

fn build_tree() -> Tree<Section> {
    Tree::new(SectionTree {})
}

fn build_form() -> Form {
    // form tab
    let mut form = Form::new();
    let mut button = Button::new("Test Button");
    button.set_click_event(|| {
        debug!("Clicked test button");
    });
    let text = TextField::new("Enter Name: ");

    let checkbox = Checkbox::new("Checkbox");

    form.add_line(text);
    form.add_line(button);
    form.add_line(checkbox);
    form
}

fn build_splitter() -> Splitter {
    let mut left = Panel::new(Button::new("Left"));
    left.set_title("Left");
    left.set_collapsible(true);

    let mut right = Panel::new(Button::new("RightS"));
    right.set_title("Right");

    Splitter::new(Orientation::VERTICAL, left, right)
}

fn build_text_area() -> TextArea {
    TextArea::new()
}

fn main() -> Result<(), Box<dyn Error>> {

    let mut app = App::new("Demo");

    // init logging
    SimpleLogger::init(LevelFilter::Debug, Config::default())?;

    // create menu
    let mut menu = MenuBar::new();
    menu.add_entry("File");
    menu.add_entry("Edit");
    menu.add_entry("Help");

    // create tab pane
    let mut tabs = TabPane::new();

    tabs.add_tab("Form", build_form());
    tabs.add_tab("Splitter", build_splitter());
    tabs.add_tab("Text Area", build_text_area());

    // create tree
    let mut tree = build_tree();
    let app_clone = app.clone();
    tree.set_click_event(move |webview, section| {
        app_clone.send(webview, section)
    });
    let tree_ref = CompRef::new(tree);

    // create split pane
    let main_split = Splitter::new(Orientation::HORIZONTAL, tree_ref.clone(), tabs);
    let split_ref = Rc::new(RefCell::new(main_split));

    app.subscribe(|_webview, event: &Section| {
        debug!("Received event: {:?}", event);

    });

    let split_ref = CompRef::new(CompRef::new_from_rc(&split_ref));

    let mut page = Page::new();
    page.set_header(menu);
    page.set_content(split_ref);

    app.set_content(page);
    app.run()
}
