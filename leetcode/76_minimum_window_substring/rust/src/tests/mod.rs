use super::*;

#[test]
fn test_exact_match() {
    // Given
    let searching = "a".to_string();
    let searching_for = "a".to_string();
    let expected = "a".to_string();

    // When
    let actual = min_window(searching, searching_for);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_same_length_not_match() {
    // Given
    let searching = "a".to_string();
    let searching_for = "c".to_string();

    // When
    let actual = min_window(searching, searching_for);

    // Then
    assert!(actual.is_empty());
}

#[test]
fn test_same_length_reversed() {
    // Given
    let searching = "abc".to_string();
    let searching_for = "cba".to_string();
    let expected = "abc".to_string();

    // When
    let actual = min_window(searching, searching_for);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_searching_for_larger() {
    // Given
    let searching = "ba".to_string();
    let searching_for = "bab".to_string();

    // When
    let actual = min_window(searching, searching_for);

    // Then
    assert!(actual.is_empty());
}

#[test]
fn test_larger_than_searching_for_substring() {
    // Given
    let searching = "ADOBECODEBANC".to_string();
    let searching_for = "ABC".to_string();
    let expected = "BANC".to_string();

    // When
    let actual = min_window(searching, searching_for);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_larger_than_searching_for_substring_2() {
    // Given
    let searching = "ABC".to_string();
    let searching_for = "AC".to_string();
    let expected = "ABC".to_string();

    // When
    let actual = min_window(searching, searching_for);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_exact_searching_for_substring() {
    // Given
    let searching = "ADOBECODEBANC".to_string();
    let searching_for = "CODE".to_string();
    let expected = "ECOD".to_string();

    // When
    let actual = min_window(searching, searching_for);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_exact_searching_for_substring_at_end() {
    // Given
    let searching = "ADOBECODEBANC".to_string();
    let searching_for = "ANC".to_string();
    let expected = "ANC".to_string();

    // When
    let actual = min_window(searching, searching_for);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_searching_for_no_substring() {
    // Given
    let searching = "ADOBECODEBANC".to_string();
    let searching_for = "DAZ".to_string();

    // When
    let actual = min_window(searching, searching_for);

    // Then
    assert!(actual.is_empty());
}
