use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Should have been able to read the file");
    let lines: Vec<&str> = input.split_terminator("\n").collect();

    let part1_result = part1(&lines);
    print!("Part 1: {part1_result}\n");
    let part2_result = part2(&lines);
    print!("Part 2: {part2_result}\n");
}

struct Mapping {
    sub_map: Vec<(i64, i64, i64)>
}

impl Mapping {
    fn map(&self, n: i64)  -> i64 {
        for &(destination, source, length) in &self.sub_map {
            if source <= n && n < source + length {
                return n - source + destination
            }
        }
        n
    }
}

fn part1(lines: &Vec<&str>) -> i64 {
    let mut mappings: Vec<Mapping> = Vec::new();
    let mut seeds: Vec<i64> = Vec::new();
    let mut tuples: Vec<(i64, i64, i64)> = Vec::new();
    for l in lines {
        if l.starts_with("seeds") {
            l.split(":").nth(1).unwrap().split(" ").filter(|n| !n.is_empty()).map(|n| n.parse::<i64>().unwrap()).for_each(|n| seeds.push(n));
        } else if l.contains(":") && !tuples.is_empty() {
            let m = Mapping { sub_map: tuples.clone() };
            mappings.push(m);
            tuples.clear();
        } else if l.chars().next().map(|c| c.is_digit(10)).unwrap_or(false) {
            let mut i = l.split(" ").filter(|n| !n.is_empty()).map(|n| n.parse::<i64>().unwrap());
            let tuple = (i.next().unwrap(), i.next().unwrap(), i.next().unwrap());
            tuples.push(tuple);
        } else {
            // Do nothing but skip
        }
    }
    let m = Mapping { sub_map: tuples.clone() };
    mappings.push(m);
    tuples.clear();

    let location = seeds.iter().map(|&s| {
        // put seed through each map
        let mut index = s;
        for mapping in &mappings {
            index = mapping.map(index);
        }
        index
    });

    location.min().unwrap()
}

fn part2(lines: &Vec<&str>) -> i64 {
    let mut mappings: Vec<Mapping> = Vec::new();
    let mut seeds: Vec<(i64, i64)> = Vec::new();
    let mut tuples: Vec<(i64, i64, i64)> = Vec::new();
    for l in lines {
        if l.starts_with("seeds") {
            let v: Vec<i64> = l.split(":").nth(1).unwrap().split(" ").filter(|n| !n.is_empty()).map(|n| n.parse::<i64>().unwrap()).collect();
            v.chunks(2).for_each(|chunk| seeds.push((chunk[0], chunk[1])));
            print!("seeds: {seeds:?}\n");
        } else if l.contains(":") && !tuples.is_empty() {
            let m = Mapping { sub_map: tuples.clone() };
            mappings.push(m);
            tuples.clear();
        } else if l.chars().next().map(|c| c.is_digit(10)).unwrap_or(false) {
            let mut i = l.split(" ").filter(|n| !n.is_empty()).map(|n| n.parse::<i64>().unwrap());
            let tuple = (i.next().unwrap(), i.next().unwrap(), i.next().unwrap());
            tuples.push(tuple);
        } else {
            // Do nothing but skip
        }
    }
    let m = Mapping { sub_map: tuples.clone() };
    mappings.push(m);
    tuples.clear();

    let location = seeds.iter().inspect(|s| {
        let start = s.0;
        let end = s.0 + s.1;
        print!("start: {start}, end: {end}\n")
    }).flat_map(|p| p.0 .. p.0+p.1).map(|s| {
        // put seed through each map
        let mut index = s;
        if index % 10000000 == 0 { print!("{index}\n"); }
        for mapping in &mappings {
            index = mapping.map(index);
        }
        index
    });

    location.min().unwrap()
}
