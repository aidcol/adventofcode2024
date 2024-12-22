mod solution;

pub fn run(day: u8) {
    match day {
        1 => {
            let day = solution::day01::Day01;
            solution::get_solution(&day);
        }
        _ => println!("Day {} is not implemented yet", day),
    }
}
