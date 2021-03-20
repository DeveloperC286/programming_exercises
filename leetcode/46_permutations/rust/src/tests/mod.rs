use super::*;

#[test]
fn test_empty_vector() {
    // Given
    let numbers = vec![];
    let expected: Vec<Vec<i32>> = vec![];

    // When
    let actual = permute(numbers);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_single_digit() {
    // Given
    let numbers = vec![1];
    let expected: Vec<Vec<i32>> = vec![vec![1]];

    // When
    let actual = permute(numbers);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_two_digits() {
    // Given
    let numbers = vec![0, 1];
    let expected: Vec<Vec<i32>> = vec![vec![0, 1], vec![1, 0]];

    // When
    let actual = permute(numbers);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_three_digits() {
    // Given
    let numbers = vec![1, 2, 3];
    let expected: Vec<Vec<i32>> = vec![
        vec![1, 2, 3],
        vec![1, 3, 2],
        vec![2, 1, 3],
        vec![2, 3, 1],
        vec![3, 1, 2],
        vec![3, 2, 1],
    ];

    // When
    let actual = permute(numbers);

    // Then
    assert_eq!(actual, expected);
}
