pub fn product_except_self(mut numbers: Vec<i32>) -> Vec<i32> {
    let mut left_of_product = vec![1];

    for i in 0..(numbers.len() - 1) {
        left_of_product.push(left_of_product[i] * numbers[i]);
    }

    let mut right_of_product = vec![1];
    numbers.reverse();

    for i in 0..(numbers.len() - 1) {
        right_of_product.push(right_of_product[i] * numbers[i]);
    }
    right_of_product.reverse();

    let mut products = vec![];

    for i in 0..numbers.len() {
        products.push(left_of_product[i] * right_of_product[i]);
    }

    products
}

#[cfg(test)]
mod tests;
