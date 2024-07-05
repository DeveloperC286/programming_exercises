pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut index = 0;

    while (index + 1) < nums.len() {
        if nums[index] == nums[index + 1] {
            nums.remove(index);
        } else {
            index += 1;
        }
    }

    nums.len() as i32
}

#[cfg(test)]
mod tests;
