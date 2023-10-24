/// Simple binary search tree
///
/// For every node of value `v`, all elements in the left sub-tree are smaller
/// than `v` and all elements in the right sub-tree are larger than `v`.
#[derive(Debug)]
pub struct Tree(Option<Box<Node>>);

/// Internal Node representation with a `value` and the left and right sub-trees.
#[derive(Debug)]
struct Node {
    value: i32,
    left: Tree,
    right: Tree,
}

impl Tree {
    /// Returns an empty tree
    pub fn new() -> Self {
        panic!("Not implemented")
    }

    /// Returns a tree containing a single value
    fn leaf(value: i32) -> Self {
        panic!("Not implemented")
    }

    /// Inserts `value` into the tree.
    /// Returns `false` iff the `value` was already contained in the tree.
    pub fn insert(&mut self, value: i32) -> bool {
        panic!("Not implemented")
    }

    /// Returns true if and only if `value` belongs to the tree.
    pub fn contains(&self, value: i32) -> bool {
        panic!("Not implemented")
    }

    /// Deletes `value` from the tree.
    /// When the value is not found the tree, `false` is returned.
    pub fn delete(&mut self, value: i32) {
        panic!("Not implemented");
    }
}
