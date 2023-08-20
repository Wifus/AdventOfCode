pub fn part_one(input: String) -> u64 {
    let parsed = input
        .split("\r\n")
        .map(|s| s.parse::<u64>())
        .collect::<Vec<_>>();

    let mut max = 0;
    let mut counter = 0;

    for row in parsed {
        match row {
            Ok(value) => {
                counter += value;     
            },
            Err(_) => {
                max = std::cmp::max(max, counter);
                counter = 0;
            },
        }
    }

    max
}

pub fn part_two(input: String) -> u64 {
    let parsed = input
        .split("\r\n")
        .map(|s| s.parse::<u64>())
        .collect::<Vec<_>>();

    let mut counts = vec![0];

    for row in parsed {
        match row {
            Ok(value) => {
                let size = counts.len();
                *counts.get_mut(size - 1).unwrap() += value;
            },
            Err(_) => {
                counts.push(0);
            },
        }
    }

    counts.sort();
    counts.iter().rev().take(3).sum()
}
