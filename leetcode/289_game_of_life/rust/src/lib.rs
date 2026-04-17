pub fn game_of_life(board: &mut [Vec<i32>]) {
    let height = board.len();
    let width = board[0].len();

    // Calculate all the changes
    let boards_number_of_neighbors: Vec<Vec<usize>> = (0..height)
        .map(|y| {
            (0..width)
                .map(|x| get_number_of_neighbors(board, height, width, y, x))
                .collect()
        })
        .collect();

    // Make all the changes
    for y in 0..height {
        for x in 0..width {
            let n = boards_number_of_neighbors[y][x];
            board[y][x] = match (board[y][x], n) {
                (1, 2 | 3) | (0, 3) => 1,
                _ => 0,
            };
        }
    }
}

fn get_number_of_neighbors(
    board: &[Vec<i32>],
    height: usize,
    width: usize,
    y: usize,
    x: usize,
) -> usize {
    let mut number_of_neighbors = 0;

    // Horizontal neighbors
    if y > 0 {
        if board[y - 1][x] == 1 {
            number_of_neighbors += 1;
        }

        // Diagonal neighbors
        if x > 0 && board[y - 1][x - 1] == 1 {
            number_of_neighbors += 1;
        }

        if x < (width - 1) && board[y - 1][x + 1] == 1 {
            number_of_neighbors += 1;
        }
    }

    if y < (height - 1) {
        if board[y + 1][x] == 1 {
            number_of_neighbors += 1;
        }

        // Diagonal neighbors
        if x > 0 && board[y + 1][x - 1] == 1 {
            number_of_neighbors += 1;
        }

        if x < (width - 1) && board[y + 1][x + 1] == 1 {
            number_of_neighbors += 1;
        }
    }

    // Vertical neighbors
    if x > 0 && board[y][x - 1] == 1 {
        number_of_neighbors += 1;
    }

    if x < (width - 1) && board[y][x + 1] == 1 {
        number_of_neighbors += 1;
    }

    number_of_neighbors
}

#[cfg(test)]
mod tests;
