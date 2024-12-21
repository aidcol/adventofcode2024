use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_input(path: &str) -> Vec<Vec<i32>> {
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

pub fn is_safe(report: &Vec<i32>) -> bool {
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

pub fn is_safe_dampened(report: &Vec<i32>) -> bool {
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

pub fn get_num_safe(reports: &Vec<Vec<i32>>) -> usize {
    reports
        .iter()
        .filter(|report| is_safe(report))
        .count()
}

pub fn get_num_safe_dampened(reports: &Vec<Vec<i32>>) -> usize {
    reports
        .iter()
        .filter(|report| is_safe_dampened(report))
        .count()
}

#[cfg(test)]
mod tests;
