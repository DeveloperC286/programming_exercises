use super::*;

#[test]
fn test_one_row() {
    //Given
    let grid = vec![vec![-1]];
    let expected = 1;

    //When
    let actual = count_negatives(grid);

    //Then
    assert_eq!(actual, expected);
}

#[test]
fn test_two_rows() {
    //Given
    let grid = vec![vec![1, -1], vec![-1, -1]];
    let expected = 3;

    //When
    let actual = count_negatives(grid);

    //Then
    assert_eq!(actual, expected);
}

#[test]
fn test_four_rows() {
    //Given
    let grid = vec![
        vec![4, 3, 2, -1],
        vec![3, 2, 1, -1],
        vec![1, 1, -1, -2],
        vec![-1, -1, -2, -3],
    ];
    let expected = 8;

    //When
    let actual = count_negatives(grid);

    //Then
    assert_eq!(actual, expected);
}

#[test]
fn test_no_negatives() {
    //Given
    let grid = vec![vec![3, 2], vec![1, 0]];
    let expected = 0;

    //When
    let actual = count_negatives(grid);

    //Then
    assert_eq!(actual, expected);
}
