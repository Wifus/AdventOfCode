pub fn part_one(input: String) -> u64 {
    input
        .lines()
        .map(|line| {
            let Some((elf1, elf2)) = line.split_once(",") else {unreachable!()};
            let Some((elf1_start, elf1_end)) = elf1.split_once("-") else {unreachable!()};
            let Some((elf2_start, elf2_end)) = elf2.split_once("-") else {unreachable!()};

            let (Ok(e1s), Ok(e1e), Ok(e2s), Ok(e2e)) = (
                elf1_start.parse::<u64>(),
                elf1_end.parse::<u64>(),
                elf2_start.parse::<u64>(),
                elf2_end.parse::<u64>(),
            ) else {unreachable!()};

            ((e1s >= e2s && e1e <= e2e) || (e2s >= e1s && e2e <= e1e)) as u64
        })
        .sum()
}

pub fn part_two(input: String) -> u64 {
    input
        .lines()
        .map(|line| {
            let Some((elf1, elf2)) = line.split_once(",") else {unreachable!()};
            let Some((elf1_start, elf1_end)) = elf1.split_once("-") else {unreachable!()};
            let Some((elf2_start, elf2_end)) = elf2.split_once("-") else {unreachable!()};

            let (Ok(e1s), Ok(e1e), Ok(e2s), Ok(e2e)) = (
                elf1_start.parse::<u64>(),
                elf1_end.parse::<u64>(),
                elf2_start.parse::<u64>(),
                elf2_end.parse::<u64>(),
            ) else {unreachable!()};

            if e1s >= e2s {
                (e2e >= e1s) as u64
            } else {
                (e1e >= e2s) as u64
            }
        })
        .sum()
}
