use std::fs;
use std::collections::{HashMap, HashSet};

fn parse_input_from_file(path: &str) -> (Vec<String>, Vec<String>) {
    let content = fs::read_to_string(path).expect("Failed to read input file.");
    let mut sections = content.splitn(2, "\n\n");
    let rule_section = sections.next().unwrap_or("");
    let update_section = sections.next().unwrap_or("");

    let rule_lines = rule_section.lines().map(|s| s.to_string()).collect();
    let update_lines = update_section.lines().map(|s| s.to_string()).collect();

    (rule_lines, update_lines)
}

fn parse_rules(rule_lines: &Vec<String>) -> HashMap<i32, HashSet<i32>> {
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    for line in rule_lines {
        let parts: Vec<&str> = line.trim().split('|').collect();
        if parts.len() == 2 {
            let x = parts[0].parse::<i32>().unwrap();
            let y = parts[1].parse::<i32>().unwrap();
            rules.entry(x).or_default().insert(y);
        }
    }
    rules
}

fn parse_updates(update_lines: &Vec<String>) -> Vec<Vec<i32>> {
    update_lines.iter()
        .map(|line| {
            line.split(',')
                .map(|s| s.trim().parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_update_valid(update: &[i32], rules: &HashMap<i32, HashSet<i32>>) -> bool {
    let mut seen: HashSet<i32> = HashSet::new();

    for &page in update {
        if let Some(after_pages) = rules.get(&page) {
            if seen.intersection(after_pages).next().is_some() {
                return false;
            }
        }
        seen.insert(page);
    }

    true
}

fn sum_middle_pages(updates: &[Vec<i32>], rules: &HashMap<i32, HashSet<i32>>) -> i32 {
    updates.iter()
        .filter(|update| is_update_valid(update, rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

pub struct Day05;

impl super::Solution for Day05 {
    fn get_part_one(&self) -> String {
        let path = "src/input/day05.txt";
        let (rule_lines, update_lines) = parse_input_from_file(path);
        let rules = parse_rules(&rule_lines);
        let updates = parse_updates(&update_lines);
        let result = sum_middle_pages(&updates, &rules);
        result.to_string()
    }

    fn get_part_two(&self) -> String {
        "Not implemented yet".to_string()
    }
}

#[cfg(test)]
mod tests;
