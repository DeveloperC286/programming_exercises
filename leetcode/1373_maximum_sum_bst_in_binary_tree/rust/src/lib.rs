#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

pub fn max_sum_bst(root_optional: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let (max_sum, tree_sum) = max_sum_bst_internal(root_optional);
    max(max(max_sum.unwrap_or(0), tree_sum.unwrap_or(0)), 0)
}

fn max_sum_bst_internal(
    node_optional: Option<Rc<RefCell<TreeNode>>>,
) -> (Option<i32>, Option<i32>) {
    if let Some(node_rc) = node_optional.clone() {
        let node = node_rc.borrow();

        if node.left.is_none() && node.right.is_none() {
            return (Some(node.val), Some(node.val));
        }

        let (left_max_sum, left_tree_sum) = max_sum_bst_internal(node.left.clone());
        let (right_max_sum, right_tree_sum) = max_sum_bst_internal(node.right.clone());
        let tree_max_sum_bst = match is_binary_search_tree(node_optional, None, None) {
            true => match (left_tree_sum, right_tree_sum) {
                (Some(left_tree_sum), None) => Some(node.val + left_tree_sum),
                (None, Some(right_tree_sum)) => Some(node.val + right_tree_sum),
                (Some(left_tree_sum), Some(right_tree_sum)) => {
                    Some(node.val + left_tree_sum + right_tree_sum)
                }
                (None, None) => Some(node.val),
            },
            false => None,
        };

        return match (tree_max_sum_bst, left_max_sum, right_max_sum) {
            (None, Some(left_max_sum), None) => (Some(left_max_sum), None),
            (None, None, Some(right_max_sum)) => (Some(right_max_sum), None),
            (None, Some(left_max_sum), Some(right_max_sum)) => {
                (Some(max(left_max_sum, right_max_sum)), None)
            }
            (Some(tree_max_sum_bst), Some(left_max_sum), None) => (
                Some(max(left_max_sum, tree_max_sum_bst)),
                Some(tree_max_sum_bst),
            ),
            (Some(tree_max_sum_bst), None, Some(right_max_sum)) => (
                Some(max(right_max_sum, tree_max_sum_bst)),
                Some(tree_max_sum_bst),
            ),
            (Some(tree_max_sum_bst), Some(left_max_sum), Some(right_max_sum)) => (
                Some(max(max(left_max_sum, right_max_sum), tree_max_sum_bst)),
                Some(tree_max_sum_bst),
            ),
            (_, _, _) => unreachable!(),
        };
    }

    (None, None)
}

fn is_binary_search_tree(
    node: Option<Rc<RefCell<TreeNode>>>,
    lower_bound: Option<i32>,
    upper_bound: Option<i32>,
) -> bool {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();

        if let Some(upper_bound_value) = upper_bound {
            if node.val >= upper_bound_value {
                return false;
            }
        }

        if let Some(lower_bound_value) = lower_bound {
            if node.val <= lower_bound_value {
                return false;
            }
        }

        return is_binary_search_tree(node.left.clone(), lower_bound, Some(node.val))
            && is_binary_search_tree(node.right.clone(), Some(node.val), upper_bound);
    }

    true
}

#[cfg(test)]
mod tests;
