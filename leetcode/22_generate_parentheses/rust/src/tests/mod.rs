use super::*;

#[test]
fn test_n_0() {
    // Given
    let n = 0;

    // When
    let actual = generate_parenthesis(n);

    // Then
    assert!(actual.is_empty());
}

#[test]
fn test_n_1() {
    // Given
    let n = 1;
    let expected = vec!["()".to_string()];

    // When
    let actual = generate_parenthesis(n);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_n_3() {
    // Given
    let n = 3;
    let expected = vec![
        "((()))".to_string(),
        "(()())".to_string(),
        "(())()".to_string(),
        "()(())".to_string(),
        "()()()".to_string(),
    ];

    // When
    let actual = generate_parenthesis(n);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_n_4() {
    // Given
    let n = 4;
    let expected = vec![
        "(((())))".to_string(),
        "((()()))".to_string(),
        "((())())".to_string(),
        "((()))()".to_string(),
        "(()(()))".to_string(),
        "(()()())".to_string(),
        "(()())()".to_string(),
        "(())(())".to_string(),
        "(())()()".to_string(),
        "()((()))".to_string(),
        "()(()())".to_string(),
        "()(())()".to_string(),
        "()()(())".to_string(),
        "()()()()".to_string(),
    ];

    // When
    let actual = generate_parenthesis(n);

    // Then
    assert_eq!(actual, expected);
}
