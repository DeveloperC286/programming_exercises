use std::collections::VecDeque;

pub fn num_islands(mut map: Vec<Vec<char>>) -> i32 {
    let mut number_of_islands = 0;

    let max_height = map.len();
    let max_width = map[0].len();

    for height_index in 0..max_height {
        for width_index in 0..max_width {
            if map[height_index][width_index] == '1' {
                number_of_islands += 1;

                let mut neighbours: VecDeque<(usize, usize)> = VecDeque::new();
                map[height_index][width_index] = '0';
                neighbours.push_back((height_index, width_index));

                // BFS and remove all attached.
                while !neighbours.is_empty() {
                    let (height_index, width_index) = neighbours.pop_front().unwrap();

                    // Neighbour on Right.
                    if width_index < (max_width - 1) && map[height_index][width_index + 1] == '1' {
                        map[height_index][width_index + 1] = '0';
                        neighbours.push_back((height_index, width_index + 1));
                    }

                    // Neighbour on left.
                    if width_index > 0 && map[height_index][width_index - 1] == '1' {
                        map[height_index][width_index - 1] = '0';
                        neighbours.push_back((height_index, width_index - 1));
                    }

                    // Neighbour above.
                    if height_index > 0 && map[height_index - 1][width_index] == '1' {
                        map[height_index - 1][width_index] = '0';
                        neighbours.push_back((height_index - 1, width_index));
                    }

                    // Neighbour below.
                    if height_index < (max_height - 1) && map[height_index + 1][width_index] == '1'
                    {
                        map[height_index + 1][width_index] = '0';
                        neighbours.push_back((height_index + 1, width_index));
                    }
                }
            }
        }
    }

    number_of_islands
}

#[cfg(test)]
mod tests;
