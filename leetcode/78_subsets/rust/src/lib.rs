use std::collections::HashSet;

pub fn subsets(generating_from: Vec<i32>) -> Vec<Vec<i32>> {
    let desired_length = generating_from.len();
    let upper_bound = 2usize.pow(desired_length as u32);

    let mut subsets = HashSet::new();

    for i in 0..upper_bound {
        let binary_string = format!("{i:0desired_length$b}");
        let mut set = vec![];

        for (index, binary_char) in binary_string.chars().enumerate() {
            if binary_char == '1' {
                set.push(generating_from[index]);
            }
        }

        set.sort_unstable();
        subsets.insert(set);
    }

    subsets.into_iter().collect()
}

#[cfg(test)]
mod tests;
