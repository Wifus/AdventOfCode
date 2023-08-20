fn as_priority(char: char) -> u64 {
    match char {
        'a'..='z' => char as u64 - 96,
        'A'..='Z' => char as u64 - 38,
        _ => unreachable!(),
    }
}

pub fn part_one(input: String) -> u64 {
    input
        .lines()
        .map(|line: &str| {
            let (sack1, sack2) = line.split_at(line.len() / 2);

            as_priority(sack1.chars().filter(|&c| sack2.contains(c)).next().unwrap())
        })
        .sum()
}

pub fn part_two(input: String) -> u64 {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            as_priority(
                chunk[0]
                    .chars()
                    .filter(|&c| chunk[1].contains(c) && chunk[2].contains(c))
                    .next()
                    .unwrap(),
            )
        })
        .sum()
}
