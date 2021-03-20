pub fn string_compression(compressing: &str) -> String {
    let mut compressed = String::new();
    let mut last_character = compressing.chars().next().unwrap();
    let mut count = 0;

    for character in compressing.chars() {
        if last_character == character {
            count += 1;
        } else {
            //add to compressed string
            compressed.push(last_character);
            compressed.push_str(&count.to_string());

            //reset vars to new char
            last_character = character;
            count = 1;
        }
    }
    compressed.push(last_character);
    compressed.push_str(&count.to_string());

    if compressed.len() < compressing.len() {
        compressed
    } else {
        compressing.to_string()
    }
}

#[cfg(test)]
mod tests;
