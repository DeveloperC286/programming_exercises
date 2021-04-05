use std::collections::HashSet;
use std::collections::VecDeque;

pub fn min_knight_moves(desired_x: i32, desired_y: i32) -> i32 {
    let mut positions: VecDeque<((i32, i32), i32)> = VecDeque::new();
    positions.push_back(((0, 0), 0));
    let mut already_tried_positions: HashSet<(i32, i32)> = HashSet::new();

    while let Some(((x, y), number_of_moves)) = positions.pop_front() {
        if x == desired_x && y == desired_y {
            return number_of_moves;
        }

        for (x_altering, y_altering) in &[
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2),
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
        ] {
            let new_position = (x + x_altering, y + y_altering);

            if !already_tried_positions.contains(&new_position) {
                already_tried_positions.insert(new_position);
                positions.push_back((new_position, number_of_moves + 1));
            }
        }
    }

    0
}

#[cfg(test)]
mod tests;
