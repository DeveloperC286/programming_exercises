use super::*;

#[test]
fn test_equal_length_with_padded_zeros() {
    // Given
    let version_1 = "1.01".to_string();
    let version_2 = "1.001".to_string();
    let expected = 0;

    // When
    let actual = compare_version(version_1, version_2);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_unequal_length_with_padded_zeros() {
    // Given
    let version_1 = "1.0".to_string();
    let version_2 = "1.0.0".to_string();
    let expected = 0;

    // When
    let actual = compare_version(version_1, version_2);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_equal_length_larger_version_2() {
    // Given
    let version_1 = "0.1".to_string();
    let version_2 = "1.1".to_string();
    let expected = -1;

    // When
    let actual = compare_version(version_1, version_2);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_unequal_length_larger_version_1() {
    // Given
    let version_1 = "1.0.1".to_string();
    let version_2 = "1".to_string();
    let expected = 1;

    // When
    let actual = compare_version(version_1, version_2);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_unequal_length_larger_version_2() {
    // Given
    let version_1 = "7.5.2.4".to_string();
    let version_2 = "7.5.3".to_string();
    let expected = -1;

    // When
    let actual = compare_version(version_1, version_2);

    // Then
    assert_eq!(actual, expected);
}
