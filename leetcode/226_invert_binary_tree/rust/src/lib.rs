use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn invert_tree(node: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = node.as_ref() {
        let mut node = node.borrow_mut();

        let inverted_left_tree = invert_tree(node.left.as_ref().map(Rc::clone));
        let inverted_right_tree = invert_tree(node.right.as_ref().map(Rc::clone));

        node.right = inverted_left_tree;
        node.left = inverted_right_tree;
    }

    node
}

#[cfg(test)]
mod tests;
