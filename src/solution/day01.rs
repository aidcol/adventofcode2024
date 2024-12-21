use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_input(path: &str) -> (Vec<i32>, Vec<i32>) {
    let mut list1 = vec![];
    let mut list2 = vec![];

    let file = File::open(path).expect("Cannot open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut items = line.split_whitespace();

        let num1: i32 = items
            .next()
            .unwrap()
            .parse()
            .expect("Cannot parse number");

        let num2: i32 = items
            .next()
            .unwrap()
            .parse()
            .expect("Cannot parse number");

        list1.push(num1);
        list2.push(num2);
    }

    (list1, list2)
}

pub fn find_distance(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut list1_clone = list1.clone();
    let mut list2_clone = list2.clone();
    
    list1_clone.sort();
    list2_clone.sort();

    let distance = std::iter::zip(list1_clone, list2_clone)
        .map(|(a, b)| (a - b).abs())
        .sum();

    distance
}

pub fn find_similarity(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let similarity = list1
        .iter()
        .map(|num| {
            num * list2
                .iter()
                .filter(|x| &num == x)
                .count() as i32
        })
        .sum();

    similarity
}

#[cfg(test)]
mod tests;
