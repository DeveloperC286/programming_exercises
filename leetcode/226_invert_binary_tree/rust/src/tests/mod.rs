use super::*;

use insta::assert_debug_snapshot;

#[test]
fn test_single_node_with_left_and_right_tree() {
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
    let actual = invert_tree(tree);

    // Then
    assert_debug_snapshot!("test_single_node_with_left_and_right_tree", actual);
}

#[test]
fn test_multiple_nodes_with_left_and_right_tree() {
    // Given
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
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
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
        }))),
    })));

    // When
    let actual = invert_tree(tree);

    // Then
    assert_debug_snapshot!("test_multiple_nodes_with_left_and_right_tree", actual);
}
