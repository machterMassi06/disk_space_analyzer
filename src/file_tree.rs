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

        Self::populate_map(&root_path,&mut map);
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
