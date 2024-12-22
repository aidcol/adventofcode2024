pub mod day01;
pub mod day02;
pub mod day03;

use crate::solution::day01::Day01;

pub trait Solution {
    fn get_part_one(&self) -> String;
    fn get_part_two(&self) -> String;
}

pub fn get_solution(day: &impl Solution) {
    let part_one = day.get_part_one();
    let part_two = day.get_part_two();

    println!("The solution to Part 1 is: \n{}", part_one);
    println!("The solution to Part 2 is: \n{}", part_two);
}
