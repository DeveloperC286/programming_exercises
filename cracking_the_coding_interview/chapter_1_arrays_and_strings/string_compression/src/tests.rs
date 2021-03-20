use super::*;
use rstest::rstest;

#[rstest(
    expected,
    compressing,
    case("a2b1c5a3", "aabcccccaaa"),
    case("abc", "abc"),
    case("abbc", "abbc")
)]
fn test_string_compression(expected: &str, compressing: &str) {
    let returned = string_compression(compressing);
    assert_eq!(expected.to_string(), returned);
}
