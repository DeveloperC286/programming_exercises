use super::*;

#[test]
fn test_singular_generating_from() {
    // Given
    let generating_from = vec![0];

    // When
    let actual = subsets(generating_from);

    // Then
    assert_eq!(actual.len(), 2);
    assert!(actual.contains(&vec![]));
    assert!(actual.contains(&vec![0]));
}

#[test]
fn test_multiple_generating_from() {
    // Given
    let generating_from = vec![1, 2, 3];

    // When
    let actual = subsets(generating_from);

    // Then
    assert_eq!(actual.len(), 8);
    assert!(actual.contains(&vec![]));
    assert!(actual.contains(&vec![1]));
    assert!(actual.contains(&vec![2]));
    assert!(actual.contains(&vec![3]));
    assert!(actual.contains(&vec![1, 2]));
    assert!(actual.contains(&vec![1, 3]));
    assert!(actual.contains(&vec![2, 3]));
    assert!(actual.contains(&vec![1, 2, 3]));
}
