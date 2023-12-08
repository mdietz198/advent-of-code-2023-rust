use std::env;
use std::fs;
use std::collections::HashMap;
use regex::Regex;
use std::time::SystemTime;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Should have been able to read the file");
    let lines: Vec<&str> = input.split_terminator("\n").collect();

    let right_left = lines[0];
    
    let re = Regex::new(r"([0-9A-Z]+)").unwrap();
    let node_map:HashMap<_, _> = lines[2..].iter().map(|l| {
        let mut iter = re.find_iter(l).map(|m| m.as_str());
        (iter.next().unwrap(), (iter.next().unwrap(), iter.next().unwrap()))
    }).collect();

    let part1_result = solve(vec!["AAA"], |node| node == "ZZZ", right_left, &node_map);
    print!("Part1: {part1_result:?}\n");
    //let part2_start = node_map.keys().filter(|k| k.chars().last().unwrap() == 'A').map(|k| *k).collect::<Vec<&str>>();
    let part2_start = vec!("JGA", "MQA", "DLA", "GSA", "MLA","AAA");
    // JGA => 20803
    // MQA => 23147
    // DLA => 17287
    // GSA => 19631
    // MLA => 12599
    // AAA => 13771
    print!("part2_start: {part2_start:?}\n");
    let part2_result = solve(part2_start, |node| node.chars().last().unwrap() == 'Z',  right_left, &node_map);
    print!("Part2: {part2_result:?}\n");
}

fn solve(start: Vec<&str>, is_terminal_node: fn(&str) -> bool, right_left: &str, node_map: &HashMap<&str, (&str, &str)>) -> i64 {
    let mut rl_cycle = right_left.chars().enumerate().cycle();
    let mut paths: Vec<Path> = start.iter().map(|s| Path {
        start_step: 0,
        start_node: String::from(*s),
        current_node: String::from(*s),
        current_step: 0
    }).collect();
    paths.iter().for_each(|p| print!("Path: {p:?}\n"));
    let mut steps = 0;
    // memoize the next end node and steps until it gets there
    let mut memoize_next_end: HashMap<(usize, String), (String, i64)> = HashMap::new();
    let (mut r_l_index, mut r_l) = rl_cycle.next().unwrap();
    let mut tick = 0;
    let now = SystemTime::now();
    while !all_paths_terminal_on_same_step(&paths, is_terminal_node) {
        if steps / 100000000 > tick {
            let time_past = now.elapsed().unwrap().as_secs();
            let max = 13129439557681_i64;
            let percent = steps as f64 / max as f64;
            print!("{steps}. {percent}% complete in {time_past} seconds\n");
            print!("{max}\n");
            tick = steps / 100000000;
        }

        // Operate on the path with the minimum next step: This could be optimized by storing paths
        // in a priority queue based on the next step but I know paths only has 6 elements so just
        // searching over the vector
        let mut smallest_path = paths.iter_mut().min().unwrap();
        //print!("Smallest path: {smallest_path:?}\n");

        if smallest_path.current_step != steps {
            (r_l_index, r_l) = rl_cycle.nth((smallest_path.current_step - steps - 1) as usize).unwrap();
        }

        steps = smallest_path.current_step;
        let current_node = &smallest_path.current_node;

        let start_node = &smallest_path.start_node;
        let step_diff = smallest_path.current_step - smallest_path.start_step;

        let (start_step, start_node, next_step, next_node) =  if memoize_next_end.contains_key(&(r_l_index, smallest_path.current_node.clone())) {
            /*
            print!("Cache hit\n");
            print!("key: ({r_l_index:?}, {current_node:?}\n");
            print!("{memoize_next_end:?}\n");
            */
            // If already cached this step, jump to the next step
            let next = &memoize_next_end[&(r_l_index, smallest_path.current_node.clone())];
            let next_step = smallest_path.current_step + next.1;
            let current_step = smallest_path.current_step;
            // print!("current_step: {current_step}, next_step: {next_step:?}\n");
            (current_step, current_node, next_step, next.0.as_str())
        } else {
            let (start_step, start_node) = if is_terminal_node(&smallest_path.current_node) && !memoize_next_end.contains_key(&(r_l_index, start_node.clone())) {
                // If at a terminal node, store in cache
                // print!("caching: ({r_l_index:?}, {start_node:?}) -> ({current_node:?}, {step_diff:?}) \n");
                memoize_next_end.insert((r_l_index, start_node.clone()), (current_node.clone(), step_diff));
                // print!("{memoize_next_end:?}\n");
                (smallest_path.current_step, current_node)
            } else {
                (smallest_path.start_step, start_node)
            };
            // Since not memoized, move on to next node
            let current_node: &str = current_node;
            let next_node = match r_l {
                'L' => {
                    node_map[current_node].0
                }
                'R' => {
                    node_map[current_node].1
                }
                x => panic!("Unexpected value {x:?}")
            };
            (start_step, start_node, smallest_path.current_step + 1, next_node)
        } ;
        //print!("Updated path: ({start_step:?}, {start_node:?}, {next_step:?}, {next_node:?})\n");
        smallest_path.update(start_step, start_node.clone(), next_step, String::from(next_node));
    }
    paths[0].current_step
}

fn all_paths_terminal_on_same_step(paths: &Vec<Path>, is_terminal_node: fn(&str) -> bool) -> bool {
    let mut paths_iter = paths.iter();
    let first_path_step = paths_iter.next().unwrap().current_step;
    let all_paths_same_step = paths_iter.all(|p| p.current_step == first_path_step);

    all_paths_same_step && paths.iter().all(|p| is_terminal_node(&p.current_node))
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Path {
    current_step: i64,
    start_node: String,
    current_node: String,
    start_step: i64,
}

impl Path {
    fn update(&mut self, start_step: i64, start_node: String, current_step: i64, current_node: String) {
        self.start_step = start_step;
        self.start_node = start_node;
        self.current_step = current_step;
        self.current_node = current_node;
    }
}
