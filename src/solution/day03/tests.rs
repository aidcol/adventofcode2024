use super::*;

#[test]
fn test_parse_mul_ops() {
    let line = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let result = parse_line(line);
    let expected = 161;
    assert_eq!(result, expected);
}

#[test]
fn test_parse_input() {
    let path = "src/solution/day03/test.txt";
    let result = parse_input(path);
    let expected = 161;
    assert_eq!(result, expected);
}

#[test]
fn test_parse_all_ops() {
    let line = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let result = parse_line_with_conditionals(line);
    assert_eq!(result, vec![
        Operation::Mul(2, 4),
        Operation::Dont,
        Operation::Mul(5, 5),
        Operation::Mul(11, 8),
        Operation::Do,
        Operation::Mul(8, 5),
    ]);
}

#[test]
fn test_parse_input_with_conditionals() {
    let path = "src/solution/day03/test.txt";
    let result = parse_input_with_conditionals(path);
    let expected = 48;
    assert_eq!(result, expected);
}
