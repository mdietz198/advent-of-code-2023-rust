use std::env;
use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Should have been able to read the file");
    let lines: Vec<&str> = input.split_terminator("\n").collect();

    let right_left = lines[0];
    
    let re = Regex::new(r"([A-Z]+)").unwrap();
    let node_map:HashMap<_, _> = lines[2..].iter().map(|l| {
        let mut iter = re.find_iter(l).map(|m| m.as_str());
        (iter.next().unwrap(), (iter.next().unwrap(), iter.next().unwrap()))
    }).collect();

    let part1_result = part1("AAA", right_left, node_map);
    print!("Part1: {part1_result:?}\n");
    let part2_result = part2(&lines);
    print!("Part2: {part2_result:?}\n");
}

fn part1(start: &str, right_left: &str, node_map: HashMap<&str, (&str, &str)>) -> i32 {
    let mut rl_cycle = right_left.chars().cycle();
    let mut node: &str = start;
    let mut i = 0;
    while node != "ZZZ"  {
        node = match rl_cycle.next() {
            Some('L') => {
                node_map[node].0
            }
            Some('R') => {
                node_map[node].1
            }
            x => panic!("Unexpected value {x:?}")
        };
        i += 1;
    }
    i
}

fn part2(lines: &Vec<&str>) -> i32 {
    0
}
