use std::cmp::min;
use std::collections::HashSet;

pub fn maximum_minimum_path(grid: Vec<Vec<i32>>) -> i32 {
    let mut to_search = vec![((0, 0), grid[0][0], HashSet::new())];

    let height = grid.len() - 1;
    let width = grid[0].len() - 1;
    // Do a basic all down then all left path.
    let mut maximum_minimum_path: i32 = min(
        (0..height).map(|y| grid[y][0]).min().unwrap(),
        (0..width).map(|x| grid[height][x]).min().unwrap(),
    );

    while let Some((point, current_maximum_minimum_path, mut visited)) = to_search.pop() {
        if !visited.contains(&point) {
            visited.insert(point);
            let (y, x) = point;
            if current_maximum_minimum_path > maximum_minimum_path {
                if x == width && y == height {
                    maximum_minimum_path = current_maximum_minimum_path;
                }

                get_neighbouring_values(&grid, height, width, x, y)
                    .into_iter()
                    .filter(|(point, _)| !visited.contains(point))
                    .for_each(|(point, value)| {
                        if value > maximum_minimum_path {
                            let cloned_visited = visited.clone();
                            to_search.push((
                                point,
                                min(value, current_maximum_minimum_path),
                                cloned_visited,
                            ));
                        }
                    });
            }
        }
    }

    maximum_minimum_path
}

fn get_neighbouring_values(
    grid: &[Vec<i32>],
    height: usize,
    width: usize,
    x: usize,
    y: usize,
) -> Vec<((usize, usize), i32)> {
    let mut neighbouring = vec![];

    // Horizontal neighbors
    if y > 0 {
        neighbouring.push(((y - 1, x), grid[y - 1][x]));
    }

    if y < height {
        neighbouring.push(((y + 1, x), grid[y + 1][x]));
    }

    // Vertical neighbors
    if x > 0 {
        neighbouring.push(((y, x - 1), grid[y][x - 1]));
    }

    if x < width {
        neighbouring.push(((y, x + 1), grid[y][x + 1]));
    }

    neighbouring
}

#[cfg(test)]
mod tests;
