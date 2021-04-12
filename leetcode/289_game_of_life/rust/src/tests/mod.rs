use super::*;

#[test]
fn test_3_by_4_board() {
    // Given
    let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
    let expected = vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]];

    // When
    game_of_life(&mut board);

    // Then
    assert_eq!(board, expected);
}

#[test]
fn test_2_by_2_board() {
    // Given
    let mut board = vec![vec![1, 1], vec![1, 0]];
    let expected = vec![vec![1, 1], vec![1, 1]];

    // When
    game_of_life(&mut board);

    // Then
    assert_eq!(board, expected);
}
