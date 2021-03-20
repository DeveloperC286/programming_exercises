#[cfg(test)]
#[macro_use]
extern crate proptest;

use std::collections::HashSet;

pub fn is_unique_iterative(testing: &str) -> bool {
    let mut characters_seen: HashSet<char> = HashSet::new();

    for character in testing.chars() {
        if characters_seen.contains(&character) {
            return false;
        } else {
            characters_seen.insert(character);
        }
    }

    true
}

pub fn is_unique_sorted_iterative(testing: &str) -> bool {
    let mut sorted_testing: Vec<char> = testing.chars().collect();
    sorted_testing.sort_unstable();

    for i in 0..(sorted_testing.len() - 1) {
        if sorted_testing.get(i) == sorted_testing.get(i + 1) {
            return false;
        }
    }

    true
}

pub fn is_unique_sorted_dedup(testing: &str) -> bool {
    let mut sorted_testing: Vec<char> = testing.chars().collect();
    sorted_testing.sort_unstable();
    sorted_testing.dedup();

    sorted_testing.len() == testing.len()
}

#[cfg(test)]
mod tests;
