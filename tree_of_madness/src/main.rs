use tree_of_madness::build_tree;
use tree_of_madness::tree::Tree;
fn main() {
    let tree = build_tree![35, 10, 4, 12, 46, 38, 50];
    //      35
    //  10     46
    // 4  12 38  50
    for x in tree.iter_shared() {
        println!("{x}");
    }
}
