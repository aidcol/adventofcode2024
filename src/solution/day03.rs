use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn parse_line(line: &str) -> i32 {
    let re: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let captures = re
        .captures_iter(line)
        .map(|c| c.extract());

    let mut ops = vec![];
    for (_, [left, right]) in captures {
        let left = left.parse::<i32>().unwrap();
        let right = right.parse::<i32>().unwrap();
        ops.push((left, right));
    }

    ops.iter()
        .map(|(left, right)| left * right)
        .sum()
}

fn parse_input(path: &str) -> i32 {
    let file = File::open(path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut result = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        result += parse_line(&line);
    }

    result
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Mul(i32, i32),
    Do,
    Dont,
}

fn parse_line_with_conditionals(line: &str) -> Vec<Operation> {
    let pattern = r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)";
    let re = Regex::new(pattern).unwrap();

    let mut ops = vec![];
    for capture in re.captures_iter(line) {
        let op = capture.get(0).unwrap().as_str();
        match op {
            "do()" => ops.push(Operation::Do),
            "don't()" => ops.push(Operation::Dont),
            _ => {
                let left = capture
                    .get(1)
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
                let right = capture
                    .get(2)
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap();
                ops.push(Operation::Mul(left, right));
            }
        }
    }
    
    ops
}

enum State {
    Enabled,
    Disabled,
}

fn parse_input_with_conditionals(path: &str) -> i32 {
    let file = File::open(path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut ops = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        ops.extend(parse_line_with_conditionals(&line));
    }

    let mut result = 0;
    let mut state = State::Enabled;
    for op in ops {
        match op {
            Operation::Mul(left, right) => {
                if let State::Enabled = state {
                    result += left * right;
                }
            }
            Operation::Do => {
                state = State::Enabled;
            }
            Operation::Dont => {
                state = State::Disabled;
            }
        }
    }

    result
}

pub struct Day03;

impl super::Solution for Day03 {
    fn get_part_one(&self) -> String {
        let result = parse_input("src/input/day03.txt");
        result.to_string()
    }

    fn get_part_two(&self) -> String {
        let result = parse_input_with_conditionals("src/input/day03.txt");
        result.to_string()
    }
}

#[cfg(test)]
mod tests;
