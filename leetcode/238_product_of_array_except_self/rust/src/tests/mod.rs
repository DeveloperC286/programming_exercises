use super::*;

#[test]
fn test_positive_values() {
    // Given
    let nums = vec![1, 2, 3, 4];
    let expected = vec![24, 12, 8, 6];

    // When
    let actual = product_except_self(nums);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_zero_value() {
    // Given
    let nums = vec![-1, 1, 0, -3, 3];
    let expected = vec![0, 0, 9, 0, 0];

    // When
    let actual = product_except_self(nums);

    // Then
    assert_eq!(actual, expected);
}
