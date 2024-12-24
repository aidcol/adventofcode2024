use super::*;

#[test]
fn test_parse_input() {
    let path = "src/solution/day04/test.txt";
    let expected = vec![
        vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
        vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
        vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
        vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
        vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
        vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
        vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
        vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
        vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
        vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
    ];
    let result = parse_input(path);
    assert_eq!(result, expected);
}

#[test]
fn test_dfs_gets_direction() {
    let path = "src/solution/day04/test.txt";
    let grid = parse_input(path);
    let result = dfs(&grid, 0, 4);
    assert_eq!(result, 1);
}

#[test]
fn test_dfs_gets_multiple_directions() {
    let path = "src/solution/day04/test.txt";
    let grid = parse_input(path);
    let result = dfs(&grid, 9, 3);
    assert_eq!(result, 2);
}

#[test]
fn test_search() {
    let path = "src/solution/day04/test.txt";
    let grid = parse_input(path);
    let result = search(grid);
    assert_eq!(result, 18);
}