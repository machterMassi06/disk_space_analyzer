use crate::size::Size;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;

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
        let root_path=PathBuf::from(root);
        let mut map =HashMap::new();

        Self::populate_map(&root_path,&mut map)?;
        Ok(FileTree { root:root_path, map })
    }

    fn populate_map(root :&PathBuf,map : &mut HashMap<PathBuf, EntryNode>)->std::io::Result<()>{
        let entries = fs::read_dir(root)?;
        for entry in entries{
            let entry =entry?;
            let path =entry.path();
            
            if path.is_dir(){
                // si c'est un repertoire
                map.insert(path.clone(), EntryNode::Directory { children: Vec::new() });
                Self::populate_map(&path, map)?;// appel recursifs sur ses enfants
                // Enregistrer ce répertoire dans son parent 
                if let Some(parent) = path.parent() {
                    if let Some(EntryNode::Directory { children }) = map.get_mut(parent) {
                        children.push(path.clone());
                    } 
                }

            }
            else if path.is_file(){
                // si c'est un fichier ,recup sa taille 
                let size=Size::new(path.metadata()?.len());
                map.insert(path.clone(), EntryNode::File { size});
                // Enregistrer ce fichier dans son répertoire parent
                if let Some(parent) = path.parent() {
                    if let Some(EntryNode::Directory { children}) = map.get_mut(parent) {
                        children.push(path.clone());
                    }
                }
            }
        } 
        Ok(())
    }
    pub fn get_root(&self) -> &Path {
        &self.root
    }

    pub fn get_children(&self, path: &Path) -> Option<&[PathBuf]> {
        match self.map.get(path){
            Some(EntryNode::Directory { children })=> Some(children),
            _=> None ,
        }
    }

    pub fn get_size(&self, path: &Path) -> Option<Size> {
        match self.map.get(path){
            Some(node)=> match node{
                EntryNode::File { size }=>Some(size.clone()),
                EntryNode::Directory { children }=> {
                    let total_size=children.iter().filter_map(|child|{
                        match self.map.get(child){
                            Some(child_node)=>match child_node{
                                EntryNode::File {size} =>Some(size.to_bytes()),
                                EntryNode::Directory {..}=>{
                                    // Appel récursif pour les répertoires
                                    self.get_size(child).map(|s| s.to_bytes())
                                },
                            },
                            None=>None ,
                        }
                    }).sum();
                    Some(Size::new(total_size))
                }
            },
            None => None ,
        }
    }

    pub fn files(&self) -> Vec<PathBuf> {
        let mut files = Vec::new();
        let mut stack = vec![self.root.clone()];
        while let Some(current_path)=stack.pop(){
            if let Some(node)=self.map.get(&current_path){
                match node {
                    EntryNode::File {..}=>files.push(current_path.clone()),
                    EntryNode::Directory { children }=>{
                        for child in children{
                            stack.push(child.clone());
                        }
                    }
                }
            }
        }
        files
    }
}


