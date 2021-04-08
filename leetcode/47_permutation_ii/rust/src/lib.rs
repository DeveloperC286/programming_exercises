pub fn permute_unique(mut permute: Vec<i32>) -> Vec<Vec<i32>> {
    let mut permutations = vec![];
    permute_internal(&mut permute, &mut permutations, 0);

    permutations.sort();
    permutations.dedup();
    permutations
}

fn permute_internal(
    permuting: &mut Vec<i32>,
    permutations: &mut Vec<Vec<i32>>,
    mutating_index: usize,
) {
    if permuting.len() == mutating_index {
        permutations.push(permuting.clone());
    }

    for swapping_with_index in mutating_index..permuting.len() {
        permuting.swap(mutating_index, swapping_with_index);
        permute_internal(permuting, permutations, mutating_index + 1);
        permuting.swap(swapping_with_index, mutating_index);
    }
}

#[cfg(test)]
mod tests;
