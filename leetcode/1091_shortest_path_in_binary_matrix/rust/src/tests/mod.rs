use super::*;

#[test]
fn test_2_by_2_grid() {
    // Given
    let grid = vec![vec![0, 1], vec![1, 0]];
    let expected = 2;

    // When
    let actual = shortest_path_binary_matrix(grid);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_3_by_3_grid() {
    // Given
    let grid = vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
    let expected = 4;

    // When
    let actual = shortest_path_binary_matrix(grid);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_3_by_3_grid_with_no_path() {
    // Given
    let grid = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
    let expected = -1;

    // When
    let actual = shortest_path_binary_matrix(grid);

    // Then
    assert_eq!(actual, expected);
}
