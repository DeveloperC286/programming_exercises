#[cfg(test)]
extern crate proptest;

pub fn largest_number(searching: &[i32]) -> Option<i32> {
    fn internal_largest_number<'a>(
        mut searching: impl std::iter::Iterator<Item = &'a i32>,
    ) -> Option<i32> {
        match searching.next() {
            None => None,
            Some(head) => match internal_largest_number(searching) {
                None => Some(*head),
                Some(next_largest) => {
                    if *head > next_largest {
                        Some(*head)
                    } else {
                        Some(next_largest)
                    }
                }
            },
        }
    }

    internal_largest_number(searching.iter())
}

#[cfg(test)]
mod tests;
