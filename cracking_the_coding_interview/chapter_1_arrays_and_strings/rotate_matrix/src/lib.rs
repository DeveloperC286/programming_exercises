extern crate nalgebra;
use nalgebra::DMatrix;

pub fn rotate_matrix(matrix: DMatrix<i32>) -> DMatrix<i32> {
    if matrix.nrows() != matrix.ncols() {
        return matrix;
    }

    let n = matrix.nrows();

    let mut row = 1;
    let mut column = 1;

    let mut modifying_matrix: Vec<i32> = matrix.iter().copied().collect();

    for value in matrix.iter() {
        let (new_row, new_column) = calculate_new_position(row, column, n);

        //move value to new position
        modifying_matrix = update_matrix_vector(modifying_matrix, n, new_row, new_column, *value);

        let (new_row, new_column) = increment_position(row, column, n);
        row = new_row;
        column = new_column;
    }

    DMatrix::from_vec(n, n, modifying_matrix)
}

fn calculate_new_position(row: i32, column: i32, n: usize) -> (i32, i32) {
    (column, (n as i32 + 1) - row)
}

fn increment_position(row: i32, column: i32, n: usize) -> (i32, i32) {
    (row + (column / n as i32), (column % n as i32) + 1)
}

fn update_matrix_vector(
    vector: Vec<i32>,
    n: usize,
    row: i32,
    column: i32,
    new_value: i32,
) -> Vec<i32> {
    //index to update at
    let updating = (((row - 1) as usize) * n) + ((column - 1) as usize);

    //updated matrix vector
    let mut updated_vector: Vec<i32> = vector;
    updated_vector[updating] = new_value;

    updated_vector
}

#[cfg(test)]
mod tests;
