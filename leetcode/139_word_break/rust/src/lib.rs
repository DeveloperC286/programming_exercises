pub fn word_break(breaking: String, mut words: Vec<String>) -> bool {
    // Sort words and use the largest first.
    words.sort_unstable_by(|a, b| a.len().cmp(&b.len()).reverse());
    let mut already_processed = vec![false; breaking.len()];

    // Depth first searching.
    fn process(
        breaking: &str,
        words: &[String],
        break_from: usize,
        already_processed: &mut Vec<bool>,
    ) -> bool {
        if !already_processed[break_from] {
            already_processed[break_from] = true;
            for word in words {
                let break_till = break_from + word.len();

                if break_till <= breaking.len()
                    && breaking[break_from..].starts_with(word)
                    && (break_till == breaking.len()
                        || process(&breaking, &words, break_till, already_processed))
                {
                    return true;
                }
            }
        }

        false
    }

    process(&breaking, &words, 0, &mut already_processed)
}

#[cfg(test)]
mod tests;
