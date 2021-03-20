use rstest::rstest;

use super::*;

#[rstest(
    expected,
    testing,
    comparing,
    case(true, "ced", "dce"),
    case(false, "adf", "fde"),
    case(false, "adf", "aadf")
)]
fn test_is_permutation(expected: bool, testing: &str, comparing: &str) {
    assert_eq!(expected, is_permutation_iterative(testing, comparing));
    assert_eq!(
        expected,
        is_permutation_sorted_iterative(testing, comparing)
    );
}

proptest! {
    #[test]
    fn test_string_is_permutation_of_itself(ref string in "\\PC*") {
        prop_assert!(is_permutation_iterative(string, string));
        prop_assert!(is_permutation_sorted_iterative(string, string));
    }

    #[test]
    fn test_different_strings_are_not_permutations(ref string in "\\PC*") {
        prop_assume!(!string.is_empty());
        //Clone string and replace a random char to create a non permutation.
        let mut different_string_chars :Vec<char>= string.chars().collect();
        let  different_string_char_index = rand::random::<usize>() % different_string_chars.len();

        //Generate a char not equal to the one replacing.
        let mut random_char = rand::random::<char>();
        while random_char == *different_string_chars.get(different_string_char_index).unwrap() {
             random_char = rand::random::<char>();
        }

        let _ = std::mem::replace(&mut different_string_chars[different_string_char_index], random_char);
        let different_string:String = different_string_chars.into_iter().collect();

        prop_assert!(!is_permutation_iterative(string, &different_string));
        prop_assert!(!is_permutation_sorted_iterative(string, &different_string));
    }

    #[test]
    fn test_reordered_string_is_permutation_of_itself(ref string in "\\PC*") {
       let reversed_string = string.chars().rev().collect::<String>();
        prop_assert!(is_permutation_iterative(string, &reversed_string));
        prop_assert!(is_permutation_sorted_iterative(string, &reversed_string));
    }
}
