use std::collections::HashSet;

fn as_priority(char: char) -> u64 {
    match char {
        'a'..='z' => char as u64 - 96,
        'A'..='Z' => char as u64 - 38,
        _ => panic!("{}", format!("Unexpected character '{char}' in the input")),
    }
}

pub fn part_one(input: String) -> u64 {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(sack1, sack2)| {
            let sack1 = HashSet::<_>::from_iter(sack1.chars());
            let sack2 = HashSet::<_>::from_iter(sack2.chars());

            as_priority(sack1.intersection(&sack2).next().unwrap().clone())
        })
        .sum()
}

pub fn part_two(input: String) -> u64 {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|w| {
            let sack1 = HashSet::<_>::from_iter(w[0].chars());
            let sack2 = HashSet::<_>::from_iter(w[1].chars());
            let sack3 = HashSet::<_>::from_iter(w[2].chars());

            as_priority(
                HashSet::<_>::from_iter(sack1.intersection(&sack2).map(|&x| x.clone()))
                    .intersection(&sack3)
                    .next()
                    .unwrap()
                    .clone(),
            )
        })
        .sum()
}
