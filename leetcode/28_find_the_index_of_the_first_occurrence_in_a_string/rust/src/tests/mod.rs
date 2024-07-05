use super::*;

#[test]
fn test_contains_needle() {
    // Given
    let haystack = "sadbutsad".to_string();
    let needle = "sad".to_string();

    // When
    let returned = str_str(haystack, needle);

    // Then
    assert_eq!(returned, 0);
}

#[test]
fn test_doesnt_contain_needle() {
    // Given
    let haystack = "leetcode".to_string();
    let needle = "leeto".to_string();

    // When
    let returned = str_str(haystack, needle);

    // Then
    assert_eq!(returned, -1);
}


#[test]
fn test_larger_needle() {
    // Given
    let haystack = "abb".to_string();
    let needle = "abaaa".to_string();

    // When
    let returned = str_str(haystack, needle);

    // Then
    assert_eq!(returned, -1);
}
