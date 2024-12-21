pub mod day01;

pub fn day01() {
    let path = "src/input/day01.txt";
    let (mut list1, mut list2) = day01::parse_input(path);

    list1.sort();
    list2.sort();

    let distance = day01::find_distance(&list1, &list2);
    println!("The solution to Part 1 is: {}", distance);

    let similarity = day01::find_similarity(&list1, &list2);
    println!("The solution to Part 2 is: {}", similarity);
}
