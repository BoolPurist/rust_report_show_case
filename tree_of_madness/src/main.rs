use tree_of_madness::build_tree;
use tree_of_madness::tree::Tree;
fn main() {
    let mut tree = build_tree![100, 25, 10, 30, 40];

    let has_deleted = tree.delete(&25);
    dbg!(tree.iter_shared().next());
    //     100
    //    25
    //  10 30
    //        40
    // assert!(has_deleted);
    // for x in tree.iter_shared() {
    //     dbg!(x);
    // }
}
