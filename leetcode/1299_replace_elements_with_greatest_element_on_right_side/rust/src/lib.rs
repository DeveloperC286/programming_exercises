pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
    (0..(arr.len()))
        .map(|index| *arr[index + 1..].iter().max().unwrap_or(&-1))
        .collect()
}

#[cfg(test)]
mod tests;
