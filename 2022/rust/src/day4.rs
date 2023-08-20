struct Elf {
    pub start: u64,
    pub end: u64,
}

fn parse_line(line: &str) -> (Elf, Elf) {
    let (elf1, elf2) = line.split_once(",").unwrap();
    let (elf1_start, elf1_end) = elf1.split_once("-").unwrap();
    let (elf2_start, elf2_end) = elf2.split_once("-").unwrap();

    (
        Elf { start: elf1_start.parse::<u64>().unwrap(), end: elf1_end.parse::<u64>().unwrap()},
        Elf { start: elf2_start.parse::<u64>().unwrap(), end: elf2_end.parse::<u64>().unwrap()},
    )
}

pub fn part_one(input: String) -> u64 {
    input
        .lines()
        .map(|line| {
            let (e1, e2) = parse_line(line);

            ((e1.start >= e2.start && e1.end <= e2.end) || (e2.start >= e1.start && e2.end <= e1.end)) as u64
        })
        .sum()
}

pub fn part_two(input: String) -> u64 {
    input
        .lines()
        .map(|line| {
            let (e1, e2) = parse_line(line);

            (e1.start.max(e2.start) <= e1.end.min(e2.end)) as u64
        })
        .sum()
}
