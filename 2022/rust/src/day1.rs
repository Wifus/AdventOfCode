pub fn part_one(input: String) -> u64 {
    input
        .split("\n\n")
        .map(|values| values.lines().map(|v| v.parse::<u64>().unwrap()).sum())
        .max()
        .unwrap_or(0)
}

pub fn part_two(input: String) -> u64 {
    let mut counts = input
        .split("\n\n")
        .map(|values| values.lines().map(|v| v.parse::<u64>().unwrap()).sum())
        .collect::<Vec<_>>();

    counts.sort();
    counts.iter().rev().take(3).sum()
}
