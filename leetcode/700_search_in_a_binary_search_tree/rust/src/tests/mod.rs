use super::*;
use insta::assert_debug_snapshot;

#[test]
fn test_empty_tree() {
    // Given
    let tree = None;
    let searching_for = 1;

    // When
    let actual = search_bst(tree, searching_for);

    // Then
    assert_debug_snapshot!("test_empty_tree", actual);
}

#[test]
fn test_multiple_nodes_tree_found() {
    // Given
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: None,
            right: None,
        }))),
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
        }))),
    })));
    let searching_for = 2;

    // When
    let actual = search_bst(tree, searching_for);

    // Then
    assert_debug_snapshot!("test_multiple_nodes_tree_found", actual);
}

#[test]
fn test_multiple_nodes_tree_not_found() {
    // Given
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: None,
            right: None,
        }))),
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
        }))),
    })));
    let searching_for = 5;

    // When
    let actual = search_bst(tree, searching_for);

    // Then
    assert_debug_snapshot!("test_multiple_nodes_tree_not_found", actual);
}
