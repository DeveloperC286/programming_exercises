use super::*;
use proptest::prelude::*;
use rstest::rstest;

#[rstest(
    expected,
    testing,
    case(true, "tactcoa"),
    case(true, "amacaraaamnaacr"),
    case(false, "abc"),
    case(true, "𐬹")
)]
fn test_palindrome_permutation(expected: bool, testing: &str) {
    assert_eq!(expected, is_palindrome_permutation(testing));
}

proptest! {
    #[test]
    fn test_string_appened_is_palindrome_permutation(ref string in "\\PC*") {
        let result = is_palindrome_permutation(&format!("{}{}", string, string));
        prop_assert!(result);
    }

    #[test]
    fn test_string_odd_appened_is_palindrome_permutation(ref string in "\\PC*", ref character in "\\PC") {
        let result = is_palindrome_permutation(&format!("{}{}{}", string, character, string));
        prop_assert!(result);
    }
}
