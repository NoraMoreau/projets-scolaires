use crate::size::Size;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{fs, vec};
use std::env;

#[derive(Debug)]
pub struct FileTree {
    root: PathBuf,
    map: HashMap<PathBuf, EntryNode>,
    //est une table de hachage qui associe à chaque nœud (identifié par son chemin) 
    //un objet EntryNode
}

#[derive(Debug)]
enum EntryNode {
    Taille(Size), //pour un fichier la taille
    Liste(Vec<PathBuf>), //pour un dossier la liste de son contenu
}

impl FileTree {
    pub fn new(root: &Path) -> std::io::Result<Self> {

        let mut table: HashMap<PathBuf, EntryNode> = HashMap::new();

        if let Ok(entrées) = fs::read_dir(root) { //on récupère les enfants du rep dans entrées grâce à read_dir  
            for x in entrées {
                if let Ok(une_entrée) = x { //desencapsuler x = Result<DirEntry, Error>

                    if let Ok(meta) = fs::metadata(une_entrée.path()) { //récupère les metadonées des entrée
                       //Si c'est un dossier récuperer la liste de son contenu, si c'est un fichier récupérer sa taille
                       if meta.is_dir() {
                            let mut list_child: Vec<PathBuf> = Vec::new();
                            if let Ok(children) = fs::read_dir(une_entrée.path()) {
                                for y in children {
                                    if let Ok(child) = y {
                                        list_child.push(child.path());
                                    }
                                }
                            }

                            table.insert(une_entrée.path(), EntryNode::Liste(list_child));
                            println!("repertoire");

                       } else {
                            let taille = meta.len(); //récupère la taille de meta
                            let s = Size::new(taille); //on convertit la taille(meta.len) en Size
                            table.insert(une_entrée.path(), EntryNode::Taille(s));
                            println!("fichier");
                       }
                        
                    } 
                }
            }

        } else {
            // Afficher le message d'erreur détaillé grâce à err si la lecture du répertoire échoue
            if let Err(err) = fs::read_dir(root) {
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

        return &self.root;

    }

    pub fn get_children(&self/*, path: &Path */) -> Vec<PathBuf> { //normalement return Option<&[PathBuf]> mais pour l'instant Vec<PathBuf>

        //Verifier si on a bien un répertoir et non un fichier 
        if let Ok(_exist_dir) = fs::metadata(&self.root) {

        } else {
            panic!("Le répertoire n'existe pas, ou ce n'est pas un répertoire ou il y a une erreur lors de l'accès.");
        }

        let mut vec_path: Vec<PathBuf> = Vec::new();
        //Recuperer dans un vecteur les chemins des enfants du répertoire
        for (path, _entry) in self.map.iter() {
            vec_path.push(path.clone());
        }

        return vec_path;
    }

    pub fn get_size(&self, path: &Path) -> Option<Size> {
        unimplemented!()
    }

    pub fn files(&self) -> &[PathBuf] { //signature peut être unvec,  Vec<PathBuf>
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
        let chemin = std::path::Path::new("src/test");

        if let Ok(current_dir) = env::current_dir() {
            println!("Répertoire de travail actuel : {:?}", current_dir);
        } else {
            eprintln!("Erreur lors de l'obtention du répertoire de travail actuel");
        }


        if let Ok(exist_dir) = fs::metadata(chemin) {
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
        let chemin = std::path::Path::new("src/test");
        let tree = FileTree::new(chemin);

        //Verifie juste par l'affichage que l'arbre corresponde bien à ce qu'on a dans le répertoire test
        if let Ok(tree) = tree {
            println!("FileTree: {:?} ", tree);
            println!("Map: {:?} \n", tree.map);
            for (path, entry) in tree.map {
                println!("Path: {:?}, Entry : {:?} ", path, entry);
            }
            println!("\n");
        }
    }

    #[test]
    fn test_get_root() {
        let chemin = std::path::Path::new("src/test");
        let tree = FileTree::new(chemin);
        
        if let Ok(tree) = tree {
            println!("get_root() = {:?}", tree.get_root());
        }
    }

    #[test]
    fn test_get_children() {
        let chemin = std::path::Path::new("src/test");
        let tree = FileTree::new(chemin);
        
        if let Ok(tree) = tree {
            println!("get_children() = {:?}", tree.get_children());
        }
    }
    
}
