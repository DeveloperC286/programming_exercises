use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn search_bst(
    searching: Option<Rc<RefCell<TreeNode>>>,
    search_for: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(searching_rc) = searching {
        let searching_node = searching_rc.borrow();

        return match search_for.cmp(&searching_node.val) {
            Ordering::Equal => Some(Rc::clone(&searching_rc)),
            Ordering::Less => search_bst(searching_node.left.clone(), search_for),
            Ordering::Greater => search_bst(searching_node.right.clone(), search_for),
        };
    }

    None
}

#[cfg(test)]
mod tests;
