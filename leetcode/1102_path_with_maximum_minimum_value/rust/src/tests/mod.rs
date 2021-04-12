use super::*;

#[test]
fn test_3_by_3_grid() {
    // Given
    let grid = vec![vec![5, 4, 5], vec![1, 2, 6], vec![7, 4, 6]];
    let expected = 4;

    // When
    let actual = maximum_minimum_path(grid);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_4_by_4_grid() {
    // Given
    let grid = vec![
        vec![5, 4, 3, 1],
        vec![7, 4, 5, 2],
        vec![4, 9, 8, 2],
        vec![2, 3, 6, 6],
    ];
    let expected = 4;

    // When
    let actual = maximum_minimum_path(grid);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_6_by_5_grid() {
    // Given
    let grid = vec![
        vec![3, 4, 6, 3, 4],
        vec![0, 2, 1, 1, 7],
        vec![8, 8, 3, 2, 7],
        vec![3, 2, 4, 9, 8],
        vec![4, 1, 2, 0, 0],
        vec![4, 6, 5, 4, 3],
    ];
    let expected = 3;

    // When
    let actual = maximum_minimum_path(grid);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_2_by_6_grid() {
    // Given
    let grid = vec![vec![2, 2, 1, 2, 2, 2], vec![1, 2, 2, 2, 1, 2]];
    let expected = 2;

    // When
    let actual = maximum_minimum_path(grid);

    // Then
    assert_eq!(actual, expected);
}
