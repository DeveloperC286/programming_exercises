use std::collections::HashSet;

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    if n == 0 {
        return vec![];
    }

    if n == 1 {
        return vec!["()".to_string()];
    }

    let mut permutations = HashSet::new();

    for child_permutation in generate_parenthesis(n - 1) {
        permutations.insert(format!("({})", child_permutation));

        for inserting_at in 0..child_permutation.len() {
            permutations.insert(format!(
                "{}(){}",
                &child_permutation[0..inserting_at],
                &child_permutation[inserting_at..child_permutation.len()],
            ));
        }
    }

    let mut permutations_vec: Vec<String> = permutations.into_iter().collect();
    permutations_vec.sort();

    permutations_vec
}

#[cfg(test)]
mod tests;
