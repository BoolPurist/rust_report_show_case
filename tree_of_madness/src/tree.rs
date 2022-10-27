pub mod iteration;

use crate::node::{DiretionFromParent, Node, RootNode};
use std::cmp::Ordering;
use std::rc::Rc;
#[derive(Debug)]
pub struct Tree<T> {
    root: Option<RootNode<T>>,
}
enum SearchResult<T> {
    TreeEmpty,
    Found(RootNode<T>),
    ClosestToValue(RootNode<T>, DiretionFromParent),
}
#[macro_export]
macro_rules! build_tree {
    ($($v:expr),*) => {{
        let mut _tree = Tree::new();
        $(_tree.add($v);)*
        _tree
    }};
}

impl<T: Ord> Tree<T> {
    pub fn new() -> Self {
        Tree { root: None }
    }

    fn find_value_from(root: &Option<RootNode<T>>, wanted_value: &T) -> SearchResult<T> {
        if let Some(root) = root.as_ref() {
            let mut current_node = Rc::clone(root);
            loop {
                let ordering = wanted_value.cmp(current_node.borrow().get_value_ref());
                match ordering {
                    Ordering::Equal => {
                        return SearchResult::Found(Rc::clone(&current_node));
                    }
                    Ordering::Less => {
                        let left_child = current_node.borrow().get_left_child();
                        if let Some(new_current_node_child) = left_child {
                            current_node = new_current_node_child;
                        } else {
                            return SearchResult::ClosestToValue(
                                Rc::clone(&current_node),
                                DiretionFromParent::Left,
                            );
                        }
                    }
                    Ordering::Greater => {
                        let right_child = current_node.borrow().get_right_child();
                        if let Some(new_right_child) = right_child {
                            current_node = new_right_child;
                        } else {
                            return SearchResult::ClosestToValue(
                                Rc::clone(&current_node),
                                DiretionFromParent::Right,
                            );
                        }
                    }
                }
            }
        } else {
            return SearchResult::TreeEmpty;
        }
    }

    pub fn add(&mut self, new_value: T) -> bool {
        match Self::find_value_from(&self.root, &new_value) {
            SearchResult::TreeEmpty => {
                self.root = Some(Node::new(new_value));
                true
            }
            SearchResult::Found(_) => false,
            SearchResult::ClosestToValue(attach_to, direction) => {
                match direction {
                    DiretionFromParent::Left => Node::spawn_left_child(&attach_to, new_value),
                    DiretionFromParent::Right => Node::spawn_right_child(&attach_to, new_value),
                    DiretionFromParent::NoParent => panic!("Can not add value to tree.\nReason: missing side(left, right) where to insert new value."),
                };

                true
            }
        }
    }

    /// Returns true if given value is in the tree, otherwiese returns false.
    /// # Example
    /// ```
    /// use tree_of_madness::build_tree;
    /// use tree_of_madness::tree::Tree;
    ///
    /// let tree = build_tree![10, 3, 4, 8, 6, 16];
    /// assert!(tree.contains(&10));
    /// assert!(!tree.contains(&9));
    /// ```
    pub fn contains(&self, searched: &T) -> bool {
        match Self::find_value_from(&self.root, searched) {
            SearchResult::TreeEmpty | SearchResult::ClosestToValue(..) => false,
            SearchResult::Found(_) => true,
        }
    }

    pub fn delete(&mut self, to_delete: &T) -> bool {
        return match Self::find_value_from(&self.root, to_delete) {
            SearchResult::TreeEmpty | SearchResult::ClosestToValue(..) => false,
            SearchResult::Found(gone_with_it) => {
                let left_right = gone_with_it.borrow().left_right_taken();
                match left_right {
                    (false, false) => Node::take_child_from_parent(&gone_with_it),
                    (false, true) => {
                        let new_right_child = Node::take_right_child(&gone_with_it)
                            .expect("Here it is known that there is a right child.");

                        replace_found_with_taken_child(self, gone_with_it, new_right_child);
                    }
                    (true, false) => {
                        let new_left_child = Node::take_left_child(&gone_with_it)
                            .expect("Here it is known that there is a left child.");

                        replace_found_with_taken_child(self, gone_with_it, new_left_child);
                    }
                    (true, true) => unimplemented!(),
                };

                true
            }
        };

        fn replace_found_with_taken_child<T>(
            tree: &mut Tree<T>,
            gone_with_it: RootNode<T>,
            new_child: RootNode<T>,
        ) {
            let changed_parent =
                Node::let_parent_replace_child_with(gone_with_it, Rc::clone(&new_child));
            // There is no parent for the child of the delteted node. In this case the deleted node is the
            // root of the tree.
            if let None = changed_parent {
                tree.root = Some(new_child);
            }
        }
    }
}

#[cfg(test)]
mod testing {
    use super::*;
    #[test]
    fn should_add_and_figure_what_added() {
        let tree = build_tree![10, 3, 4, 10, 8, 6, 16];

        assert!(tree.contains(&10));
        assert!(tree.contains(&3));
        assert!(tree.contains(&4));
        assert!(tree.contains(&8));
        assert!(tree.contains(&6));
        assert!(tree.contains(&16));

        assert!(!tree.contains(&11));
        assert!(!tree.contains(&-10));
        assert!(!tree.contains(&0));
    }

    #[test]
    fn should_delete_leafs() {
        //      35
        //   10    16
        //  4
        // 6  8
        let mut tree = build_tree![35, 10, 4, 8, 6, 16];

        assert!(tree.delete(&6));
        assert!(tree.delete(&8));

        assert!(!tree.delete(&6));
        assert!(!tree.delete(&-8));

        assert!(tree.delete(&16));

        assert!(!tree.contains(&6));
        assert!(!tree.contains(&8));
        assert!(!tree.contains(&16));

        assert!(tree.contains(&35));
        assert!(tree.contains(&10));
        assert!(tree.contains(&4));
    }

    #[test]
    fn should_delete_without_losing_one_child() {
        //      35
        //   10    46
        //  4     38
        //   8
        let mut tree = build_tree![35, 10, 4, 8, 46, 38];

        assert!(tree.delete(&46));
        assert!(tree.delete(&4));

        assert!(!tree.delete(&46), "46 was already deleted");
        assert!(!tree.delete(&4), "4 was already deleted");

        assert!(!tree.contains(&46));
        assert!(!tree.contains(&4));

        assert!(tree.contains(&8));
        assert!(tree.contains(&38));

        assert!(tree.contains(&35));
        assert!(tree.contains(&10));
        //      35
        //    10  38
        //   8
    }
    #[test]
    fn should_delete_root_with_one_left_child() {
        //      35
        //   10
        //  8  23
        let mut tree = build_tree![35, 10, 8, 23];

        assert!(tree.delete(&35));

        assert!(!tree.delete(&35), "35 was already deleted");

        assert!(!tree.contains(&35));

        assert!(tree.contains(&8));
        assert!(tree.contains(&23));
        assert!(tree.contains(&10));
        //      10
        //    8   23
    }
    #[test]
    fn should_delete_root_with_one_right_child() {
        //      35
        //            80
        //          40  83
        let mut tree = build_tree![35, 80, 40, 83];

        assert!(tree.delete(&35));

        assert!(!tree.delete(&35), "35 was already deleted");

        assert!(!tree.contains(&35));

        assert!(tree.contains(&80));
        assert!(tree.contains(&40));
        assert!(tree.contains(&83));
        //            80
        //          40  83
    }
}
