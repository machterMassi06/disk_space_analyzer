use crate::size::Size;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct FileTree {
    root: PathBuf,
    map: HashMap<PathBuf, EntryNode>,
}

enum EntryNode {}

impl FileTree {
    pub fn new(root: &Path) -> std::io::Result<Self> {
        unimplemented!()
    }

    pub fn get_root(&self) -> &Path {
        unimplemented!()
    }

    pub fn get_children(&self, path: &Path) -> Option<&[PathBuf]> {
        unimplemented!()
    }

    pub fn get_size(&self, path: &Path) -> Option<Size> {
        unimplemented!()
    }

    pub fn files(&self) -> &[PathBuf] {
        unimplemented!()
    }
}
