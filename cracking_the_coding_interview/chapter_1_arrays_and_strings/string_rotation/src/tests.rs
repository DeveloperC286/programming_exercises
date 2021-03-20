use super::*;
use rstest::rstest;

#[rstest(
    expected,
    testing,
    rotated_string,
    case(true, "waterbottle", "erbottlewat"),
    case(false, "football", "llbaboot"),
    case(false, "ansible", "tbleans"),
    case(true, "candle", "dlecan")
)]
fn test_string_rotation(expected: bool, testing: &str, rotated_string: &str) {
    assert_eq!(expected, string_rotation(testing, rotated_string));
}

#[rstest(
    expected,
    testing,
    rotated_string,
    case(true, "waterbottle", "erbottlewat"),
    case(false, "football", "llbaboot"),
    case(false, "ansible", "tbleans"),
    case(true, "candle", "dlecan")
)]
fn test_string_rotation_brute_force(expected: bool, testing: &str, rotated_string: &str) {
    assert_eq!(
        expected,
        string_rotation_brute_force(testing, rotated_string)
    );
}
