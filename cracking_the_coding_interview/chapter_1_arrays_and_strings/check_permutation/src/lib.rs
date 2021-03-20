#[cfg(test)]
#[macro_use]
extern crate proptest;

use std::collections::HashMap;

pub fn is_permutation_iterative(testing: &str, comparing: &str) -> bool {
    if testing.len() != comparing.len() {
        return false;
    }

    let testing_letter_frequency: HashMap<char, u32> = build_letter_frequency_hashmap(testing);
    let comparing_letter_frequency: HashMap<char, u32> = build_letter_frequency_hashmap(comparing);

    testing_letter_frequency == comparing_letter_frequency
}

fn build_letter_frequency_hashmap(string: &str) -> HashMap<char, u32> {
    let mut letter_frequency: HashMap<char, u32> = HashMap::new();

    for character in string.chars() {
        *letter_frequency.entry(character).or_insert(0) += 1;
    }

    letter_frequency
}

pub fn is_permutation_sorted_iterative(testing: &str, comparing: &str) -> bool {
    if testing.len() != comparing.len() {
        return false;
    }

    let mut testing_chars: Vec<char> = testing.chars().collect();
    testing_chars.sort_unstable();

    let mut comparing_chars: Vec<char> = comparing.chars().collect();
    comparing_chars.sort_unstable();

    testing_chars == comparing_chars
}

#[cfg(test)]
mod tests;
