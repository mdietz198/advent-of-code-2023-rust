use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Should have been able to read the file");
    let lines: Vec<String> = input.split("\n").map(String::from).collect();
    let mut digits: Vec<u32> = Vec::new();
    let mut sum:u32 = 0;
    let patterns = [
        (String::from("0"), 0),
        (String::from("1"), 1),
        (String::from("2"), 2),
        (String::from("3"), 3),
        (String::from("4"), 4),
        (String::from("5"), 5),
        (String::from("6"), 6),
        (String::from("7"), 7),
        (String::from("8"), 8),
        (String::from("9"), 9),
        (String::from("zero"), 0),
        (String::from("one"), 1),
        (String::from("two"), 2),
        (String::from("three"), 3),
        (String::from("four"), 4),
        (String::from("five"), 5),
        (String::from("six"), 6),
        (String::from("seven"), 7),
        (String::from("eight"), 8),
        (String::from("nine"), 9),
        ];
    let mut index;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        println!("{}", line);
        index = 0;
        digits.clear();
        while index < line.len() {
            for (pattern, value) in patterns.iter() {
                if line[index..].starts_with(pattern) {
                    print!("Pattern: {}, value: {:?}\n", pattern, value);
                    digits.push(value.clone());
                    continue;
                }
            }
            index += 1;
        }
 
        sum = sum + (10 * digits.first().unwrap()) + digits.last().unwrap();
        print!("Partial sum: {}\n", sum);
    }
    print!("Sum: {}\n", sum);
    print!("Done");
}
