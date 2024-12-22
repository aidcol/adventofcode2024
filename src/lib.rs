mod solution;

pub fn run(day: u8) {
    match day {
        1 => {
            let day01 = solution::day01::Day01;
            solution::run_solution(&day01);
        },
        2 => {
            let day02 = solution::day02::Day02;
            solution::run_solution(&day02);
        },
        3 => {
            let day03 = solution::day03::Day03;
            solution::run_solution(&day03);
        },
        _ => println!("Day {} is not implemented yet", day),
    };
}
