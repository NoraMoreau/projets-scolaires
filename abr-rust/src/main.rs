use abr::Tree;
use abr::Node;

fn main() {
    let mut t = Tree::leaf(9);           
    t.insert(12);
    t.insert(5);
    t.insert(7);
    t.delete(12);

    println!("{:#?}", t);
}