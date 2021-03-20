extern crate nalgebra;
use super::*;
use nalgebra::DMatrix;
use rstest::rstest;

#[rstest(
    expected,
    rotating,
    case(
        DMatrix::from_vec(2, 2, vec![3, 1, 4, 2]),
        DMatrix::from_vec(2, 2, vec![1, 2, 3, 4])
    ),
    case(
        DMatrix::from_vec(3, 3, vec![7, 4, 1, 8, 5, 2, 9, 6, 3]),
        DMatrix::from_vec(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
    ),
    case(
        DMatrix::from_vec(5, 5, vec![21, 16, 11, 6, 1, 22, 17, 12, 7, 2, 23, 18, 13, 8, 3, 24, 19, 14, 9, 4, 25, 20, 15, 10, 5]),
        DMatrix::from_vec(5, 5, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25])
    ),
    case(
        DMatrix::from_vec(3, 3, vec![9, 8, 7, 6, 5, 4, 3, 2, 1]),
        DMatrix::from_vec(3, 3, vec![7, 4, 1, 8, 5, 2, 9, 6, 3])
    )
)]
fn test_rotate_matrix(expected: DMatrix<i32>, rotating: DMatrix<i32>) {
    let returned = rotate_matrix(rotating);
    assert_eq!(expected, returned);
}
