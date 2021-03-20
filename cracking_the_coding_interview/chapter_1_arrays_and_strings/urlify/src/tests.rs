use super::*;
use rstest::rstest;

#[rstest(
    expected,
    urlifing,
    length,
    case("test%20test", "test test  ", 9),
    case("test%20test%20test", "test test test    ", 14),
    case("%20test", " test  ", 5),
    case("test%20", "test   ", 5)
)]
fn test_is_unique(expected: &str, urlifing: &str, length: i32) {
    let returned = urlify(urlifing.to_string(), length);
    assert_eq!(expected, returned);
}
