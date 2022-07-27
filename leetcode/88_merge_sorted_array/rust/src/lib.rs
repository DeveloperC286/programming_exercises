pub fn merge(
    merge_into: &mut Vec<i32>,
    merge_into_len: i32,
    to_merge: &mut [i32],
    to_merge_len: i32,
) {
    let merge_into_len: usize = merge_into_len as usize;
    let to_merge_len: usize = to_merge_len as usize;

    merge_into.truncate(merge_into_len);

    let mut merge_into_index: usize = 0;
    let mut to_merge_index: usize = 0;

    while to_merge_index < to_merge_len {
        if merge_into_index == merge_into.len()
            || to_merge[to_merge_index] <= merge_into[merge_into_index]
        {
            merge_into.insert(merge_into_index, to_merge[to_merge_index]);
            to_merge_index += 1;
        } else {
            merge_into_index += 1;
        }
    }
}

#[cfg(test)]
mod tests;
