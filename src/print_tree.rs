use std::path::PathBuf;

use crate::file_tree::FileTree;

impl FileTree {
    pub fn show(&self) {
        let path = PathBuf::from(self.get_root());
        self.show_node(&path,0);
    }

    pub fn show_node(&self,path:&PathBuf,level:usize){
        if let Some(_node)=self.get_node(path){
            let indent="    ".repeat(level);
            if let Some(size)=self.get_size(path){
                println!("{}{} {}{}",indent, size, indent, path.display());
            }
            match self.get_children(path){
                Some(children)=> {
                    for child in children{
                        self.show_node(child, level+1);
                    }
                },
                None => {},
            }

        }
    }
}
