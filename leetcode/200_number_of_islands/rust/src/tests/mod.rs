use super::*;

#[test]
fn test_one_large_island() {
    // Given
    let map = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];

    // When
    let actual = num_islands(map);

    // Then
    assert_eq!(actual, 1);
}

#[test]
fn test_multiple_diagonal_islands() {
    // Given
    let map = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];

    // When
    let actual = num_islands(map);

    // Then
    assert_eq!(actual, 3);
}

#[test]
fn test_t_shaped_island() {
    // Given
    let map = vec![
        vec!['1', '1', '1'],
        vec!['0', '1', '0'],
        vec!['1', '1', '1'],
    ];

    // When
    let actual = num_islands(map);

    // Then
    assert_eq!(actual, 1);
}

#[test]
fn test_s_shaped_island() {
    // Given
    let map = vec![
        vec!['1', '0', '1', '1', '1'],
        vec!['1', '0', '1', '0', '1'],
        vec!['1', '1', '1', '0', '1'],
    ];

    // When
    let actual = num_islands(map);

    // Then
    assert_eq!(actual, 1);
}

/*
[['1', '0', '1', '1', '1'],
 ['1', '0', '1', '0', '1'],
  ['1', '1', '1', '0', '1']]

[['0', '0', '1', '1', '1'],
['0', '0', '1', '0', '1'],
['0', '0', '0', '0', '1']]

 */
