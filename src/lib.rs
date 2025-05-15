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
        4 => {
            let day04 = solution::day04::Day04;
            solution::run_solution(&day04);
        },
        5 => {
            let day05 = solution::day05::Day05;
            solution::run_solution(&day05);
        },
        _ => println!("Day {} is not implemented yet", day),
    };
}
