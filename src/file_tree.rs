use crate::size::Size;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct FileTree {
    root: PathBuf,
    map: HashMap<PathBuf, EntryNode>,
}
// implementation de l'enum selon mes besoin 
// peuvent etre soit des fichiers soit des repertoires 
enum EntryNode {
    File {size : Size},
    Directory{children : Vec<PathBuf>},
}

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
