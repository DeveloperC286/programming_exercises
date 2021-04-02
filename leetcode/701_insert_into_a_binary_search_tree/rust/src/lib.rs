use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn insert_into_bst(
    root: Option<Rc<RefCell<TreeNode>>>,
    inserting: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        return Some(Rc::new(RefCell::new(TreeNode::new(inserting))));
    }

    let mut cursor: Option<Rc<RefCell<TreeNode>>> = root.clone();

    loop {
        // If it was None then we'd have inserted the value.
        let rc = cursor.unwrap();
        let mut inserting_at = rc.borrow_mut();

        if inserting < inserting_at.val {
            if inserting_at.left.is_some() {
                cursor = inserting_at.left.clone();
            } else {
                inserting_at.left = Some(Rc::new(RefCell::new(TreeNode::new(inserting))));
                break;
            }
        } else if inserting_at.right.is_some() {
            cursor = inserting_at.right.clone();
        } else {
            inserting_at.right = Some(Rc::new(RefCell::new(TreeNode::new(inserting))));
            break;
        }
    }

    root
}

#[cfg(test)]
mod tests;
