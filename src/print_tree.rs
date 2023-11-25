use crate::file_tree::FileTree;
use crate::size::Size;

impl FileTree {
    pub fn show(&self) {
        let racine = self.get_root(); //récupérer la racine et sa taille
        let taille_racine: Option<Size>; //additionner les tailles des fichiers et sous-rep
        let fichiers_self = self.files();
        let taille_fichiers_self: Option<Size>;
        let repertoires_self = self.get_children(&racine); //récupérer que les rep
        let taille_repertoires_self: Option<Size>; //appeler get-childrene et récupérer size sur les fichier

        for x in repertoires_self {
            //self.get_size(&x);
        }

        unimplemented!()
    }
}
