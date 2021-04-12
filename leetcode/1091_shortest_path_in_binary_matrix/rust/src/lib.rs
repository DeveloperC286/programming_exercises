use std::collections::{HashSet, VecDeque};

pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
    let mut to_search: VecDeque<((usize, usize), i32)> = VecDeque::new();
    let mut already_searched: HashSet<(usize, usize)> = HashSet::new();

    if grid[0][0] == 0 {
        to_search.push_back(((0, 0), 1));
    }

    let height = grid.len() - 1;
    let width = grid[0].len() - 1;

    while !to_search.is_empty() {
        let (point, number_of_steps) = to_search.pop_front().unwrap();

        if !already_searched.contains(&point) {
            already_searched.insert(point);
            let (y, x) = point;

            if x == width && y == height {
                return number_of_steps;
            }

            // Horizontal neighbors
            if y > 0 {
                if grid[y - 1][x] == 0 {
                    to_search.push_back(((y - 1, x), number_of_steps + 1));
                }

                // Diagonal neighbors
                if x > 0 && grid[y - 1][x - 1] == 0 {
                    to_search.push_back(((y - 1, x - 1), number_of_steps + 1));
                }

                if x < width && grid[y - 1][x + 1] == 0 {
                    to_search.push_back(((y - 1, x + 1), number_of_steps + 1));
                }
            }

            if y < height {
                if grid[y + 1][x] == 0 {
                    to_search.push_back(((y + 1, x), number_of_steps + 1));
                }

                // Diagonal neighbors
                if x > 0 && grid[y + 1][x - 1] == 0 {
                    to_search.push_back(((y + 1, x - 1), number_of_steps + 1));
                }

                if x < width && grid[y + 1][x + 1] == 0 {
                    to_search.push_back(((y + 1, x + 1), number_of_steps + 1));
                }
            }

            // Vertical neighbors
            if x > 0 && grid[y][x - 1] == 0 {
                to_search.push_back(((y, x - 1), number_of_steps + 1));
            }

            if x < width && grid[y][x + 1] == 0 {
                to_search.push_back(((y, x + 1), number_of_steps + 1));
            }
        }
    }

    -1
}

#[cfg(test)]
mod tests;
