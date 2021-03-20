extern crate nalgebra;
use nalgebra::DMatrix;

use std::collections::HashSet;

pub fn zero_matrix(matrix: DMatrix<i32>) -> DMatrix<i32> {
    let mut modifying_matrix: Vec<i32> = matrix.iter().copied().collect();
    let number_of_columns = matrix.ncols();
    let number_of_rows = matrix.nrows();

    let (rows_clearing, columns_clearing) = get_rows_columns_clearing(matrix);

    modifying_matrix = clear_columns(
        columns_clearing,
        modifying_matrix,
        number_of_rows,
        number_of_columns,
    );
    modifying_matrix = clear_rows(rows_clearing, modifying_matrix, number_of_columns);

    DMatrix::from_vec(number_of_rows, number_of_columns, modifying_matrix)
}

fn clear_columns(
    columns_clearing: HashSet<i32>,
    modifying_matrix: Vec<i32>,
    number_of_rows: usize,
    number_of_columns: usize,
) -> Vec<i32> {
    let mut modifying_matrix: Vec<i32> = modifying_matrix;

    for column in columns_clearing.iter() {
        for row in 0..number_of_rows {
            let updating = row * number_of_columns + (*column as usize);
            modifying_matrix[updating] = 0
        }
    }

    modifying_matrix
}

fn clear_rows(
    rows_clearing: HashSet<i32>,
    modifying_matrix: Vec<i32>,
    number_of_columns: usize,
) -> Vec<i32> {
    let mut modifying_matrix: Vec<i32> = modifying_matrix;

    for row in rows_clearing.iter() {
        for column in 0..number_of_columns {
            let updating = (*row as usize) * number_of_columns + column;
            modifying_matrix[updating] = 0
        }
    }

    modifying_matrix
}

fn get_rows_columns_clearing(matrix: DMatrix<i32>) -> (HashSet<i32>, HashSet<i32>) {
    let mut row = 0;
    let mut column = 0;

    let mut rows_clearing = HashSet::new();
    let mut columns_clearing = HashSet::new();

    for value in matrix.iter() {
        if *value == 0 {
            rows_clearing.insert(row);
            columns_clearing.insert(column);
        }

        row += column / (matrix.ncols() as i32 - 1);
        column = (column + 1) % matrix.ncols() as i32;
    }

    (rows_clearing, columns_clearing)
}

#[cfg(test)]
mod tests;
