use std::env;
use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Should have been able to read the file");
    let lines: Vec<&str> = input.split_terminator("\n").collect();

    //let part1_result = part1(lines);
    //print!("Part1:: {part1_result:?}\n");
    let part2_result = part2(lines);
    print!("Part2: {part2_result:?}\n");
}

fn part1(lines: Vec<&str>) -> i32 {
    let (numbers, symbol_indices) = prepare_data(lines);
    let filtered_numbers: Vec<PartNumber> = numbers.into_iter().filter(|p| p.has_adjacent_symbol(&symbol_indices)).collect();
    filtered_numbers.iter().map(|p| p.number).sum()
}

fn part2(lines: Vec<&str>) -> i32 {
    let (numbers, symbol_indices) = prepare_data(lines);
    let gear_indices: Vec<&(i32, i32)> = symbol_indices.iter().filter(|(_, symbol)| **symbol == '*').map(|(index, _)| index).collect();
    let mut sum = 0;
    for gear_index in gear_indices {
        let mut adjacent_numbers: Vec<PartNumber> = Vec::new();
        for n in &numbers {
            if n.get_adjacent_indices_with_symbol('*', &symbol_indices).contains(&gear_index) {
                adjacent_numbers.push(*n)
            }
        }
        if adjacent_numbers.len() == 2 {
            sum += adjacent_numbers[0].number * adjacent_numbers[1].number;
        }
    }
    sum
}

fn prepare_data(lines: Vec<&str>) -> (Vec<PartNumber>, HashMap<(i32, i32), char>) {
    let re = Regex::new(r"([0-9]+)|([^.\n0-9])").unwrap();
    let mut numbers: Vec<PartNumber> = Vec::new();
    // Map contains:
    // key: (row, column) starting in top left.
    // value: the symbol at that position
    let mut symbol_indices:HashMap<(i32, i32), char> = HashMap::new();
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
                Some(symbol) => {
                    // Must be a symbol
                    symbol_indices.insert((row as i32, start_column as i32), symbol);
                }
                other => {panic!( "Unexpected Other {other:?}")}
            }
        }
    }
    (numbers, symbol_indices)
}

#[derive(Debug, Copy, Clone)]
struct PartNumber {
    number: i32,
    row: i32,
    start_column: i32,
    end_column: i32
}

impl PartNumber {
    fn has_adjacent_symbol(&self, symbol_indices: &HashMap<(i32, i32), char>) -> bool {
        for i in self.row as i32 - 1 .. self.row as i32 + 2 {
            for j in self.start_column as i32 - 1 .. self.end_column as i32 + 2 {
               if symbol_indices.contains_key(&(i, j)) {
                   return true
               }
            }
        }
        false
    }

    fn get_adjacent_indices_with_symbol(&self, symbol: char, symbol_indices: &HashMap<(i32, i32), char>) -> Vec<(i32, i32)> {
        let mut result: Vec<(i32, i32)> = Vec::new();
        for i in self.row as i32 - 1 .. self.row as i32 + 2 {
            for j in self.start_column as i32 - 1 .. self.end_column as i32 + 2 {
               if symbol_indices.get(&(i, j)) == Some(&symbol) {
                   result.push((i, j));
               }
            }
        }
        result
    }
}
