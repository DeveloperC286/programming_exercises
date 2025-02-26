use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut depths = vec![0];

    if let Some(root_node) = root {
        // Initial root node is at depth 1.
        let mut to_search: Vec<(Rc<RefCell<TreeNode>>, i32)> = vec![(root_node, 1)];

        while let Some((node_searching_rc, depth)) = to_search.pop() {
            let node_searching = node_searching_rc.borrow();

            if node_searching.left.is_none() && node_searching.right.is_none() {
                // At a leaf node so add this depth to the list of depths.
                depths.push(depth);
            }

            if node_searching.left.is_some() {
                to_search.push((node_searching.left.clone().unwrap(), depth + 1));
            }

            if node_searching.right.is_some() {
                to_search.push((node_searching.right.clone().unwrap(), depth + 1));
            }
        }
    }

    *depths.iter().max().unwrap()
}

#[cfg(test)]
mod tests;
