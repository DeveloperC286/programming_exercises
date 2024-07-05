use std::cmp::{max, min};

pub fn max_area(heights: Vec<i32>) -> i32 {
    let mut max_area = 0;

    let mut start = 0;
    let mut end = heights.len() - 1;

    while start != end {
        let width = (end - start) as i32;
        let height = min(heights[start], heights[end]);
        let area = width * height;
        max_area = max(max_area, area);

        if heights[start] > heights[end] {
            end -= 1;
        } else {
            start += 1;
        }
    }

    max_area
}

#[cfg(test)]
mod tests;
