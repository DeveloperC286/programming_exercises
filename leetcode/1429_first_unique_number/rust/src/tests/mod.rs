use super::*;

#[test]
fn test_successive_add() {
    // Given
    let mut first_unique = FirstUnique::new(vec![809]);

    // Then
    assert_eq!(first_unique.show_first_unique(), 809);

    // When
    first_unique.add(809);

    // Then
    assert_eq!(first_unique.show_first_unique(), -1);
}

#[test]
fn test_new_add() {
    // Given
    let mut first_unique = FirstUnique::new(vec![7, 7, 7, 7, 7, 7]);

    // Then
    assert_eq!(first_unique.show_first_unique(), -1);

    // When
    first_unique.add(7);
    first_unique.add(3);
    first_unique.add(3);
    first_unique.add(7);
    first_unique.add(17);

    // Then
    assert_eq!(first_unique.show_first_unique(), 17);
}

//["FirstUnique","showFirstUnique","add","showFirstUnique","add","showFirstUnique","add","showFirstUnique"]
//[[[2,3,5]],[],[5],[],[2],[],[3],[]]
#[test]
fn test_successive_show_first_unique() {
    // Given
    let mut first_unique = FirstUnique::new(vec![2, 3, 5]);

    // Then
    assert_eq!(first_unique.show_first_unique(), 2);

    // When/Then
    first_unique.add(5);
    assert_eq!(first_unique.show_first_unique(), 2);

    // When/Then
    first_unique.add(2);
    assert_eq!(first_unique.show_first_unique(), 3);

    // When/Then
    first_unique.add(3);
    assert_eq!(first_unique.show_first_unique(), -1);
}
