use super::*;

#[test]
fn test_single_pair() {
    // Given
    let input = "()".to_string();

    // When/Then
    assert!(is_valid(input));
}

#[test]
fn test_single_parentheses() {
    // Given
    let input = "[".to_string();

    // When/Then
    assert!(!is_valid(input));
}

#[test]
fn test_single_unmatched_pair() {
    // Given
    let input = "[}".to_string();

    // When/Then
    assert!(!is_valid(input));
}

#[test]
fn test_multiple_consecutive_pairs() {
    // Given
    let input = "{}()[]".to_string();

    // When/Then
    assert!(is_valid(input));
}

#[test]
fn test_multiple_embedded_pairs() {
    // Given
    let input = "{{()}}".to_string();

    // When/Then
    assert!(is_valid(input));
}

#[test]
fn test_multiple_embedded_unmatched_pairs() {
    // Given
    let input = "{{(}}".to_string();

    // When/Then
    assert!(!is_valid(input));
}
