use super::*;

#[test]
fn test_all_words_used() {
    // Given
    let breaking = "leetcode".to_string();
    let words = vec!["leet".to_string(), "code".to_string()];

    // When/Then
    assert!(word_break(breaking, words));
}

#[test]
fn test_word_used_multiple_times() {
    // Given
    let breaking = "applepenapple".to_string();
    let words = vec!["apple".to_string(), "pen".to_string()];

    // When/Then
    assert!(word_break(breaking, words));
}

#[test]
fn test_can_not_word_break() {
    // Given
    let breaking = "catsandog".to_string();
    let words = vec![
        "cats".to_string(),
        "dog".to_string(),
        "sand".to_string(),
        "and".to_string(),
        "cat".to_string(),
    ];

    // When/Then
    assert!(!word_break(breaking, words));
}

#[test]
fn test_tries_longest_first() {
    // Given
    let breaking = "aaaa".to_string();
    let words = vec![
        "a".to_string(),
        "aa".to_string(),
        "aaa".to_string(),
        "aaaa".to_string(),
    ];

    // When/Then
    assert!(word_break(breaking, words));
}

#[test]
fn test_multiple_initial_paths() {
    // Given
    let breaking = "abcd".to_string();
    let words = vec![
        "a".to_string(),
        "abc".to_string(),
        "b".to_string(),
        "cd".to_string(),
    ];

    // When/Then
    assert!(word_break(breaking, words));
}
