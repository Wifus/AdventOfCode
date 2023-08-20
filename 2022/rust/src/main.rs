pub mod day1;

fn load_input(day: u8) -> String {
    std::fs::read_to_string(format!("../input/{}.txt", day)).expect(&format!("No input for Day {} found!", day))
}

fn main() {
    let input = load_input(1);

    dbg!(day1::part_one(input.clone()));
    dbg!(day1::part_two(input.clone()));
}
