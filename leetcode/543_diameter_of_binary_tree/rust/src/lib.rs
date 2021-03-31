use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let (_, max_diameter) = get_max_depth_and_diameter(root, 0);
    max_diameter
}

fn get_max_depth_and_diameter(
    node: Option<Rc<RefCell<TreeNode>>>,
    current_depth: i32,
) -> (i32, i32) {
    if let Some(node) = node {
        let inner_node = node.borrow();

        let (left_depth, left_diameter) =
            get_max_depth_and_diameter(inner_node.left.clone(), current_depth + 1);
        let (right_depth, right_diameter) =
            get_max_depth_and_diameter(inner_node.right.clone(), current_depth + 1);
        let new_diameter = (left_depth - current_depth) + (right_depth - current_depth) - 2;

        return (
            max(left_depth, right_depth),
            max(max(left_diameter, right_diameter), new_diameter),
        );
    }

    (current_depth, 0)
}

#[cfg(test)]
mod tests;
