use super::*;

#[test]
fn test_mailbox_per_house() {
    // Given
    let houses = vec![3, 6, 14, 10];
    let number_of_mailboxes = 4;
    let expected = 0;

    // When
    let actual = min_distance(houses, number_of_mailboxes);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_only_one_mailbox() {
    // Given
    let houses = vec![7, 4, 6, 1];
    let number_of_mailboxes = 1;
    let expected = 8;

    // When
    let actual = min_distance(houses, number_of_mailboxes);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_two_max_per_mailbox() {
    // Given
    let houses = vec![1, 4, 8, 10, 20];
    let number_of_mailboxes = 3;
    let expected = 5;

    // When
    let actual = min_distance(houses, number_of_mailboxes);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_more_than_two_per_mailbox() {
    // Given
    let houses = vec![2, 3, 5, 12, 18];
    let number_of_mailboxes = 2;
    let expected = 9;

    // When
    let actual = min_distance(houses, number_of_mailboxes);

    // Then
    assert_eq!(actual, expected);
}
