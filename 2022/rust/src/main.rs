pub mod day1;

const DAY: u8 = 1;

fn main() {
    let input = std::fs::read_to_string(format!("../input/{}.txt", DAY)).expect("Input not found!");

    dbg!(day1::part_one(input.clone()));
    dbg!(day1::part_two(input.clone()));
}
