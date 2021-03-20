use super::*;

#[test]
fn test_empty_string() {
    // Given
    let searching = "".to_string();

    // When
    let actual = first_uniq_char(searching);

    // Then
    assert_eq!(actual, -1);
}

#[test]
fn test_first_character() {
    // Given
    let searching = "leetcode".to_string();

    // When
    let actual = first_uniq_char(searching);

    // Then
    assert_eq!(actual, 0);
}

#[test]
fn test_third_character() {
    // Given
    let searching = "loveleetcode".to_string();

    // When
    let actual = first_uniq_char(searching);

    // Then
    assert_eq!(actual, 2);
}
