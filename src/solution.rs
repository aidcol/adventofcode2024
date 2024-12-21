mod day01;
mod day02;
mod day03;

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

pub fn day02() {
    let path = "src/input/day02.txt";
    let reports = day02::parse_input(path);
    
    let num_safe = day02::get_num_safe(&reports);
    println!("The solution to Part 1 is: {}", num_safe);

    let num_safe_dampened = day02::get_num_safe_dampened(&reports);
    println!("The solution to Part 2 is: {}", num_safe_dampened);
}

pub fn day03() {
    let path = "src/input/day03.txt";

    let result = day03::parse_input(path);
    println!("The solution to Part 1 is {}", result);

    let result_with_conditionals = day03::parse_input_with_conditionals(path);
    println!("The solution to Part 2 is {}", result_with_conditionals);
}
