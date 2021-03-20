use std::collections::{HashMap, HashSet};

pub fn first_uniq_char(searching: String) -> i32 {
    let mut character_count = HashMap::new();

    for character in searching.chars() {
        *character_count.entry(character).or_insert(0) += 1;
    }

    let uniq_chars: HashSet<char> = character_count
        .into_iter()
        .filter(|(_, count)| *count == 1)
        .map(|(character, _)| character)
        .collect();

    for (index, character) in searching.chars().enumerate() {
        if uniq_chars.contains(&character) {
            return index as i32;
        }
    }

    -1
}

#[cfg(test)]
mod tests;
