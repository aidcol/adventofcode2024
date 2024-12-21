use super::*;

#[test]
fn test_parse_input() {
    let path = "src/solution/day01/test.txt";
    let list1 = vec![3, 4, 2, 1, 3, 3];
    let list2 = vec![4, 3, 5, 3, 9, 3];
    let expected = (list1, list2);
    assert_eq!(parse_input(path), expected);
}

#[test]
fn test_find_distance() {
    let list1 = vec![3, 4, 2, 1, 3, 3];
    let list2 = vec![4, 3, 5, 3, 9, 3];
    let expected = 11;
    assert_eq!(find_distance(&list1, &list2), expected);
}

#[test]
fn test_find_similarity() {
    let list1 = vec![3, 4, 2, 1, 3, 3];
    let list2 = vec![4, 3, 5, 3, 9, 3];
    let expected = 31;
    assert_eq!(find_similarity(&list1, &list2), expected);
}
