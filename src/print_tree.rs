use std::path::PathBuf;
use std::cmp::Reverse;
use crate::file_tree::FileTree;
use crate::size::Size;
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
            // Récupération et tri des enfants par taille décroissante
            match self.get_children(path){
                Some(children)=> {
                    //tri des enfants par taille décroissante
                    let mut children_vec :Vec<_>= children.iter().cloned().collect();
                    children_vec.sort_unstable_by_key(|child| Reverse(self.get_size(child).unwrap_or(Size::new(0))));
                    for child in children_vec{
                        self.show_node(&child, level+1);
                    }
                },
                None => {},
            }

        }
    }
}
