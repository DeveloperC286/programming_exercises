pub fn reverse(mut reversing: i32) -> i32 {
    let mut reversed = 0;

    while reversing != 0 {
        let next_digit = reversing % 10;
        reversing /= 10;

        // Overflow detection.
        if reversed > i32::MAX / 10 || (reversed == i32::MAX / 10 && next_digit > 7) {
            return 0;
        }
        if reversed < i32::MIN / 10 || (reversed == i32::MIN / 10 && next_digit < -8) {
            return 0;
        }

        reversed = reversed * 10 + next_digit;
    }

    reversed
}

#[cfg(test)]
mod tests;
