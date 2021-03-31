use super::*;

#[test]
fn test_multiple_nodes_with_left_and_right_tree() {
    // Given
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
    })));
    let expected = 3;

    // When
    let actual = diameter_of_binary_tree(tree);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_single_node_with_left_child_tree() {
    // Given
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        }))),
        right: None,
    })));
    let expected = 1;

    // When
    let actual = diameter_of_binary_tree(tree);
    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_large_unbalanced_tree() {
    // Given
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: -7,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: -3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: -9,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 6,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 0,
                            left: None,
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: -1,
                                left: None,
                                right: None,
                            }))),
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 6,
                            left: Some(Rc::new(RefCell::new(TreeNode {
                                val: -4,
                                left: None,
                                right: None,
                            }))),
                            right: None,
                        }))),
                    }))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: -7,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: -6,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 5,
                            left: None,
                            right: None,
                        }))),
                        right: None,
                    }))),

                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: -6,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 9,
                            left: None,
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: -2,
                                left: None,
                                right: None,
                            }))),
                        }))),
                        right: None,
                    }))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: -4,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        }))),
    })));
    let expected = 8;

    // When
    let actual = diameter_of_binary_tree(tree);

    // Then
    assert_eq!(actual, expected);
}
