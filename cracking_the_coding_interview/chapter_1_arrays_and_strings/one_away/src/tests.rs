use rstest::rstest;

use super::*;

#[rstest(
    current,
    desired,
    case("pale", "pale"),
    case("unchanged", "unchanged"),
    case("", ""),
    case("sat", "sat")
)]
fn test_one_away_no_change(current: &str, desired: &str) {
    assert_eq!(
        Change::NONE,
        one_away(current.to_string(), desired.to_string())
    );
}

#[rstest(
    current,
    desired,
    case("unchanged", "changed"),
    case("sat", "saturday"),
    case("pale", "bake"),
    case("ela", "pale"),
    case("pale", "aleb"),
    case("", "al")
)]
fn test_one_away_impossible_change(current: &str, desired: &str) {
    assert_eq!(
        Change::IMPOSSIBLE,
        one_away(current.to_string(), desired.to_string())
    );
}

#[rstest(
    to_insert,
    current,
    desired,
    case('a', "ple", "pale"),
    case('l', "shel", "shell"),
    case('s', "he", "she"),
    case('c', "", "c"),
    case('é', "", "é")
)]
fn test_one_away_insert_change(to_insert: char, current: &str, desired: &str) {
    assert_eq!(
        Change::INSERT { to_insert },
        one_away(current.to_string(), desired.to_string())
    );
}

#[rstest(
    to_remove,
    current,
    desired,
    case('a', "pale", "ple"),
    case('l', "shell", "shel"),
    case('s', "she", "he"),
    case('c', "c", "")
)]
fn test_one_away_remove_change(to_remove: char, current: &str, desired: &str) {
    assert_eq!(
        Change::REMOVE { to_remove },
        one_away(current.to_string(), desired.to_string())
    );
}

#[rstest(
    to_move,
    current,
    desired,
    case('l', "plae", "pale"),
    case('e', "shlle", "shell"),
    case('s', "she", "hes")
)]
fn test_one_away_move_change(to_move: char, current: &str, desired: &str) {
    assert_eq!(
        Change::MOVE { to_move },
        one_away(current.to_string(), desired.to_string())
    );
}

#[rstest(
    to_replace,
    current,
    desired,
    case('p', "pale", "bale"),
    case('a', "data", "date"),
    case('n', "one", "ole"),
    case('é', "é", "𒑱")
)]
fn test_one_away_replace_change(to_replace: char, current: &str, desired: &str) {
    assert_eq!(
        Change::REPLACE { to_replace },
        one_away(current.to_string(), desired.to_string())
    );
}

proptest! {
    #[test]
fn proptest_one_away_no_change(ref string in "\\PC*") {
        prop_assert!(Change::NONE == one_away(string.to_string(), string.to_string()));
    }

    #[test]
    fn proptest_one_away_impossible_change(ref string in "\\PC*", number_of_random_chars in 2u32..10) {
        let mut string2_chars: Vec<char> = string.chars().collect();

        for _ in 0..number_of_random_chars {
            string2_chars.push(rand::random::<char>());
        }

        let string2: String = string2_chars.into_iter().collect();
        prop_assert!(Change::IMPOSSIBLE == one_away(string.to_string(), string2));
    }

    #[test]
    fn proptest_one_away_insert_change(ref string in "\\PC*") {
        let mut string2_chars: Vec<char> = string.chars().collect();
        let to_insert = rand::random::<char>();
        string2_chars.push(to_insert);
        let string2: String = string2_chars.into_iter().collect();

        let expected = Change::INSERT { to_insert };

        prop_assert!(expected == one_away(string.to_string(), string2));
    }

    #[test]
    fn proptest_one_away_remove_change(ref string in "\\PC*") {
        let mut string2_chars: Vec<char> = string.chars().collect();
        let to_remove = rand::random::<char>();
        string2_chars.push(to_remove);
        let string2: String = string2_chars.into_iter().collect();

        let expected = Change::REMOVE { to_remove };

        prop_assert!(expected == one_away(string2, string.to_string()));
    }

    #[test]
    fn proptest_one_away_replace_change(ref string in "\\PC*") {
        prop_assume!(!string.is_empty());
        let mut string2_chars: Vec<char> = string.chars().collect();
        let to_replace = string2_chars.pop().unwrap();
        string2_chars.push( rand::random::<char>());
        let string2: String = string2_chars.into_iter().collect();

        let expected = Change::REPLACE{ to_replace};

        println!("{:?} vs {:?}", string2, string);
        prop_assert!(expected == one_away(string2, string.to_string()));
    }
}
