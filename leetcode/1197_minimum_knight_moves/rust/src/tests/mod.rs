use super::*;

#[test]
fn test_one_move() {
    // Given
    let (desired_x, desired_y) = (2, 1);
    let expected = 1;

    // When
    let actual = min_knight_moves(desired_x, desired_y);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_two_moves() {
    // Given
    let (desired_x, desired_y) = (1, 1);
    let expected = 2;

    // When
    let actual = min_knight_moves(desired_x, desired_y);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_four_moves() {
    // Given
    let (desired_x, desired_y) = (5, 5);
    let expected = 4;

    // When
    let actual = min_knight_moves(desired_x, desired_y);

    // Then
    assert_eq!(actual, expected);
}
