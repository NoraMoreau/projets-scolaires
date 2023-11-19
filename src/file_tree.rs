use crate::size::Size;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use std::env;

#[derive(Debug)]
pub struct FileTree {
    root: PathBuf,
    map: HashMap<PathBuf, Option<EntryNode>>, //En attendant Option<None> puis après modifier
    //est une table de hachage qui associe à chaque nœud (identifié par son chemin) 
    //un objet EntryNode
}

#[derive(Debug)]
enum EntryNode { //En attendant Option<None> puis après modifier
    Taille,
    Liste,
    //capturer les informations d'un fichier/répertoire, taille de ses fichier, liste des fichier du repertoire
}

impl FileTree {
    pub fn new(root: &Path) -> std::io::Result<Self> {
        //on va utiliser read_dir pour lire le repertoire ou fichier root
        //on va mettre les resultats de root dans la table de hachage et l'envoyer
        //dans un FilTree

        let mut table: HashMap<PathBuf, Option<EntryNode>> = HashMap::new();
        if let Ok(entrées) = fs::read_dir(root) {
            for x in entrées {
                if let Ok(entrée) = x {
                    let chemin_entrée = entrée.path();
                    table.insert(chemin_entrée, None);
                }
            }
        } else {
            // Afficher le message d'erreur si la lecture du répertoire échoue
            if let Err(err) = fs::read_dir(root) {
                // Afficher le message d'erreur détaillé
                eprintln!("Erreur lors de la lecture du répertoire {:?} : {}", root, err);
                return Err(err);
            }
        }

        Ok(FileTree {
            root: root.to_path_buf(),
            map: table,
        })
  
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

#[cfg(test)]
mod tests{
    use super::*; //ne pas oublier d'importer les crate!!!
    #[test]
    fn test_read_dir() {
        let chemin = fs::read_dir("src/test");
        println!("Chemin: {:?}", chemin);
        assert!(chemin.is_ok());
    }
   
    #[test]
    fn test_path_existe() {
        let c = std::path::Path::new("src/test");

        if let Ok(current_dir) = env::current_dir() {
            println!("Répertoire de travail actuel : {:?}", current_dir);
        } else {
            eprintln!("Erreur lors de l'obtention du répertoire de travail actuel");
        }


        if let Ok(exist_dir) = fs::metadata(c) {
            //Le test passe si on se trouve ici, mais si nous somme dans le else il échoue
            println!("Le répertoire existe.");
        } else {
            println!("Le répertoire n'existe pas ou il y a une erreur lors de l'accès.");
            // Fait échouer le test si le répertoire n'existe pas ou s'il y a une erreur
            assert!(false, "Le répertoire n'existe pas ou il y a une erreur lors de l'accès.");
        }
    }

    #[test]
    fn test_path() {
        let c = std::path::Path::new("src/test");
        let tree = FileTree::new(c);

        if let Ok(tree) = tree {
            println!("FileTree: {:?} ", tree);
            println!("Map: {:?} \n", tree.map);
            for (path, entry) in tree.map {
                println!("Path: {:?}, Entry : {:?} ", path, entry);
            }
            println!("\n")
        }
    }
    
}
