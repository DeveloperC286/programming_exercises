use super::*;
use insta::assert_debug_snapshot;

#[test]
fn test_empty_tree() {
    // Given
    let tree = None;

    // When
    let actual = insert_into_bst(tree, 1);

    // Then
    assert_debug_snapshot!("test_empty_tree", actual);
}

#[test]
fn test_two_height_tree() {
    // Given
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
    })));

    // When
    let actual = insert_into_bst(tree, 4);

    // Then
    assert_debug_snapshot!("test_two_height_tree", actual);
}

#[test]
fn test_three_height_tree() {
    // Given
    //Input: root = [40,20,60,10,30,50,70], val = 25
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 40,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(10)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(30)))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 60,
            left: Some(Rc::new(RefCell::new(TreeNode::new(50)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(70)))),
        }))),
    })));

    // When
    let actual = insert_into_bst(tree, 25);

    // Then
    assert_debug_snapshot!("test_three_height_tree", actual);
}
