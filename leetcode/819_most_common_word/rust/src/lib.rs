use std::collections::{HashMap, HashSet};

pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
    let banned: HashSet<String> = banned.into_iter().collect();
    let mut word_map = HashMap::new();

    let cleaned_paragraph: String = paragraph
        .to_lowercase()
        .chars()
        .filter(|character| *character != '\'')
        .map(|character| match character {
            ',' | '.' | '!' | '?' | ';' => ' ',
            _ => character,
        })
        .collect();

    for word in cleaned_paragraph
        .split(' ')
        .map(|word| word.to_string())
        .filter(|word| !word.is_empty())
        .filter(|word| !banned.contains(word))
    {
        *word_map.entry(word.to_string()).or_insert(0) += 1;
    }

    //assume result and is unique
    let mut count_first: Vec<(i32, String)> = word_map
        .into_iter()
        .map(|(word, count)| (count, word))
        .collect();
    count_first.sort();

    count_first.last().unwrap().1.clone()
}

#[cfg(test)]
mod tests;
