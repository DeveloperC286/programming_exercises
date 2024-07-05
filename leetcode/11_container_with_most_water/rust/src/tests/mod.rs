use super::*;

#[test]
fn test_multiple_containers() {
    // Given
    let heights = vec![1,8,6,2,5,4,8,3,7];

    // When
    let returned = max_area(heights);

    // Then
    assert_eq!(returned, 49);
}

#[test]
fn test_two_containers() {
    // Given
    let heights = vec![1,1];

    // When
    let returned = max_area(heights);

    // Then
    assert_eq!(returned, 1);
}
