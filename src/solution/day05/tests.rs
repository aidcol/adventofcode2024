use super::*;

#[test]
fn test_parse_input_from_file() {
    let path = "src/solution/day05/test.txt";
    let rule_strs = vec![
        "47|53", "97|13", "97|61", "97|47", "75|29", "61|13", "75|53",
        "29|13", "97|29", "53|29", "61|53", "97|53", "61|29", "47|13",
        "75|47", "97|75", "47|61", "75|61", "47|29", "75|13", "53|13",
    ];
    let rule_lines = rule_strs.iter().map(|s| s.to_string()).collect();
    
    let update_strs = vec![
        "75,47,61,53,29",
        "97,61,53,29,13",
        "75,29,13",
        "75,97,47,61,53",
        "61,13,29",
        "97,13,75,29,47",
    ];
    let update_lines = update_strs.iter().map(|s| s.to_string()).collect();

    let result = parse_input_from_file(path);
    assert_eq!(result, (rule_lines, update_lines));
}

#[test]
fn test_is_update_valid() {
    let rules = vec![
        "47|53", "97|13", "97|61", "97|47", "75|29", "61|13", "75|53",
        "29|13", "97|29", "53|29", "61|53", "97|53", "61|29", "47|13",
        "75|47", "97|75", "47|61", "75|61", "47|29", "75|13", "53|13",
    ];
    let rules = rules.iter().map(|s| s.to_string()).collect();
    let rules = parse_rules(&rules);
    
    let valid_update = vec![75, 47, 61, 53, 29];
    let invalid_update = vec![61, 13, 29];

    assert!(is_update_valid(&valid_update, &rules));
    assert!(!is_update_valid(&invalid_update, &rules));
}

#[test]
fn test_sum_middle_pages() {
    let (rule_lines, update_lines) = parse_input_from_file("src/solution/day05/test.txt");

    let rules = parse_rules(&rule_lines);
    let updates = parse_updates(&update_lines);
    let result = sum_middle_pages(&updates, &rules);
    let expected = 143;
    assert_eq!(result, expected);
}