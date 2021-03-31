use super::*;

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
    let actual = is_valid_bst(tree);

    // Then
    assert!(actual);
}

#[test]
fn test_multiple_nodes_with_left_and_right_tree() {
    // Given
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
        }))),
    })));

    // When
    let actual = is_valid_bst(tree);

    // Then
    assert!(!actual);
}

#[test]
fn test_multiple_nodes_with_left_and_right_tree_2() {
    // Given
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 6,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    })));

    // When
    let actual = is_valid_bst(tree);

    // Then
    assert!(!actual);
}

#[test]
fn test_single_node_with_max_value_tree() {
    // Given
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 2147483647,
        left: None,
        right: None,
    })));

    // When
    let actual = is_valid_bst(tree);

    // Then
    assert!(actual);
}

#[test]
fn test_single_node_with_same_left_value_tree() {
    // Given
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        }))),
        right: None,
    })));

    // When
    let actual = is_valid_bst(tree);

    // Then
    assert!(!actual);
}
