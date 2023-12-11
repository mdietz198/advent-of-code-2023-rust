use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Should have been able to read the file");
    let lines: Vec<&str> = input.split_terminator("\n").collect();

    let part1_result = solve(&lines).unwrap();
    print!("Part1: {part1_result:?}\n");
    //let part2_result = solve(&lines, false);
    //print!("Part2: {part2_result:?}\n");
}

fn solve(lines: &Vec<&str>) -> Option<i64> {
    let board: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    print!("{board:?}\n");

    let start = board.iter().map(|v| {
        let column = v.iter().enumerate().find(|(_, &c)| c == 'S').map(|(column, _)| column);
        column
    }).enumerate().find(|(_, column_option)| *column_option != None).map(|(i, column_option)| (i as i64, column_option.unwrap() as i64))?;

    let mut path_length = 1;
    let mut current_step = start;
    let mut next_step = valid_steps(&current_step, &board)[0];
    while next_step != start {
        print!("{next_step:?}\n");
        let last_step = current_step;
        current_step = next_step;
        next_step = valid_steps(&current_step, &board).iter().find(|p| **p != last_step).unwrap().clone();
        //print!("current: {current_step:?} and next_step: {next_step:?}\n");
        path_length += 1;
    }
    return Some(path_length / 2);
}

fn valid_steps(current: &(i64, i64), board: &Vec<Vec<char>>) -> Vec<(i64, i64)> {
    match board[current.0 as usize][current.1 as usize] {
        'F' => {vec![(current.0 + 1, current.1), (current.0, current.1 + 1)]}
        '7' => {vec![(current.0, current.1 - 1), (current.0 + 1, current.1)]}
        'J' => {vec![(current.0, current.1 - 1), (current.0 - 1, current.1)]}
        'L' => {vec![(current.0 - 1, current.1), (current.0, current.1 + 1)]}
        '-' => {vec![(current.0, current.1 - 1), (current.0, current.1 + 1)]}
        '|' => {vec![(current.0 - 1, current.1), (current.0 + 1, current.1)]}
        '.' => {vec![]}
        'S' => {
            let result = vec![
                (current.0 - 1, current.1),
                (current.0 + 1, current.1),
                (current.0, current.1 - 1),
                (current.0, current.1 + 1)
            ].iter().filter(|p| p.0 >= 0 && p.0 < board.len() as i64 && p.1 >= 0 && p.1 < board[0].len() as i64)
            .filter(|&p| valid_steps(p, board).contains(current)).map(|p| p.clone()).collect();
            result
        }
        c => panic!("unexpected: {c}")
    }
}
