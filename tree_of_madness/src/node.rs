//! Many operations on a node are implemented as associated function instead as methods.
//! Reason: A node only gets exposed as a packed  Rc<RefCell<...>> construct to the user.
//! The associated functions borrow of the inner node themself,
//! This eases usage and reduce runtime violation via borrowing on RefCell,
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};
#[derive(Debug, Copy, Clone)]
pub enum DiretionFromParent {
    Left,
    Right,
    NoParent,
}
type ParentNode<T> = Weak<RefCell<Node<T>>>;
pub(crate) type RootNode<T> = Rc<RefCell<Node<T>>>;
#[derive(Debug)]
pub(crate) struct Node<T> {
    parent: Option<ParentNode<T>>,
    value: Rc<T>,
    dir_to_parent: DiretionFromParent,
    left: Option<RootNode<T>>,
    right: Option<RootNode<T>>,
}

impl<T> Node<T> {
    pub fn new(new_value: T) -> RootNode<T> {
        Rc::new(RefCell::new(Node {
            parent: None,
            left: None,
            right: None,
            dir_to_parent: DiretionFromParent::NoParent,
            value: Rc::new(new_value),
        }))
    }

    pub fn get_shared_value(&self) -> Rc<T> {
        Rc::clone(&self.value)
    }

    pub fn get_direction_from_parent(&self) -> DiretionFromParent {
        self.dir_to_parent
    }

    pub fn get_value_ref(&self) -> &T {
        &self.value
    }

    /// Returns left child as shared owned value.
    pub fn get_left_child_shared(&self) -> Option<RootNode<T>> {
        self.left.as_ref().map(|node| Rc::clone(node))
    }

    /// Returns right child as shared owned value.
    pub fn get_right_child_shared(&self) -> Option<RootNode<T>> {
        self.right.as_ref().map(|node| Rc::clone(node))
    }

    /// Creates a new node with the given value and then makes this new node
    /// the left child of the provided node.
    pub fn spawn_left_child(parent: &RootNode<T>, left_value: T) {
        let left_child = Node::new(left_value);
        {
            parent.borrow_mut().left = Some(Rc::clone(&left_child));
        }

        Self::set_parent(parent, &left_child, DiretionFromParent::Left);
    }

    /// Creates a new node with the given value and then makes this new node
    /// the right child of the provided node.
    pub fn spawn_right_child(parent: &RootNode<T>, right_value: T) {
        let right_child = Node::new(right_value);
        {
            parent.borrow_mut().right = Some(right_child.clone());
        }

        Self::set_parent(parent, &right_child, DiretionFromParent::Right);
    }

    /// Returns parent of node. It increments the reference counter to the undelying node.
    /// Returns None if the node has no parent.
    /// In this case the node the root usually.
    pub fn get_parent(child: &RootNode<T>) -> Option<RootNode<T>> {
        child
            .borrow()
            .parent
            .as_ref()
            .and_then(|parent| parent.upgrade())
    }

    pub fn take_child_from_parent(child: &RootNode<T>) {
        let dir_from_parent = child.borrow().get_direction_from_parent();
        let parent = Self::get_parent(&child);
        match dir_from_parent {
            DiretionFromParent::NoParent => (),
            DiretionFromParent::Left => {
                _ = Self::take_left_child(
                    &parent.expect("With left as direction from parent, there must be a parent"),
                );
            }
            DiretionFromParent::Right => {
                _ = Self::take_right_child(
                    &parent.expect("With right as direction from parent, there must be a parent"),
                );
            }
        }
    }

    fn set_parent(parent: &RootNode<T>, child: &RootNode<T>, dir: DiretionFromParent) {
        let weak_to_parent = Some(Rc::downgrade(&parent));
        {
            let mut mut_child = child.borrow_mut();
            mut_child.parent = weak_to_parent;
            mut_child.dir_to_parent = dir;
        }
    }

    fn unset_parent(child: &RootNode<T>) {
        let mut child_mut = child.borrow_mut();
        child_mut.parent = None;
        child_mut.dir_to_parent = DiretionFromParent::NoParent;
    }

    /// Removes left child on given node and returns this child as orphan, with no parent.
    /// If there is no child to be removed then None is returned.
    pub fn take_left_child(parent: &RootNode<T>) -> Option<RootNode<T>> {
        Self::take_child(&mut parent.borrow_mut().left)
    }

    /// Removes right child on given node and returns this child as orphan, with no parent.
    /// If there is no child to be removed then None is returned.
    pub fn take_right_child(parent: &RootNode<T>) -> Option<RootNode<T>> {
        Self::take_child(&mut parent.borrow_mut().right)
    }

    pub fn replace_left_child_with(
        parent: &RootNode<T>,
        new_left_child: RootNode<T>,
    ) -> Option<RootNode<T>> {
        Node::set_parent(parent, &new_left_child, DiretionFromParent::Left);

        let old_left_child = parent.borrow_mut().left.replace(new_left_child);

        if let Some(ref orphan) = old_left_child {
            Self::unset_parent(orphan);
        }

        old_left_child
    }

    pub fn replace_right_child_with(
        parent: &RootNode<T>,
        new_right_child: RootNode<T>,
    ) -> Option<RootNode<T>> {
        Node::set_parent(parent, &new_right_child, DiretionFromParent::Right);

        let old_right_child = parent.borrow_mut().right.replace(new_right_child);

        if let Some(ref orphan) = old_right_child {
            Self::unset_parent(orphan);
        }

        old_right_child
    }

    /// Parent of parameter old_child replaces old_child with the parameter new_child as respective
    /// child.
    /// Returns the parent of the old_child. Returns None if the old_child has no parent.
    ///
    /// Example: if old_child is the left child of another node, parent, then the new_child
    /// will become the new left child of the parent.
    pub fn let_parent_replace_child_with(
        old_child: RootNode<T>,
        new_child: RootNode<T>,
    ) -> Option<RootNode<T>> {
        if let Some(parent) = Self::get_parent(&old_child) {
            let direction = old_child.borrow().get_direction_from_parent();
            match direction {
                DiretionFromParent::Left => Self::replace_left_child_with(&parent, new_child),
                DiretionFromParent::Right => Self::replace_right_child_with(&parent, new_child),
                DiretionFromParent::NoParent => panic!(
                    "At this point, there must be a parent. The direction is missing to connect the child to the parent"
                    ),
            };

            Some(parent)
        } else {
            None
        }
    }

    fn take_child(child_to_take: &mut Option<RootNode<T>>) -> Option<RootNode<T>> {
        if let Some(orphan) = child_to_take.take() {
            {
                Self::unset_parent(&orphan);
            }
            Some(orphan)
        } else {
            None
        }
    }

    /// Returns the node with the largest value from the parameter to_search_from as root.
    /// Returns none if the to_search_from has no children.
    pub fn find_greatest_node_from(to_search_from: &RootNode<T>) -> Option<RootNode<T>> {
        let mut previous_node = None;
        let mut current_largest = to_search_from.borrow().get_right_child_shared();
        while let Some(next_right_node) = current_largest {
            previous_node = Some(Rc::clone(&next_right_node));
            current_largest = next_right_node.borrow().get_right_child_shared();
        }

        previous_node
    }

    /// Searches the node with largest node from the parameter to_search_from as root.
    /// Then if any
    /// Returns none if the parameter to_search_from has no right children
    pub fn extract_greatest_node_from(to_search_from: &RootNode<T>) -> Option<RootNode<T>> {
        let largest_node = Self::find_greatest_node_from(to_search_from)?;

        if let Some(left_child_largest) = Self::take_left_child(&largest_node) {
            _ = Self::let_parent_replace_child_with(Rc::clone(&largest_node), left_child_largest);
        } else {
            _ = Self::take_child_from_parent(&largest_node);
        }

        Some(largest_node)
    }

    pub fn left_right_taken(&self) -> (bool, bool) {
        (self.left.is_some(), self.right.is_some())
    }
}

#[cfg(test)]
mod testing {

    use super::Node;

    #[test]
    fn should_left_add() {
        let root = Node::new(2u32);
        let expected_value = 1u32;
        Node::spawn_left_child(&root, expected_value);

        let actual_left_child = root.borrow().get_left_child_shared();
        let actual_right_child = root.borrow().get_right_child_shared();

        match actual_left_child {
            Some(child) => assert_eq!(&expected_value, child.borrow().get_value_ref()),
            None => panic!("No left child created"),
        }

        match actual_right_child {
            Some(_) => panic!("Should not add left value as right child"),
            None => (),
        }
    }

    #[test]
    fn should_right_add() {
        let root = Node::new(2u32);
        let expected_value = 1u32;
        Node::spawn_right_child(&root, expected_value);

        let actual_right_child = root.borrow().get_right_child_shared();
        let actual_left_child = root.borrow().get_left_child_shared();

        match actual_right_child {
            Some(child) => assert_eq!(&expected_value, child.borrow().get_value_ref()),
            None => panic!("No right child created"),
        }
        match actual_left_child {
            Some(_) => panic!("Should not add right value as left child"),
            None => (),
        }
    }

    #[test]
    fn should_add_and_remove_left() {
        let root = Node::new(2u32);
        let expected_value = 0u32;
        Node::spawn_left_child(&root, expected_value);
        let taken_child = Node::take_left_child(&root);

        match taken_child {
            Some(child) => assert_eq!(&expected_value, child.borrow().get_value_ref()),
            None => panic!("left child is not returned as removed node."),
        }

        let borrowed_root = root.borrow();
        match borrowed_root.get_left_child_shared() {
            Some(_) => panic!("Left child was not removed."),
            None => (),
        }
    }
    #[test]
    fn should_add_and_remove_right() {
        let root = Node::new(2u32);
        let expected_value = 0u32;
        Node::spawn_right_child(&root, expected_value);
        let taken_child = Node::take_right_child(&root);

        match taken_child {
            Some(child) => assert_eq!(&expected_value, child.borrow().get_value_ref()),
            None => panic!("right child is not returned as removed node."),
        }

        let borrowed_root = root.borrow();
        match borrowed_root.get_left_child_shared() {
            Some(_) => panic!("Right child was not removed."),
            None => (),
        }
    }
}
