pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    match candies.iter().max() {
        Some(largest_amount_of_candies) => candies
            .iter()
            .map(|amount_of_candies| {
                (amount_of_candies + extra_candies) >= *largest_amount_of_candies
            })
            .collect(),
        None => vec![],
    }
}

#[cfg(test)]
mod tests;
