use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Should have been able to read the file");
    let lines: Vec<&str> = input.split_terminator("\n").collect();

    let part1_result = solve(&lines, true);
    print!("Part1: {part1_result:?}\n");
    let part2_result = solve(&lines, false);
    print!("Part2: {part2_result:?}\n");
}

fn solve(lines: &Vec<&str>, from_end: bool) -> i64 {
    lines.iter().map(|l| {
        extrapolate(l.split(" ").map(|i| i.parse::<i64>().unwrap()).collect(), from_end)
    }).sum()
}

fn extrapolate(seq: Vec<i64>, from_end: bool) -> i64 {
    if seq.iter().skip(1).all(|i| i == &seq[0]) {
        seq[0]
    } else {
        let one = seq.iter();
        let two = seq.iter().skip(1);
        let diff = one.zip(two).map(|(a, b)| b - a).collect();
        if from_end {
            seq.last().unwrap_or(&0) + extrapolate(diff, from_end)
        } else {
            seq.first().unwrap_or(&0) - extrapolate(diff, from_end)
        }
    }
}
