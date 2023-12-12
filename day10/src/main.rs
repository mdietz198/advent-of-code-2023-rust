use std::env;
use std::fs;
use std:collections:HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Should have been able to read the file");
    let lines: Vec<&str> = input.split_terminator("\n").collect();

    let path = solve(&lines);
    let part1_result = path.len() / 2;
    print!("Part1: {part1_result:?}\n");
    //let part2_result = solve(&lines, false);
    //print!("Part2: {part2_result:?}\n");
}

enum Direction {
    up,
    down,
    left,
    right
}

fn solve(lines: &Vec<&str>) -> Vec<(i64, i64)> {
    let board: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    let start = board.iter().map(|v| {
        let column = v.iter().enumerate().find(|(_, &c)| c == 'S').map(|(column, _)| column);
        column
    }).enumerate().find(|(_, column_option)| *column_option != None).map(|(i, column_option)| (i as i64, column_option.unwrap() as i64)).unwrap();

    let mut path: Vec<((i64, i64), Direction)> = Vec::new();
    let mut next_step = valid_steps(&start, &board)[0];
    path.push((start, direction(start, next_step)));
    while next_step != start {
        let last_step = path[path.len()-1].0;
        let current_step = next_step;
        next_step = valid_steps(&current_step, &board).iter().find(|p| **p != last_step).unwrap().clone();
        path.push((current_step, direction(current_step, next_step)));
    }
    print_path(&path, &board);

    return path.iter().map(|p| p.0).collect()
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
                (current.0, current.1 + 1),
                (current.0, current.1 - 1),
                (current.0 - 1, current.1),
                (current.0 + 1, current.1)
            ].iter().filter(|p| p.0 >= 0 && p.0 < board.len() as i64 && p.1 >= 0 && p.1 < board[0].len() as i64)
            .filter(|&p| valid_steps(p, board).contains(current)).map(|p| p.clone()).collect();
            result
        }
        c => panic!("unexpected: {c}")
    }
}

fn direction(start: (i64, i64), end: (i64, i64)) -> Direction {
    match (end.0 - start.0, end.1 - start.1) {
        (-1, 0) => {Direction::up}
        (1, 0) => {Direction::down}
        (0, -1) => {Direction::left}
        (0, 1) => {Direction::right}
        p => panic!("unexpected: {p:?}")
    }
}

fn print_path(path: &Vec<((i64, i64), Direction)>, board: &Vec<Vec<char>>) {
    for i in 0 .. board.len() {
        for j in 0 .. board[i].len() {
            let in_path = path.iter().find(|p| p.0 == (i as i64, j as i64)).map(|p| &p.1);
            let c = match in_path {
                _ if board[i][j] == 'S' => {'S'}
                Some(Direction::up) => {'^'}
                Some(Direction::down) => {'v'}
                Some(Direction::left) => {'<'}
                Some(Direction::right) => {'>'}
                None => {'.'}
            };
            print!("{c}");
        }
        print!("\n");
    }
}

fn find_inside_spaces(path: &Vec<(i64, i64)>, board: &Vec<Vec<char>>) -> HashSet<(i64, i64)> {
    let mut connected_to_outside: HashSet<EscapeStep> = HashSet::new();
    let mut candidates: HashSet<EscapeStep> = HashSet::new();


}

enum EscapeStep {
    Space(i64, i64),
    Squeeze((i64, i64), (i64, i64))
}
