/// Simple binary search tree
///
/// For every node of value `v`, all elements in the left sub-tree are smaller
/// than `v` and all elements in the right sub-tree are larger than `v`.
#[derive(Debug, PartialEq)]
pub struct Tree(Option<Box<Node>>);

/// Internal Node representation with a `value` and the left and right sub-trees.
#[derive(Debug, PartialEq)]
pub struct Node {
    value: i32,
    left: Tree,
    right: Tree
}

impl Tree {
    /// Returns an empty tree
    pub fn new() -> Self {
        Tree(None)
    }

   /// Returns a tree containing a single value
    pub fn leaf(value: i32) -> Self {

        Tree(Some(Box::new(Node {
            value,
            left: Tree(None),
            right: Tree(None),})))
    }

    /// Inserts `value` into the tree.
    /// Returns `false` if the `value` was already contained in the tree.
    pub fn insert(&mut self, value: i32) -> bool {

        match &mut self.0 {
            Some(noeud) => {
                if noeud.value == value {
                    return false;
                    
                } else if noeud.value > value {
                    // La valeur doit être insérée dans le sous-arbre gauche.
                    println!("La valeur {} est inférieur à {} donc elle va à gauche", value, noeud.value);
                    noeud.left.insert(value);

                } else if noeud.value < value {
                    // La valeur doit être insérée dans le sous-arbre droit.
                    println!("La valeur {} est supérieur à {} donc elle va à droite", value, noeud.value);
                    noeud.right.insert(value);
                }
            }
            None => {
                // Si la racine est vide, nous la remplaçons par une feuille.
                *self = Tree::leaf(value);
                return true;
            }
        }
        return true;
    }

    // Returns true if and only if `value` belongs to the tree.
    pub fn contains(&self, value: i32) -> bool {

        match &self.0 {
            Some(noeud) => {
                if noeud.value == value {
                    return true;
                } else if noeud.value > value {
                    // La valeur doit être recherchée dans le sous-arbre gauche.
                    return noeud.left.contains(value);
                } else {
                    // La valeur doit être recherchée dans le sous-arbre droit.
                    return noeud.right.contains(value);
                }
            }
            None => {
                // Si la racine est vide, la valeur n'existe pas dans l'arbre.
                return false;
            }
        }
    }

    /// Deletes `value` from the tree.
    /// When the value is not found the tree, `false` is returned.
    pub fn delete(&mut self, value: i32) {
        
        let mut replacement = Tree(None);

        match &mut self.0 {
            Some(noeud) => {
                if noeud.value == value {
                    if noeud.left.0.is_none() && noeud.right.0.is_none() {
                        

                    }if noeud.left.0.is_some() {
                        

                    }if noeud.right.0.is_some() {
                       

                    }

                } else if noeud.value > value {
                    // La valeur doit être recherchée dans le sous-arbre gauche.
                    noeud.left.delete(value);
                    
                } else {
                    // La valeur doit être recherchée dans le sous-arbre droit.
                    noeud.right.delete(value);
                }
            }
            None => {
                // Si la racine est vide, la valeur n'existe pas dans l'arbre on ne fait rien.
            }
        }

        /*if self.0.is_some() {

            if self.0.as_ref().unwrap().left.0.is_some() && self.0.as_ref().unwrap().right.0.is_some() {
                

            } if self.0.as_ref().unwrap().left.0.is_some() {
                let val = self.0.as_ref().unwrap().left.as_ref().unwrap().value;
                

            } if self.0.as_ref().unwrap().right.0.is_some() {
                *self = Tree(Some(Box::new(Node {
                            value,
                            left: Tree(None),
                            right: Tree(None),})));

            }
        }*/
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty(){
        assert_eq!(Tree::new(), Tree(None));
    }

    #[test]
    fn test_single(){
        let tree = Tree(Some(Box::new(Node {
            value: 9,
            left: Tree(None),
            right: Tree(None),})));

        assert_eq!(tree, Tree::leaf(9));
    }
}