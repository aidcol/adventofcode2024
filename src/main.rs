use std::io;

fn main() {
    loop {
        println!("Enter the day you want to run:");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let day = match input.trim().parse::<u8>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        if day == 0 || day > 25{
            println!("Please enter a valid day (1-25).");
            continue;
        }

        println!("Generating solution for Day {}...", day);
        adventofcode2024::run(day);
        break;
    }
}
