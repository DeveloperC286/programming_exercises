use insta::assert_debug_snapshot;

use super::*;

#[test]
fn test_empty_list() {
    // Given
    let list = None;

    // When
    let actual = reverse_list(list);

    // Then
    assert_debug_snapshot!("test_empty_list", actual);
}

#[test]
fn test_single_node_list() {
    // Given
    let list = Some(Box::new(ListNode { val: 1, next: None }));

    // When
    let actual = reverse_list(list);

    // Then
    assert_debug_snapshot!("test_single_node_list", actual);
}

#[test]
fn test_odd_length_list() {
    // Given
    let list = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    }));

    // When
    let actual = reverse_list(list);

    // Then
    assert_debug_snapshot!("test_odd_length_list", actual);
}
