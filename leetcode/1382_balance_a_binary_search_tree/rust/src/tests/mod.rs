use insta::assert_debug_snapshot;

use super::*;

#[test]
fn test_empty_tree() {
    // Given
    let tree = None;

    // When
    let actual = balance_bst(tree);

    // Then
    assert_debug_snapshot!("test_empty_tree", actual);
}

#[test]
fn test_right_only_tree() {
    // Given
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
        }))),
    })));

    // When
    let actual = balance_bst(tree);

    // Then
    assert_debug_snapshot!("test_right_only_tree", actual);
}
