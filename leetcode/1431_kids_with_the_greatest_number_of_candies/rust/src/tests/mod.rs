use super::*;

#[test]
fn test_multiple_possible_greatest() {
    // Given
    let candies = vec![2, 3, 5, 1, 3];
    let extra_candies = 3;
    let expected = vec![true, true, true, false, true];

    // When
    let actual = kids_with_candies(candies, extra_candies);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_single_possible_greatest() {
    // Given
    let candies = vec![4, 2, 1, 1, 2];
    let extra_candies = 1;
    let expected = vec![true, false, false, false, false];

    // When
    let actual = kids_with_candies(candies, extra_candies);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_multiple_possible_greatest_2() {
    // Given
    let candies = vec![12, 1, 12];
    let extra_candies = 10;
    let expected = vec![true, false, true];

    // When
    let actual = kids_with_candies(candies, extra_candies);

    // Then
    assert_eq!(actual, expected);
}
