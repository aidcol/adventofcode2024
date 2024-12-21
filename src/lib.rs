mod solution;

pub fn run(day: u8) {
    match day {
        1 => solution::day01(),
        _ => println!("Day {} is not implemented yet", day),
    }
}
