use super::*;

#[test]
fn test_positive_reverse() {
    // Given
    let reversing = 123;
    let expected = 321;

    // When
    let actual = reverse(reversing);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_positive_reverse_2() {
    // Given
    let reversing = 120;
    let expected = 21;

    // When
    let actual = reverse(reversing);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_negative_reverse() {
    // Given
    let reversing = -123;
    let expected = -321;

    // When
    let actual = reverse(reversing);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_positive_overflow() {
    // Given
    let reversing = 1534236469;
    let expected = 0;

    // When
    let actual = reverse(reversing);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_negative_overflow() {
    // Given
    let reversing = -2147483648;
    let expected = 0;

    // When
    let actual = reverse(reversing);

    // Then
    assert_eq!(actual, expected);
}
