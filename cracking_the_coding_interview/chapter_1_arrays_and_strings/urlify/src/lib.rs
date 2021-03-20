pub fn urlify(urlifing: String, length: i32) -> String {
    let mut url: Vec<char> = urlifing.chars().collect();

    let mut end_word = length - 1;
    let mut start_word = length - 1;
    let mut end_padding: i32 = urlifing.len() as i32 - 1;

    while start_word >= 0 {
        if url[start_word as usize] == ' ' {
            // push current word up
            for i in 0..(end_word - start_word) {
                url[(end_padding - i) as usize] = url[(end_word - i) as usize];
            }

            // add urlifaction
            url[start_word as usize] = '%';
            url[(start_word + 1) as usize] = '2';
            url[(start_word + 2) as usize] = '0';

            end_padding = end_word;
            end_word = start_word + 2;
        }

        start_word -= 1;
    }

    return url.iter().collect();
}

#[cfg(test)]
mod tests;
