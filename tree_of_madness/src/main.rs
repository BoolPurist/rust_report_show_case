use tree_of_madness::build_tree;
use tree_of_madness::tree::Tree;
fn main() {
    let mut tree = build_tree![100, 25, 50, 10, 30];
    for x in tree.iter_shared() {
        println!("{:?}", x);
    }

    println!("{}", "=".repeat(10));
    tree.delete(&25);

    for x in tree.iter_shared() {
        println!("{:?}", x);
    }
}
