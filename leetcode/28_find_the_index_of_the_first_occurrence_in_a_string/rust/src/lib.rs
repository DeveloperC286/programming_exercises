pub fn str_str(haystack: String, needle: String) -> i32 {
    let lead = needle.len() - 1;

    if haystack.len() >= needle.len() {
        for index in 0..(haystack.len() - lead) {
            if &haystack[index..index + needle.len()] == &needle {
                return index as i32;
            }
        }
    }

    -1
}

#[cfg(test)]
mod tests;
