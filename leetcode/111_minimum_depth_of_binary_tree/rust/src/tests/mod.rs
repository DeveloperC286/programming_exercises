use super::*;

#[test]
fn test_three_height_tree() {
    // Given
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
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
    let expected = 2;

    // When
    let actual = min_depth(tree);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_long_chained_single_linked_tree() {
    // Given
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 6,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: None,
            }))),
            right: None,
        }))),
        right: None,
    })));
    let expected = 5;

    // When
    let actual = min_depth(tree);

    // Then
    assert_eq!(actual, expected);
}
