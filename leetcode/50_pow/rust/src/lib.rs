pub fn my_pow(x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    }

    if n == 1 {
        return x;
    }

    if n < 0 {
        return 1.0 / my_pow(x, -n);
    }

    x * my_pow(x, n - 1)
}

#[cfg(test)]
mod tests;
