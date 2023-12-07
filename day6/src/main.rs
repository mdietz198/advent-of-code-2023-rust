use std::env;
use std::fs;
use std::cmp;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Should have been able to read the file");
    let lines: Vec<&str> = input.split_terminator("\n").collect();

    let part1_result = part1(&lines);
    print!("Part 1: {part1_result}\n");
    let part2_result = part2(&lines);
    print!("Part 2: {part2_result}\n");
}
/*
Races:
    (t, d)
    (7, 9)
    (15, 40)
    (30, 200)]
Math: 
distance = s * (t - s)
part of curve above record is:
over_record = s * (t - s) - d = t - s
*/

fn part1(lines: &Vec<&str>) -> i32 {
    let re = Regex::new(r"([0-9]+)").unwrap();
    let line0_iter = re.find_iter(lines[0]);
    let line1_iter = re.find_iter(lines[1]);
    let races: Vec<(i32, i32)> = line0_iter.zip(line1_iter).map(|(a, b)| (a.as_str().parse::<i32>().unwrap(), b.as_str().parse::<i32>().unwrap())).collect();
    races.iter().map(|&(t, d)| {
        (0..t).filter(|s| s * (t - s) > d).count()
    }).reduce(|a, b| a * b).unwrap() as i32
}

fn part2(lines: &Vec<&str>) -> i64 {
    let races = [(
        lines[0].split(":").nth(1).unwrap().replace(" ", "").parse::<i64>().unwrap(),
        lines[1].split(":").nth(1).unwrap().replace(" ", "").parse::<i64>().unwrap()
    )];
    races.iter().map(|&(t, d)| {
        (0..t).filter(|s| s * (t - s) > d).count()
    }).reduce(|a, b| a * b).unwrap() as i64
}

