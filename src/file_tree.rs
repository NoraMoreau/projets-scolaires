use crate::size::Size;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::exit;
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

        if let Ok(entrées) = fs::read_dir(root) { //on récupère les enfants du répertoire dans entrées grâce à read_dir  
            for x in entrées {
                if let Ok(une_entrée) = x { //desencapsuler x: Result<DirEntry, Error>

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

                       } else {
                            let taille = meta.len(); //récupère la taille de meta
                            let s = Size::new(taille); //on convertit la taille(meta.len) en Size
                            table.insert(une_entrée.path(), EntryNode::Taille(s));
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

    pub fn get_children(&self, path: &Path) -> Vec<PathBuf> { 
        let mut vec_path: Vec<PathBuf> = Vec::new();

        //Verifie que path est bien un répertoire se trouvant dans self
        if path.starts_with(&self.root) == true {
            if let Ok(meta) = fs::metadata(path) {
                if meta.is_dir() {

                } else {
                    eprintln!("Erreur sur le chemin {:?}, ce chemin n'existe pas ou il y a une erreur d'accés", path);
                    exit(3);
                }
            } else {
                eprintln!("Erreur sur le chemin {:?}, ce chemin n'existe pas ou il y a une erreur d'accés", path);
                exit(2);
            }

        } else {
            eprintln!("Erreur sur le chemin {:?}, ce chemin n'existe pas ou il y a une erreur d'accés", path);
            exit(1);
        }

        //Verifier si on a bien un répertoire et non un fichier 
        if let Ok(meta) = fs::metadata(&path) {
            if meta.is_dir() { 
            //Recuperer dans un vecteur les chemins des enfants d'un chemin path
                if let Ok(children) = fs::read_dir(path) {
                    for x in children {
                        if let Ok(child) = x {
                            vec_path.push(child.path());
                        }
                    }
                }
            } else {
                eprintln!("C'est un fichier, la fonction nécessite un répertoire.");
            }

        } else {
            eprintln!("Le répertoire n'existe pas ou il y a une erreur lors de l'accès.");
        }

        return vec_path;
    }

    pub fn get_size(&self, path: &Path) -> Option<Size> {
        //Verifie que path est bien un répertoire ou un fichier se trouvant dans self
        if path.starts_with(&self.root) == true {
            if let Ok(meta) = fs::metadata(path) {
                if meta.is_dir() || meta.is_file() {
                    
                } else {
                    eprintln!("Erreur sur le chemin {:?}, ce chemin n'existe pas ou il y a une erreur d'accés", path);
                    exit(3);
                }
            } else {
                eprintln!("Erreur sur le chemin {:?}, ce chemin n'existe pas ou il y a une erreur d'accés", path);
                exit(2);
            }

        } else {
            eprintln!("Erreur sur le chemin {:?}, ce chemin n'existe pas ou il y a une erreur d'accés", path);
            exit(1);
        }

        if let Ok(meta) = fs::metadata(path) {
            //Si on a un répertoire on iter la fonction ppur parcourir tout ce qui s'y trouve
            //et récupérer leur taille en les sommant au fur et à mesure
            if meta.is_dir() {
                if let Ok(dir) = fs::read_dir(path) {
                    let mut taille = Size::new(0);
                    //On entre dans le répertoire et on parcourt les fichiers que l'on envoit à get_size
                    for x in dir {
                        if let Ok(entrée) = x {
                            let s = self.get_size(&entrée.path());
                            if let Some(taille_iter) = s {
                                taille = taille + taille_iter;
                            }
                        }
                    }
                    return Some(taille);
                }
            
            } else {
                let taille = meta.len(); 
                let s = Size::new(taille); 
                return Some(s);
            }
            
        } else {
            eprintln!("Le fichier n'existe pas ou il y a une erreur lors de l'accès.");
        }

        return None;
    }

    pub fn files(&self) -> Vec<PathBuf> { 
        let mut vec: Vec<PathBuf> = Vec::new();
        //On parcourt les entrées du répertoire et lorsqu'on a des fichiers on les mets dans le vecteur
        if let Ok(entrées) = fs::read_dir(&self.root) {
            for x in entrées {
                if let Ok(une_entrée) = x {
                    if let Ok(meta) = fs::metadata(une_entrée.path()) {
                        if meta.is_file() {
                            vec.push(une_entrée.path());
                        }
                    }
                }
            }
        }

        return vec;
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_read_dir() {
        let chemin = fs::read_dir("src/test");
        println!("Chemin: {:?}", chemin);
        assert!(chemin.is_ok());
    }

    #[test]
    fn test_path() {
        let chemin = std::path::Path::new("src/test/sous_dossier");
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
            assert_eq!(tree.get_root(), chemin);
        }
    }

    #[test]
    fn test_get_children() {
        let chemin = std::path::Path::new("src/test");
        let tree = FileTree::new(chemin);
        let chemin2 = std::path::Path::new("src/test/sous_dossier" ); //"src/test/azerty" "src/test2/tsur" "src/test/sous_dossier"
        
        if let Ok(tree) = tree {
            println!("get_children() = {:?}", tree.get_children(chemin2));
        }
    }

    #[test]
    fn test_get_size() {
        let chemin = std::path::Path::new("src/test");
        let tree = FileTree::new(chemin);
        let chemin2 = std::path::Path::new("src/test/sous_dossier"); //"src/test/azerty" "src/test2/tsur" "src/test/test_edt"
        
        if let Ok(tree) = tree {
            if let Some(taille) = tree.get_size(chemin2) {
                println!("get_size() = {}", taille);
            }
        }
    }

    #[test]
    fn test_files() { 
        let chemin = std::path::Path::new("src/test");
        let tree = FileTree::new(chemin);
        
        if let Ok(tree) = tree {
            println!("files() = {:?}", tree.files());
        }
    }
}
