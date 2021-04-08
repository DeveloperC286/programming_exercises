pub fn subsets(generating_from: Vec<i32>) -> Vec<Vec<i32>> {
    let generating_from_len = generating_from.len();
    let upper_bound = 2usize.pow(generating_from_len as u32);

    let mut subsets = vec![];

    for i in 0..upper_bound {
        let binary_string = get_padded_binary_string(i, generating_from_len);
        let mut set = vec![];

        for (index, binary_char) in binary_string.chars().enumerate() {
            if binary_char == '1' {
                set.push(generating_from[index]);
            }
        }

        set.sort_unstable();
        subsets.push(set);
    }

    subsets.sort_unstable();
    subsets
}

fn get_padded_binary_string(i: usize, desired_length: usize) -> String {
    let mut string = format!("{:b}", i);

    while string.len() < desired_length {
        string = format!("0{}", string);
    }

    string
}

#[cfg(test)]
mod tests;
