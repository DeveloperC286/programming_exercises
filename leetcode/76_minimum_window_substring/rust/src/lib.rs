use std::collections::{HashMap, HashSet};

pub fn min_window(searching: String, searching_for: String) -> String {
    if searching_for.len() > searching.len() {
        return String::new();
    }

    let searching_for_chars = get_chars(&searching_for);
    let searching_for_char_counts = get_char_counts(&searching_for);

    if searching.len() == searching_for.len() {
        return if contains_any_order(
            &get_char_counts_and_ignore(&searching, &searching_for_chars),
            &searching_for_char_counts,
        ) {
            searching
        } else {
            String::new()
        };
    }

    let mut left_pointer: usize = 0;
    let mut right_pointer: usize = searching_for.len();
    let mut searching_char_counts = get_char_counts_and_ignore(
        &searching[left_pointer..right_pointer],
        &searching_for_chars,
    );
    let mut smallest_substring_optional: Option<String> = None;

    while right_pointer <= searching.len() {
        let comparing = &searching[left_pointer..right_pointer];

        if contains_any_order(&searching_char_counts, &searching_for_char_counts) {
            if let Some(smallest_substring) = &smallest_substring_optional {
                if smallest_substring.len() > comparing.len() {
                    smallest_substring_optional = Some(comparing.to_string());
                }
            } else {
                smallest_substring_optional = Some(comparing.to_string());
            }

            let left_pointer_char = searching.chars().nth(left_pointer).unwrap();

            if searching_for_chars.contains(&left_pointer_char) {
                let searching_char_count_value =
                    searching_char_counts.get(&left_pointer_char).unwrap();

                if *searching_char_count_value == 1 {
                    searching_char_counts.remove(&left_pointer_char);
                } else {
                    *searching_char_counts.entry(left_pointer_char).or_insert(0) -= 1;
                }
            }

            left_pointer += 1;
        } else {
            right_pointer += 1;

            if let Some(right_pointer_char) = searching.chars().nth(right_pointer - 1) {
                if searching_for_chars.contains(&right_pointer_char) {
                    *searching_char_counts.entry(right_pointer_char).or_insert(0) += 1;
                }
            }
        }
    }

    smallest_substring_optional.unwrap_or_default()
}

fn contains_any_order(
    searching_char_counts: &HashMap<char, usize>,
    searching_for_char_counts: &HashMap<char, usize>,
) -> bool {
    if searching_char_counts.len() != searching_for_char_counts.len() {
        return false;
    }

    for (searching_key, searching_value) in searching_char_counts {
        let searching_for_value = searching_for_char_counts.get(searching_key).unwrap();

        if searching_value < searching_for_value {
            return false;
        }
    }

    true
}

fn get_char_counts(building_for: &str) -> HashMap<char, usize> {
    let mut char_count: HashMap<char, usize> = HashMap::new();

    for character in building_for.chars() {
        *char_count.entry(character).or_insert(0) += 1;
    }

    char_count
}

fn get_char_counts_and_ignore(
    building_for: &str,
    searching_for_chars: &HashSet<char>,
) -> HashMap<char, usize> {
    let mut char_count: HashMap<char, usize> = HashMap::new();

    for character in building_for
        .chars()
        .filter(|x| searching_for_chars.contains(x))
    {
        *char_count.entry(character).or_insert(0) += 1;
    }

    char_count
}

fn get_chars(building_for: &str) -> HashSet<char> {
    let mut chars: HashSet<char> = HashSet::new();

    for character in building_for.chars() {
        chars.insert(character);
    }

    chars
}

#[cfg(test)]
mod tests;
