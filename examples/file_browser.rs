#![windows_subsystem = "windows"]
use denshi::component::layout::{Orientation, Page, Splitter};
use denshi::component::panel::Panel;
use denshi::component::tree::{Tree, TreeModel};
use denshi::App;
use std::error::Error;

use denshi::component::button::Button;
use log::LevelFilter;
use simplelog::{Config, SimpleLogger};
use std::fs;
use std::path::PathBuf;
use systemstat::{Platform, System};

struct FileTreeModel {}

impl FileTreeModel {}

impl TreeModel<PathBuf> for FileTreeModel {
    fn roots(&self) -> Vec<PathBuf> {
        let mut entries = Vec::new();

        let sys = System::new();
        match sys.mounts() {
            Ok(mounts) => {
                for mount in mounts.iter() {
                    entries.push(mount.fs_mounted_on.as_str().into());
                }
            }
            Err(x) => {}
        }
        entries
    }

    fn children(&self, parent: &PathBuf) -> Vec<PathBuf> {
        let mut entries = Vec::new();
        let read = fs::read_dir(parent);
        if read.is_ok() {
            let dir = read.unwrap();
            for entry in dir {
                entries.push(entry.unwrap().path())
            }
        }
        entries
    }

    fn caption(&self, node: &PathBuf) -> String {
        if node.file_name().is_some() {
            node.file_name().unwrap().to_str().unwrap().into()
        } else {
            node.as_os_str().to_str().unwrap().into()
        }
    }

    fn has_children(&self, node: &PathBuf) -> bool {
        node.is_dir()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // init logging
    SimpleLogger::init(LevelFilter::Debug, Config::default())?;

    let file_tree = Tree::new(FileTreeModel {});

    let mut tree_panel = Panel::new(file_tree);
    tree_panel.set_title("File Tree");

    let mut file_panel = Panel::new(Button::new("Click"));
    file_panel.set_title("Directory Content");

    let split = Splitter::new(Orientation::HORIZONTAL, tree_panel, file_panel);

    let mut page = Page::new();
    page.set_content(split);

    let mut app = App::new("File Browser");
    app.set_content(page);
    app.run()
}
