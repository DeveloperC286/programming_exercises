use assert_approx_eq::assert_approx_eq;

use super::*;

#[test]
fn test_decimal_numbers() {
    // Given
    let x = 2.1;
    let n = 3;
    let expected: f64 = 9.261000000000001;

    // When
    let actual = my_pow(x, n);

    // Then
    assert_approx_eq!(actual, expected);
}

#[test]
fn test_natural_numbers() {
    // Given
    let x = 2.0;
    let n = 10;
    let expected: f64 = 1024.0;

    // When
    let actual = my_pow(x, n);

    // Then
    assert_approx_eq!(actual, expected);
}

#[test]
fn test_negative_power() {
    // Given
    let x = 2.0;
    let n = -2;
    let expected: f64 = 0.25;

    // When
    let actual = my_pow(x, n);

    // Then
    assert_approx_eq!(actual, expected);
}
