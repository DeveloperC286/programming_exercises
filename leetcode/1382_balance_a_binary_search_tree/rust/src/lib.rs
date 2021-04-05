use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn add_node_to_sorted_vec(node: Option<Rc<RefCell<TreeNode>>>, sorted_vec: &mut Vec<i32>) {
        if let Some(node_rc) = node {
            let node = node_rc.borrow();
            add_node_to_sorted_vec(node.left.clone(), sorted_vec);
            sorted_vec.push(node.val);
            add_node_to_sorted_vec(node.right.clone(), sorted_vec);
        }
    }

    fn create_tree_from_sorted_vec(sorted_vec: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if sorted_vec.is_empty() {
            return None;
        }

        if sorted_vec.len() == 1 {
            return Some(Rc::new(RefCell::new(TreeNode {
                val: sorted_vec[0],
                left: None,
                right: None,
            })));
        }

        let middle = (sorted_vec.len() - 1) / 2;
        Some(Rc::new(RefCell::new(TreeNode {
            val: sorted_vec[middle],
            left: create_tree_from_sorted_vec(&sorted_vec[..middle]),
            right: create_tree_from_sorted_vec(&sorted_vec[middle + 1..]),
        })))
    }

    let mut sorted_vec = vec![];
    add_node_to_sorted_vec(root, &mut sorted_vec);
    create_tree_from_sorted_vec(&sorted_vec)
}

#[cfg(test)]
mod tests;
