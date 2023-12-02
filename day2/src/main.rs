use std::env;
use std::fs;
use std::collections::HashMap;
use std::cmp;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Should have been able to read the file");
    let lines: Vec<&str> = input.split_terminator("\n").collect();

    let max_map = HashMap::from([
      ("red", 12),
      ("green", 13),
      ("blue", 14)]);
    let part1_result:i32 = lines.iter().map(|l| is_possible(l, &max_map)).flatten().sum();
    
    print!("Part 1: {part1_result}\n");

    let part2_result:i32 = lines.iter().map(|l| power(l).unwrap()).sum();
    print!("Part 2: {part2_result}\n");
    print!("Done");
}

// Returns game ID if it is possible, else return None
fn is_possible(line: &str, max_map: &HashMap<&str, i32>) -> Option<i32> {
    let mut s = line.split(": ");
    let game_id = s.next()?.split(" ").last().map(|g| g.parse::<i32>().ok())?;
    let draws = s.next()?;
    for draw in draws.split("; ") {
        for cube in draw.split(", ") {
            let mut s = cube.split(" ");
            let (num, color) = (s.next()?, s.next()?);
            if max_map[color] < num.parse::<i32>().unwrap() {
                return None;
            }
        }
    }
    game_id
}

fn power(line: &str) -> Option<i32> {
    let s = line.split(": ");
    let draws = s.last()?;
    let mut max_color = HashMap::from([
        ("red", 0),
        ("green", 0),
        ("blue", 0)
    ]);
    for draw in draws.split("; ") {
        for cube in draw.split(", ") {
            let mut s = cube.split(" ");
            let (num, color) = (s.next()?.parse::<i32>().unwrap(), s.next()?);
            max_color.insert(color, cmp::max(max_color[color], num));
        }
    }
    Some(max_color["red"] * max_color["green"] * max_color["blue"])
}
