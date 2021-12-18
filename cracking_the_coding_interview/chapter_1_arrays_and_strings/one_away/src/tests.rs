use rstest::rstest;

use super::*;

#[rstest(
    expected,
    from,
    to,
    case(true, "pale", "ple"),
    case(true, "pales", "pale"),
    case(true, "pale", "bale"),
    case(false, "pale", "bake"),
    case(false, "ela", "pale"),
    case(true, "pale", "pale"),
    case(false, "pale", "aleb")
)]
fn test_one_away(expected: bool, from: &str, to: &str) {
    let returned = one_away(from.to_string(), to.to_string());
    assert_eq!(expected, returned);
}

fn add_character(string: String) -> String {
    let mut string_characters: Vec<char> = string.chars().collect();
    string_characters.push('a');
    string_characters.into_iter().collect()
}

proptest! {
    #[test]
    fn test_same_string_is_one_away(string in "\\PC*") {
        let result = one_away(string.clone(), string);
        prop_assert!(result);
    }

    #[test]
    fn test_additional_character_is_one_away(string in "\\PC*") {
        let string_with_added_character = add_character(string.clone());
        let result = one_away(string, string_with_added_character);
        prop_assert!(result);
    }

    #[test]
    fn test_one_less_character_is_one_away(string in "\\PC*") {
        let string_with_added_character = add_character(string.clone());
        let result = one_away(string_with_added_character, string);
        prop_assert!(result);
    }

    #[test]
    fn test_two_additional_characters_is_not_one_away(string in "\\PC*") {
        let string_with_added_character = add_character(add_character(string.clone()));
        let result = one_away(string, string_with_added_character);
        prop_assert!(!result);
    }

    #[test]
    fn test_two_less_characters_is_not_one_away(string in "\\PC*") {
        let string_with_added_character = add_character(add_character(string.clone()));
        let result = one_away(string_with_added_character, string);
        prop_assert!(!result);
    }
}
