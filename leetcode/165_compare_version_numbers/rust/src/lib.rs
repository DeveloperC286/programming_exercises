pub fn compare_version(version_1: String, version_2: String) -> i32 {
    let version_1_size = version_1.split('.').count();
    let version_2_size = version_2.split('.').count();

    let n = match version_1_size > version_2_size {
        true => version_1_size,
        false => version_2_size,
    };

    for i in 0..n {
        let version_1_segment = get_version_segment(&version_1, i);
        let version_2_segment = get_version_segment(&version_2, i);

        if version_1_segment != version_2_segment {
            return if version_1_segment > version_2_segment {
                1
            } else {
                -1
            };
        }
    }

    0
}

fn get_version_segment(version: &str, nth: usize) -> usize {
    version
        .split('.')
        .nth(nth)
        .map(|segment| segment.parse::<usize>().unwrap())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests;
