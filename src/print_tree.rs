use std::path::PathBuf;
use std::cmp::Reverse;
use crate::file_tree::FileTree;
use crate::size::Size;
impl FileTree {
    pub fn show(&self,lexicographic_sort:bool) {
        let path = PathBuf::from(self.get_root());
        self.show_node(&path,0,lexicographic_sort);
    }

    pub fn show_node(&self,path:&PathBuf,level:usize,lexicographic_sort:bool){
        if let Some(_node)=self.get_node(path){
            let indent="    ".repeat(level);
            if let Some(size)=self.get_size(path){
                println!("{}{} {}{}",indent, size, indent, path.display());
            }
            // Récupération et tri des enfants soit par taille decr ou ordre lexicographic
            match self.get_children(path){
                Some(children)=> {
                    
                    let mut children_vec :Vec<_>= children.iter().cloned().collect();
                    if lexicographic_sort{
                        // tri des enfants par tri lexicographic
                        children_vec.sort_by_key(|child| child.clone());
                    }
                    else {
                        //tri des enfants par taille décroissante
                        children_vec.sort_unstable_by_key(|child| Reverse(self.get_size(child).unwrap_or(Size::new(0))));
                    }
                    
                    for child in children_vec{
                        self.show_node(&child, level+1,lexicographic_sort);
                    }
                },
                None => {},
            }

        }
    }
}
