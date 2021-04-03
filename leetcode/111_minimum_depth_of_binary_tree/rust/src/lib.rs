use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(root_node) = root {
        let mut to_search: VecDeque<(Rc<RefCell<TreeNode>>, i32)> = VecDeque::new();
        // Initial root node is at depth 1.
        to_search.push_back((root_node, 1));

        while !to_search.is_empty() {
            let (node_searching_rc, depth) = to_search.pop_front().unwrap();
            let node_searching = node_searching_rc.borrow();

            if node_searching.left.is_none() && node_searching.right.is_none() {
                // At a leaf node, and as we are using BFS we know this is the min depth.
                return depth;
            }

            if node_searching.left.is_some() {
                to_search.push_back((node_searching.left.clone().unwrap(), depth + 1));
            }

            if node_searching.right.is_some() {
                to_search.push_back((node_searching.right.clone().unwrap(), depth + 1));
            }
        }
    }

    0
}

#[cfg(test)]
mod tests;
