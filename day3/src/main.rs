use std::env;
use std::fs;
use std::collections::HashSet;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Should have been able to read the file");
    let lines: Vec<&str> = input.split_terminator("\n").collect();
    let result = day1(lines);
    print!("Day1: {result:?}\n");
}

fn day1(lines: Vec<&str>) -> i32 {
    let (numbers, symbol_indices) = prepare_data(lines);
    let filtered_numbers: Vec<PartNumber> = numbers.into_iter().filter(|p| p.has_adjacent_symbol(&symbol_indices)).collect();
    filtered_numbers.iter().map(|p| p.number).sum()
}

fn prepare_data(lines: Vec<&str>) -> (Vec<PartNumber>, HashSet<(i32, i32)>) {
    let re = Regex::new(r"([0-9]+)|([^.\n0-9])").unwrap();
    let mut numbers: Vec<PartNumber> = Vec::new();
    // Set contains (row, column) starting in top left.
    let mut symbol_indices:HashSet<(i32, i32)> = HashSet::new();
    for (row, l) in lines.iter().enumerate() {
        for m in re.find_iter(l) {
            let start_column = m.start();
            let end_column = m.end() - 1; // m.end() gives the *exclusive* index so changing it to
                                          // inclusive
            let value = m.as_str();
            let mut chars = value.chars();
            let first = chars.next();
            match first {
                Some(first) if first.is_digit(10) => {
                    numbers.push(
                        PartNumber {
                            number: value.parse::<i32>().unwrap(),
                            row: row as i32,
                            start_column: start_column as i32,
                            end_column: end_column as i32
                        }
                    )
                }
                _ => {
                    // Must be a symbol
                    symbol_indices.insert((row as i32, start_column as i32));
                }
            }
        }
    }
    (numbers, symbol_indices)
}

#[derive(Debug)]
struct PartNumber {
    number: i32,
    row: i32,
    start_column: i32,
    end_column: i32
}

impl PartNumber {
    fn has_adjacent_symbol(&self, symbol_indices: &HashSet<(i32, i32)>) -> bool {
        for i in self.row as i32 - 1 .. self.row as i32 + 2 {
            for j in self.start_column as i32 - 1 .. self.end_column as i32 + 2 {
               if symbol_indices.contains(&(i, j)) {
                   return true
               }
            }
        }
        false
    }
}
