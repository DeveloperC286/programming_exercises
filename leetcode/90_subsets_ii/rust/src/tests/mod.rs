use super::*;

#[test]
fn test_singular_generating_from() {
    // Given
    let generating_from = vec![0];
    let expecting = vec![vec![], vec![0]];

    // When
    let actual = subsets_with_dup(generating_from);

    // Then
    assert_eq!(actual, expecting);
}

#[test]
fn test_multiple_generating_from() {
    // Given
    let generating_from = vec![1, 2, 3];
    let expecting = vec![
        vec![],
        vec![1],
        vec![1, 2],
        vec![1, 2, 3],
        vec![1, 3],
        vec![2],
        vec![2, 3],
        vec![3],
    ];

    // When
    let actual = subsets_with_dup(generating_from);

    // Then
    assert_eq!(actual, expecting);
}

#[test]
fn test_multiple_duplicates_generating_from() {
    // Given
    let generating_from = vec![1, 2, 2];
    let expecting = vec![
        vec![],
        vec![1],
        vec![1, 2],
        vec![1, 2, 2],
        vec![2],
        vec![2, 2],
    ];

    // When
    let actual = subsets_with_dup(generating_from);

    // Then
    assert_eq!(actual, expecting);
}
