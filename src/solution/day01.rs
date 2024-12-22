use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day01;

impl Day01 {
    fn parse_input(path: &str) -> (Vec<i32>, Vec<i32>) {
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
    
    fn find_distance(list1: &Vec<i32>, list2: &Vec<i32>) -> String {
        let mut list1_clone = list1.clone();
        let mut list2_clone = list2.clone();
        
        list1_clone.sort();
        list2_clone.sort();
    
        let distance: i32 = std::iter::zip(list1_clone, list2_clone)
            .map(|(a, b)| (a - b).abs())
            .sum();
    
        distance.to_string()
    }
    
    fn find_similarity(list1: &Vec<i32>, list2: &Vec<i32>) -> String {
        let similarity: i32 = list1
            .iter()
            .map(|num| {
                num * list2
                    .iter()
                    .filter(|x| &num == x)
                    .count() as i32
            })
            .sum();
    
        similarity.to_string()
    }
}

impl super::Solution for Day01 {
    fn get_part_one(&self) -> String {
        let path = "src/input/day01.txt";
        let (mut list1, mut list2) = Day01::parse_input(path);
    
        list1.sort();
        list2.sort();
    
        let distance = Day01::find_distance(&list1, &list2);
        distance
    }

    fn get_part_two(&self) -> String {
        let path = "src/input/day01.txt";
        let (list1, list2) = Day01::parse_input(path);
        let similarity = Day01::find_similarity(&list1, &list2);
        similarity
    }
}

#[cfg(test)]
mod tests;
