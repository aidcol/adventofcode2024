use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input(path: &str) -> Vec<Vec<i32>> {
    let mut reports = vec![];
    
    let file = File::open(path).expect("Cannot open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        let report = line
            .split_whitespace()
            .map(|level| level.parse::<i32>().unwrap())
            .collect();
        
        reports.push(report);
    }

    reports
}

enum Direction {
    Increasing,
    Decreasing,
}

fn is_safe(report: &Vec<i32>) -> bool {
    let mut direction: Option<Direction> = None;
    let mut prev = report[0];

    for &level in report.iter().skip(1) {
        let difference = level - prev;

        match difference.signum() {
            1 => {
                match direction {
                    Some(Direction::Increasing) => (),
                    Some(Direction::Decreasing) => return false,
                    None => direction = Some(Direction::Increasing),
                }
            },
            -1 => {
                match direction {
                    Some(Direction::Increasing) => return false,
                    Some(Direction::Decreasing) => (),
                    None => direction = Some(Direction::Decreasing),
                }
            },
            0 => return false,
            _ => panic!("Invalid direction"),
        }

        if difference.abs() > 3 {
            return false;
        }
        
        prev = level;
    }
    
    true
}

fn is_safe_dampened(report: &Vec<i32>) -> bool {
    if is_safe(report) {
        return true;
    }

    for index in 0..report.len() {
        let mut new_report = report.clone();
        new_report.remove(index);

        if is_safe(&new_report) {
            return true;
        }
    }

    false
}

fn get_num_safe(reports: &Vec<Vec<i32>>) -> usize {
    reports
        .iter()
        .filter(|report| is_safe(report))
        .count()
}

fn get_num_safe_dampened(reports: &Vec<Vec<i32>>) -> usize {
    reports
        .iter()
        .filter(|report| is_safe_dampened(report))
        .count()
}

pub struct Day02;

impl super::Solution for Day02 {
    fn get_part_one(&self) -> String {
        let reports = parse_input("src/input/day02.txt");
        let num_safe = get_num_safe(&reports);

        num_safe.to_string()
    }

    fn get_part_two(&self) -> String {
        let reports = parse_input("src/input/day02.txt");
        let num_safe = get_num_safe_dampened(&reports);

        num_safe.to_string()
    }
}

#[cfg(test)]
mod tests;
