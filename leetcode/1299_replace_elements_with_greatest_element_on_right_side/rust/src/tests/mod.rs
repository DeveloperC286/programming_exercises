use super::*;

#[test]
fn test_single_element_array() {
    // Given
    let array = vec![400];
    let expected = vec![-1];

    // When
    let actual = replace_elements(array);

    // Then
    assert_eq!(actual, expected);
}

#[test]
fn test_multiple_element_array() {
    // Given
    let array = vec![17, 18, 5, 4, 6, 1];
    let expected = vec![18, 6, 6, 6, 1, -1];

    // When
    let actual = replace_elements(array);

    // Then
    assert_eq!(actual, expected);
}
