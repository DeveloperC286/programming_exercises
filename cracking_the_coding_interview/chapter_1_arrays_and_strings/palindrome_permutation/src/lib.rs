use std::collections::HashSet;

pub fn is_palindrome_permutation(testing: &str) -> bool {
    let mut unpartnered_characters: HashSet<char> = HashSet::new();

    for character in testing.chars() {
        if unpartnered_characters.contains(&character) {
            unpartnered_characters.remove(&character);
        } else {
            unpartnered_characters.insert(character);
        }
    }

    let number_of_characters = testing.chars().count();
    //if even and no unmatched chars
    //if odd and theirs one unmatched char
    if number_of_characters % 2 == unpartnered_characters.len() {
        return true;
    }

    false
}

#[cfg(test)]
extern crate proptest;

#[cfg(test)]
mod tests;
