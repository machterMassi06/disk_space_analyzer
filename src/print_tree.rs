use std::path::PathBuf;
use std::cmp::Reverse;
use crate::file_tree::FileTree;
use crate::size::Size;
impl FileTree {
    pub fn show(&self,lexicographic_sort:bool,filter:Option<String>) {
        let path = PathBuf::from(self.get_root());
        match filter {
            Some(ext)=> self.show_filtred_files(&ext,lexicographic_sort),
            None => self.show_node(&path,0,lexicographic_sort),
        }
        
    }
    pub fn show_filtred_files(&self,ext:&str,lexicographic_sort:bool){
        // afficher les fichiers selon le filter (e.g,.jpg)
        let files = self.files();
        let mut files_filtred = Vec::new();
        for f in files{
            if let Some(file_ext)=f.extension(){
                if file_ext== &ext[1..]{
                    if let Some(size)=self.get_size(&f){
                        files_filtred.push((size,f));
                    }
                }
            }
        }
        // Tri des fichiers 
        if lexicographic_sort{
            files_filtred.sort_by_key(|(_,path)| path.clone());
        }else{
            files_filtred.sort_by_key(|(size,_)| *size);
            files_filtred.reverse();
        }
        for (s,path) in files_filtred{
            println!("{} {}", s, path.display());
        }
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
    pub fn detect_duplicates(&self){
        let duplicates = self.get_duplicate_files();
        match duplicates.len(){
            0=> println!("Aucun fichier dupliqué trouvé."),
            _ => {
                println!("Fichiers dupliqués détectés :");
                for group in duplicates{
                    println!("--- Groupe de fichiers dupliqués ---");
                    for file in group{
                        println!("{}",file.display());
                    }
                    println!();
                }
            }
        }
    }
}
