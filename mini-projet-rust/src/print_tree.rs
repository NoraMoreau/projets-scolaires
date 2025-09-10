use std::path::PathBuf;
use crate::file_tree::FileTree;
use crate::size::Size;

impl FileTree {
    pub fn show(&self, lexico: bool, filter: Option<String>) {

        //Trie dans l'ordre croissant la taille des children
        if lexico == false && filter == None {
            let racine = self.get_root();
            let taille_racine = self.get_size(racine);
            if let Some(taille) = taille_racine {
                println!("{}    {:?}", taille, racine);
            }
            let mut children_self = self.get_children(&racine); 
            
            children_self.sort_by(|a, b| 
                {
                    let taille_a = self.get_size(a);
                    let taille_b = self.get_size(b);
                    taille_a.cmp(&taille_b)
                });
            for x in &children_self {
                let taille_children = self.get_size(x);
                if let Some(taille) = taille_children {
                    println!("      {}    {:?}", taille, x);
                }
            }

        //Recherche les fichiers contenant un motif spécifique
        } else if filter != None && lexico == false {
            let racine = self.get_root();
            let mut children_self = self.get_children(&racine); 
            if let Some(pattern) = &filter {
                println!("Résultat des chemins contenant {:?} :", pattern);
                for x in &children_self {
                    let s = x.to_str();
                    let taille_children = self.get_size(x);
                    if let Some(s) = s {
                        if s.contains(pattern) {
                            if let Some(taille) = taille_children {
                                println!("      {}    {:?}", taille, s);
                            }
                        }
                        
                    }
                }
               
            }

        //Tri dans l'ordre croissant lexicographique les path des children
        } else if lexico == true && filter == None {
            let racine = self.get_root();
            let taille_racine = self.get_size(racine);
            if let Some(taille) = taille_racine {
                println!("{}    {:?}", taille, racine);
            }
            let mut children_self = self.get_children(&racine); 
            
            children_self.sort_by(|a, b| 
                {
                    a.cmp(&b)
                });
            for x in &children_self {
                let taille_children = self.get_size(x);
                if let Some(taille) = taille_children {
                    println!("      {}    {:?}", taille, x);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_tri_taille() {
        let chemin = std::path::Path::new("src/test/sous_dossier");
        let tree = FileTree::new(chemin);
        if let Ok(tree) = tree {
            tree.show(false, None);
        }
    }

    #[test]
    fn test_show_tri_lexico() {
        let chemin = std::path::Path::new("src/test/sous_dossier");
        let tree = FileTree::new(chemin);
        if let Ok(tree) = tree {
            tree.show(true, None);
        }
    }

    #[test]
    fn test_show_filter() {
        let chemin = std::path::Path::new("src/test/sous_dossier");
        let tree = FileTree::new(chemin);
        let f = Some("ert".to_string());
        if let Ok(tree) = tree {
            tree.show(false, f);
        }
    }
}
