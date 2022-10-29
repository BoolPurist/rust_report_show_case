use super::{RootNode, Tree};
use std::{collections::VecDeque, rc::Rc};
pub struct IterShared<T> {
    pub(super) nodes: VecDeque<RootNode<T>>,
}

impl<T> Tree<T> {
    pub fn iter_shared(&self) -> IterShared<T> {
        let mut deque: VecDeque<_> = VecDeque::new();

        if let Some(root) = self.root.as_ref() {
            deque.push_back(Rc::clone(root));
        };

        IterShared { nodes: deque }
    }
}

impl<T> Iterator for IterShared<T> {
    type Item = Rc<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.nodes.pop_front() {
            let next_borrow = next.borrow();
            if let Some(left) = next_borrow.get_left_child_shared() {
                self.nodes.push_back(left);
            };

            if let Some(right) = next_borrow.get_right_child_shared() {
                self.nodes.push_back(right);
            };

            return Some(next_borrow.get_shared_value());
        }

        None
    }
}
