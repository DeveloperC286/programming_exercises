use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    validate_node(root, None, None)
}

fn validate_node(
    node: Option<Rc<RefCell<TreeNode>>>,
    lower_bound: Option<i32>,
    upper_bound: Option<i32>,
) -> bool {
    if let Some(checking) = node {
        let inner_checking = checking.borrow();

        if let Some(lower_bound) = lower_bound {
            if inner_checking.val <= lower_bound {
                return false;
            }
        }

        if let Some(upper_bound) = upper_bound {
            if inner_checking.val >= upper_bound {
                return false;
            }
        }

        return validate_node(
            inner_checking.right.clone(),
            Some(inner_checking.val),
            upper_bound,
        ) && validate_node(
            inner_checking.left.clone(),
            lower_bound,
            Some(inner_checking.val),
        );
    }

    true
}

#[cfg(test)]
mod tests;
