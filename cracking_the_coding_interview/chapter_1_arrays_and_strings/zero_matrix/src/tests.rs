extern crate nalgebra;
use super::*;
use nalgebra::DMatrix;
use rstest::rstest;

#[rstest(
    expected,
    rotating,
    case(
        DMatrix::from_vec(2, 2, vec![1, 0,
                                     0 ,0]),
        DMatrix::from_vec(2, 2, vec![1, 2,
                                     3, 0])
    ),
    case(
        DMatrix::from_vec(3, 4, vec![1, 0, 3, 4,
                                     0, 0, 0, 0,
                                     9, 0, 11, 12]),
        DMatrix::from_vec(3, 4, vec![1, 2, 3, 4,
                                     5, 0, 7, 8,
                                     9, 10, 11, 12])
    ),
    case(
        DMatrix::from_vec(5, 5, vec![0, 0, 0, 0, 0,
                                     0, 7, 0, 9, 0,
                                     0, 0, 0, 0, 0,
                                     0, 17, 0, 19, 0,
                                     0, 0, 0, 0, 0]),
        DMatrix::from_vec(5, 5, vec![0, 2, 3, 4, 5,
                                     6, 7, 8, 9, 10,
                                     11, 12, 0, 14, 15,
                                     16, 17, 18, 19, 20,
                                     21, 22, 23, 24, 0])
    )
)]
fn test_zero_matrix(expected: DMatrix<i32>, rotating: DMatrix<i32>) {
    let returned = zero_matrix(rotating);
    assert_eq!(expected, returned);
}
