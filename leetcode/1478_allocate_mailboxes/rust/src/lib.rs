pub fn min_distance(houses: Vec<i32>, number_of_mailboxes: i32) -> i32 {
    if number_of_mailboxes == 1 {
        return single_mailbox_min_distance(houses);
    }

    get_all_partitions(houses)
        .iter()
        .filter(|partition| partition.len() == number_of_mailboxes as usize)
        .map(|partition| {
            partition
                .iter()
                .map(|subset| single_mailbox_min_distance(subset.clone()))
                .sum()
        })
        .min()
        .unwrap()
}

fn single_mailbox_min_distance(houses: Vec<i32>) -> i32 {
    return match houses.len() {
        1 => 0,
        2 => {
            let middle = (houses[0] + houses[1]) / 2;
            (middle - std::cmp::min(houses[0], houses[1]))
                + (std::cmp::max(houses[0], houses[1]) - middle)
        }
        _ => {
            let first_house = *houses.iter().min().unwrap();
            let last_hosue = *houses.iter().max().unwrap();

            ((first_house + 1)..last_hosue)
                .map(|mailbox_position| get_total_distance(&houses, mailbox_position))
                .min()
                .unwrap()
        }
    };
}

fn get_total_distance(houses: &Vec<i32>, mailbox_position: i32) -> i32 {
    houses
        .iter()
        .map(|house| {
            std::cmp::max(*house, mailbox_position) - std::cmp::min(*house, mailbox_position)
        })
        .sum()
}

fn get_all_partitions(generate_from: Vec<i32>) -> Vec<Vec<Vec<i32>>> {
    let mut sorted_partitions = get_all_partitions_internal(generate_from);
    sorted_partitions.sort();
    sorted_partitions.dedup();
    sorted_partitions
}

fn get_all_partitions_internal(generate_from: Vec<i32>) -> Vec<Vec<Vec<i32>>> {
    if generate_from.len() == 1 {
        return vec![vec![generate_from]];
    }

    let mut partitions: Vec<Vec<Vec<i32>>> = vec![];

    for generate_from_index in 0..generate_from.len() {
        //pin partion and get all perms of
        let mut working_generate_from = generate_from.clone();
        let pinned_partition = working_generate_from.remove(generate_from_index);

        for sub_partitions in get_all_partitions_internal(working_generate_from) {
            let mut partition = sub_partitions.clone();
            partition.push(vec![pinned_partition]);
            let mut sorted_partition = vec![];

            for mut sub_partition in partition {
                sub_partition.sort();
                sorted_partition.push(sub_partition);
            }

            sorted_partition.sort();
            partitions.push(sorted_partition);

            for sub_sub_partitions_index in 0..sub_partitions.len() {
                let mut partition = sub_partitions.clone();
                partition[sub_sub_partitions_index].push(pinned_partition);

                let mut sorted_partition = vec![];

                for mut sub_partition in partition {
                    sub_partition.sort();
                    sorted_partition.push(sub_partition);
                }

                sorted_partition.sort();
                partitions.push(sorted_partition);
            }
        }
    }

    partitions.sort();
    partitions.dedup();
    partitions
}

#[cfg(test)]
mod tests;
