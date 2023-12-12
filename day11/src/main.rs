use std::env;
use std::fs;
use std::cmp;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Should have been able to read the file");
    let lines: Vec<&str> = input.split_terminator("\n").collect();

    let part1_result = solve(&lines, 2);
    print!("Part1: {part1_result:?}\n");
    let part2_result = solve(&lines, 1000000);
    print!("Part2: {part2_result:?}\n");
}

fn solve(lines: &Vec<&str>, expansion: i64) -> i64 {
    let mut galaxies: Vec<(i64, i64)> = Vec::new();
    for i in 0..lines.len() {
        for (j, c) in lines[i].chars().enumerate() {
            if c == '#' {
                galaxies.push((i as i64, j as i64));
            }
        }
    }
    let rows_with_galaxies = galaxies.iter().map(|(row, _)| *row as usize).collect::<HashSet<usize>>();
    let columns_with_galaxies = galaxies.iter().map(|(_, column)| *column as usize).collect::<HashSet<usize>>();
    let empty_rows: Vec<usize> = (0..lines.len()).filter(|row| !rows_with_galaxies.contains(row)).collect();
    let empty_columns: Vec<usize> = (0..lines[0].len()).filter(|column| !columns_with_galaxies.contains(column)).collect();
    let mut sum = 0;
    for i in 0 .. galaxies.len() {
        for j in i + 1 .. galaxies.len() {
            sum += distance_between(galaxies[i], galaxies[j], &empty_rows, &empty_columns, expansion)
        }
    }
    sum
}

fn distance_between(g1: (i64, i64), g2: (i64, i64), empty_rows: &Vec<usize>, empty_columns: &Vec<usize>, expansion: i64) -> i64 {
    let max_row = cmp::max(g1.0, g2.0);
    let min_row = cmp::min(g1.0, g2.0);
    let max_column = cmp::max(g1.1, g2.1);
    let min_column = cmp::min(g1.1, g2.1);
    let distance = (max_row - min_row) 
        + (max_column - min_column) 
        + (expansion - 1) * empty_rows.iter().filter(|r| min_row < (**r as i64) && (**r as i64) < max_row).count() as i64
        + (expansion - 1) * empty_columns.iter().filter(|c| min_column < (**c as i64) && (**c as i64) < max_column).count() as i64;
    distance
}
