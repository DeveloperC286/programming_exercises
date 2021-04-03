use super::*;

#[test]
fn test_empty_tree() {
    // Given
    let tree = None;
    let expected = 0;

    // When
    let actual = max_depth(tree);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_one_height_tree() {
    // Given
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: None,
        right: None,
    })));
    let expected = 1;

    // When
    let actual = max_depth(tree);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_two_height_tree() {
    // Given
    let tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: None,
    })));
    let expected = 2;

    // When
    let actual = max_depth(tree);

    // Then
    assert_eq!(actual, expected);
}

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
    let expected = 3;

    // When
    let actual = max_depth(tree);

    // Then
    assert_eq!(actual, expected);
}
