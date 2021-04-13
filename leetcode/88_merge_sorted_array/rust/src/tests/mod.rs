use super::*;

#[test]
fn test_empty_to_merge_array() {
    // Given
    let mut merge_into = vec![1];
    let merge_into_len = 1;
    let mut to_merge = vec![];
    let to_merge_len = 0;
    let expected = vec![1];

    // When
    merge(&mut merge_into, merge_into_len, &mut to_merge, to_merge_len);

    // Then
    assert_eq!(merge_into, expected);
}

#[test]
fn test_empty_merge_into_array() {
    // Given
    let mut merge_into = vec![];
    let merge_into_len = 0;
    let mut to_merge = vec![1];
    let to_merge_len = 1;
    let expected = vec![1];

    // When
    merge(&mut merge_into, merge_into_len, &mut to_merge, to_merge_len);

    // Then
    assert_eq!(merge_into, expected);
}

#[test]
fn test_equal_odd_length_arrays() {
    // Given
    let mut merge_into = vec![1, 2, 3, 0, 0, 0];
    let merge_into_len = 3;
    let mut to_merge = vec![2, 5, 6];
    let to_merge_len = 3;
    let expected = vec![1, 2, 2, 3, 5, 6];

    // When
    merge(&mut merge_into, merge_into_len, &mut to_merge, to_merge_len);

    // Then
    assert_eq!(merge_into, expected);
}
