use super::*;

#[test]
fn test_whole_tree_is_valid_bst() {
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
    let expected = 6;

    // When
    let actual = max_sum_bst(tree);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_whole_left_tree_is_valid_bst() {
    // Given
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),

            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        }))),
    })));
    let expected = 7;

    // When
    let actual = max_sum_bst(tree);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_sub_tree_is_valid_bst() {
    // Given
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: -5,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: -3,
                            left: None,
                            right: None,
                        }))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 10,
                            left: None,
                            right: None,
                        }))),
                    }))),
                }))),
            }))),
            right: None,
        }))),
        right: None,
    })));
    let expected = 14;

    // When
    let actual = max_sum_bst(tree);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_sub_tree_is_valid_bst_2() {
    // Given
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 9,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: -5,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 1,
                            left: None,
                            right: None,
                        }))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 10,
                            left: None,
                            right: None,
                        }))),
                    }))),
                }))),
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        }))),
    })));
    let expected = 14;

    // When
    let actual = max_sum_bst(tree);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_valid_bst_tree_with_negatives() {
    // Given
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: -5,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: None,
                right: None,
            }))),
        }))),
    })));
    let expected = 25;

    // When
    let actual = max_sum_bst(tree);

    // Then
    assert_eq!(actual, expected);
}
