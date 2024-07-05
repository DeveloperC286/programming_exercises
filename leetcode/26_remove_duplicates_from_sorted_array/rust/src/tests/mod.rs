use super::*;

#[test]
fn test_with_no_duplicates() {
    // Given
    let mut sorted = vec![1, 2, 4];

    // When
    let size = remove_duplicates(&mut sorted);

    // Then
    assert_eq!(size, 3);
    assert_eq!(sorted, vec![1, 2, 4]);
}

#[test]
fn test_with_duplicates() {
    // Given
    let mut sorted = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];

    // When
    let size = remove_duplicates(&mut sorted);

    // Then
    assert_eq!(size, 5);
    assert_eq!(sorted, vec![0, 1, 2, 3, 4]);
}
