pub fn permute(mut numbers: Vec<i32>) -> Vec<Vec<i32>> {
    if numbers.is_empty() {
        return vec![];
    }

    if numbers.len() == 1 {
        return vec![numbers];
    }

    let mut permutations = vec![];

    let number = numbers.pop().unwrap();

    for sub_permutation in permute(numbers) {
        // Now insert our anchor number into each sub_permutation.
        for sub_permutation_index in 0..(sub_permutation.len() + 1) {
            let mut new_permutation = sub_permutation.clone();
            new_permutation.insert(sub_permutation_index, number);
            permutations.push(new_permutation);
        }
    }

    permutations.sort();
    permutations.dedup();
    permutations
}

#[cfg(test)]
mod tests;
