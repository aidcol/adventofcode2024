use super::*;

#[test]
fn test_parse_input() {
    let path = "src/solution/day02/test.txt";
    let reports = parse_input(path);
    let expected = vec![
        vec![7, 6, 4, 2, 1],
        vec![1, 2, 7, 8, 9],
        vec![9, 7, 6, 2, 1],
        vec![1, 3, 2, 4, 5],
        vec![8, 6, 4, 4, 1],
        vec![1, 3, 6, 7, 9],
    ];
    assert_eq!(reports, expected);
}

#[test]
fn test_is_safe_all_increasing() {
    let report = vec![1, 3, 6, 7, 9];
    assert_eq!(is_safe(&report), true);
}

#[test]
fn test_is_safe_all_decreasing() {
    let report = vec![7, 6, 4, 2, 1];
    assert_eq!(is_safe(&report), true);
}

#[test]
fn test_is_unsafe_large_increase() {
    let report = vec![1, 2, 7, 8, 9];
    assert_eq!(is_safe(&report), false);
}

#[test]
fn test_is_unsafe_large_decrease() {
    let report = vec![9, 7, 6, 2, 1];
    assert_eq!(is_safe(&report), false);
}

#[test]
fn test_is_unsafe_no_change() {
    let report = vec![8, 6, 4, 4, 1];
    assert_eq!(is_safe(&report), false);
}

#[test]
fn test_is_safe_dampened() {
    let report = vec![1, 3, 2, 4, 5];
    assert_eq!(is_safe_dampened(&report), true);
}

#[test]
fn test_is_unsafe_dampened() {
    let report = vec![1, 2, 7, 8, 9];
    assert_eq!(is_safe_dampened(&report), false);
}
