#[macro_use]
extern crate log;

pub fn one_away(from: String, to: String) -> bool {
    let (size_difference, longer_chars, shorter_chars) = sort_strings(from, to);
    let mut characters_different = 0;

    //replace
    if size_difference == 0 {
        for i in 0..shorter_chars.len() {
            if longer_chars[i] != shorter_chars[i] {
                warn!(
                    "longer_chars[{}] '{}' != shorter_chars[{}] '{}'",
                    i, longer_chars[i], i, shorter_chars[i]
                );
                characters_different += 1;
            }
        }

        info!("characters_different = {}", characters_different);
        if characters_different <= 1 {
            return true;
        }
    }

    //insert or remove
    if size_difference == 1 {
        let mut shorter_index = 0;
        let mut longer_index = 0;

        while longer_index < longer_chars.len() - 1 {
            if longer_chars[longer_index] == shorter_chars[shorter_index] {
                shorter_index += 1;
            } else {
                characters_different += 1;
            }

            longer_index += 1;
        }

        info!("characters_different = {}", characters_different);
        if characters_different <= 1 {
            return true;
        }
    }

    false
}

fn sort_strings(string1: String, string2: String) -> (i32, Vec<char>, Vec<char>) {
    if string1.chars().count() > string2.chars().count() {
        return (
            (string1.chars().count() - string2.chars().count()) as i32,
            string1.chars().collect(),
            string2.chars().collect(),
        );
    } else {
        return (
            (string2.chars().count() - string1.chars().count()) as i32,
            string2.chars().collect(),
            string1.chars().collect(),
        );
    }
}

#[cfg(test)]
#[macro_use]
extern crate proptest;

#[cfg(test)]
extern crate pretty_env_logger;

#[cfg(test)]
mod tests;
