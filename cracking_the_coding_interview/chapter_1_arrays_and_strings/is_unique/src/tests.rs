use rstest::rstest;

use super::*;

#[rstest(
    expected,
    testing,
    case(true, "aebcfk"),
    case(false, "aekja"),
    case(false, "eabce")
)]
fn test_is_unique(expected: bool, testing: &str) {
    assert_eq!(expected, is_unique_iterative(testing));
    assert_eq!(expected, is_unique_sorted_iterative(testing));
    assert_eq!(expected, is_unique_sorted_dedup(testing));
}

proptest! {
    #[test]
    fn test_unique_string_is_unique(ref unique_string in "[a-d][e-h][i-l][m-p][q-t][u-z][A-D][E-H][I_L][M-P][Q-T][U-Z]") {
        prop_assert!(is_unique_iterative(unique_string));
        prop_assert!(is_unique_sorted_iterative(unique_string));
        prop_assert!(is_unique_sorted_dedup(unique_string));
    }

    #[test]
    fn test_appended_string_is_not_unique(ref unique_string in "[a-d][e-h][i-l][m-p][q-t][u-z][A-D][E-H][I_L][M-P][Q-T][U-Z]") {
        let non_unique_string = format!("{}{}", unique_string, unique_string);
        prop_assert!(!is_unique_iterative(&non_unique_string));
        prop_assert!(!is_unique_sorted_iterative(&non_unique_string));
        prop_assert!(!is_unique_sorted_dedup(&non_unique_string));
    }
}
