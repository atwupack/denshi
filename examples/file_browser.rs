use denshi::component::layout::{Splitter, Orientation, Page};
use denshi::component::tree::{Tree, TreeModel};
use denshi::component::panel::Panel;
use denshi::App;
use std::error::Error;
use std::fs::File;

use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};


struct FileTreeModel {}

impl FileTreeModel {

}

impl TreeModel<PathBuf> for FileTreeModel {
    fn roots(&self) -> Vec<PathBuf> {
        let dir = fs::read_dir("/").unwrap();
        let mut entries = Vec::new();
        for entry in dir {
            entries.push(entry.unwrap().path())

        }
        entries
    }

    fn children(&self, parent: &PathBuf) -> Vec<PathBuf> {
        let dir = fs::read_dir(parent).unwrap();
        let mut entries = Vec::new();
        for entry in dir {
            entries.push(entry.unwrap().path())

        }
        entries
    }

    fn label(&self, node: &PathBuf) -> String {
        node.file_name().unwrap().to_str().unwrap().into()
    }

    fn has_children(&self, node: &PathBuf) -> bool {
        node.is_dir()
    }
}


fn main() -> Result<(), Box<dyn Error>> {

    let file_tree = Tree::new(FileTreeModel{});

    let file_panel = Panel::new();

    let split = Splitter::new(Orientation::HORIZONTAL, file_tree, file_panel);

    let mut page = Page::new();
    page.set_content(split);

    let app = App::new("File Browser", page);
    app.run()

}
