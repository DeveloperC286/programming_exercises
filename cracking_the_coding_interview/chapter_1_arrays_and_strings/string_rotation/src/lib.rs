pub fn string_rotation(testing: &str, rotated_string: &str) -> bool {
    let appended_rotated_strings: String = String::from(rotated_string) + rotated_string;
    appended_rotated_strings.contains(testing)
}

pub fn string_rotation_brute_force(testing: &str, rotated_string: &str) -> bool {
    if testing.len() == rotated_string.len() {
        let n = testing.len();

        for offset in 0..n {
            let mut new_string = String::new();

            for index in 0..n {
                new_string.push(rotated_string.chars().nth((index + offset) % n).unwrap());
            }

            if new_string == testing {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests;
