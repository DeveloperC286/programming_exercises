use std::collections::HashMap;
use std::collections::VecDeque;

#[allow(dead_code)]
struct FirstUnique {
    unique_values: VecDeque<i32>,
    value_counts: HashMap<i32, usize>,
}

#[allow(dead_code)]
impl FirstUnique {
    fn new(values: Vec<i32>) -> Self {
        let mut first_unique = FirstUnique {
            unique_values: VecDeque::new(),
            value_counts: HashMap::new(),
        };

        values.into_iter().for_each(|value| first_unique.add(value));

        first_unique
    }

    fn show_first_unique(&mut self) -> i32 {
        *self.unique_values.front().unwrap_or(&-1)
    }

    fn add(&mut self, value: i32) {
        let value_count = self.value_counts.entry(value).or_insert(0);
        *value_count += 1;

        match value_count {
            1 => {
                self.unique_values.push_back(value);
            }
            2 => {
                // First time it is not unique anymore so remove from unique_values.
                if let Some(index) = self.unique_values.iter().position(|&r| r == value) {
                    self.unique_values.remove(index);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests;
