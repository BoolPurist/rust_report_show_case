pub mod iteration;

use crate::node::{DiretionFromParent, Node, RootNode};
use std::cmp::Ordering;
use std::fmt::Debug;
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
                        let left_child = current_node.borrow().get_left_child_shared();
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
                        let right_child = current_node.borrow().get_right_child_shared();
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
                    (true, true) => {
                        let left_detached = Node::take_left_child(&gone_with_it)
                            .expect("Should have a left child at this point");
                        let right_detached = Node::take_right_child(&gone_with_it)
                            .expect("Should have a right child at this point");

                        let largest_node = Node::extract_greatest_node_from(&left_detached);

                        Node::replace_left_child_with(&largest_node, left_detached);
                        Node::replace_right_child_with(&largest_node, right_detached);

                        Node::let_parent_replace_child_with(gone_with_it, largest_node);
                    }
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
impl<T: Ord> Tree<T> {
    fn get_root_node(&self) -> RootNode<T> {
        Rc::clone(
            &self
                .root
                .as_ref()
                .expect("No root found to return for test."),
        )
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

    #[test]
    fn should_find_no_greatest_left_node() {
        let tree = build_tree![100];
        let root = tree.get_root_node();

        let actual_node_found = Node::find_greatest_node_from(&root);

        assert!(actual_node_found.is_none());
    }

    #[test]
    fn should_find_greatest_from_left_node() {
        let tree = build_tree![100, 25, 50, 200, 400];
        let root = tree.get_root_node();

        assert_greatest_node_subtree(&root, 400);
        assert_greatest_node_subtree(&root.borrow().get_left_child_shared().unwrap(), 50);
        assert_greatest_node_subtree(&root.borrow().get_right_child_shared().unwrap(), 400);
    }

    fn assert_greatest_node_subtree(subroot: &RootNode<i32>, expected_value: i32) {
        let actual_node_found = Node::find_greatest_node_from(&subroot)
            .expect("No greatest node from left was returned.");

        assert_eq!(&expected_value, actual_node_found.borrow().get_value_ref());
    }

    #[test]
    fn should_remove_node_with_left_right_children() {
        //      100
        //    25
        //  10   30
        // 5
        let mut tree = build_tree![100, 25, 10, 30];

        let has_deleted = tree.delete(&25);

        //     100
        //    30
        //  10
        // 5
        assert!(has_deleted);

        let actual_nodes: Vec<_> = tree.iter_shared().collect();
        // let expected_nodes = vec![100, 30, 10, 5];
        // assert_eq!(expected_nodes, actual_nodes);
    }
}
