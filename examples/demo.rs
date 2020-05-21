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
    let mut tree = Tree::new(SectionTree {});

    tree.set_click_event(|user_object| {
        debug!("Clicked node {:?}", user_object);
    });

    tree
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

    // create split pane
    let mut main_split = Splitter::new(Orientation::HORIZONTAL, build_tree(), tabs);
    main_split.set_gutter_size(10);

    let mut page = Page::new();
    page.set_header(menu);
    page.set_content(main_split);

    let mut app = App::new("Demo", page);
    app.run()
}
