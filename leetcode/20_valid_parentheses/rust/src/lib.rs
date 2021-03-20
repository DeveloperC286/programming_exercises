pub fn is_valid(s: String) -> bool {
    let mut next_expected_parentheses = vec![];

    for character in s.chars() {
        match character {
            '(' => next_expected_parentheses.push(')'),
            '[' => next_expected_parentheses.push(']'),
            '{' => next_expected_parentheses.push('}'),
            _ => {
                if Some(character) != next_expected_parentheses.pop() {
                    return false;
                }
            }
        }
    }

    next_expected_parentheses.is_empty()
}

#[cfg(test)]
mod tests;
