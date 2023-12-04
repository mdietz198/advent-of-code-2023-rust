use std::env;
use std::fs;
use std::cmp;
use std::collections::HashSet;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Should have been able to read the file");
    let lines: Vec<&str> = input.split_terminator("\n").collect();

    let part1_result = part1(&lines);
    print!("Part 1: {part1_result}\n");
    let part2_result = part2(&lines);
    print!("Part 2: {part2_result}\n");
}

fn part1(lines: &Vec<&str>) -> i32 {
    let cards = lines.into_iter().map(|l| {
        let mut number_strings = l.split(":").nth(1).unwrap().split("|");
        Card::from_number_strings(number_strings.next().unwrap(), number_strings.next().unwrap())
    });
    cards.map(|c| c.score()).sum()
}

fn part2(lines: &Vec<&str>) -> i32 {
    let cards: Vec<Card> = lines.into_iter().map(|l| {
        let mut number_strings = l.split(":").nth(1).unwrap().split("|");
        Card::from_number_strings(number_strings.next().unwrap(), number_strings.next().unwrap())
    }).collect();
    let mut score_by_card_reverse: Vec<i32> = Vec::new();
    cards.iter().rev().map(|c| c.count_matches()).enumerate().for_each(|(index, score)| {
        let debug: Vec<i32> = (cmp::max(0, index as i32 - score) as usize .. index).map(|i| score_by_card_reverse[i]).collect();
        let copied_card_score: i32 = debug.into_iter().sum();
        let combined_score = 1 + copied_card_score as i32;
        score_by_card_reverse.push(combined_score);
    });
    score_by_card_reverse.into_iter().sum()
}

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<i32>,
    my_numbers: Vec<i32>
}

impl Card {
    fn from_number_strings(winning_string: &str, my_string: &str) -> Card {
      Card {
            winning_numbers: Card::string_to_int_vec(winning_string),
            my_numbers: Card::string_to_int_vec(my_string)
      }
    }

    fn string_to_int_vec(input: &str) -> Vec<i32> {
        let v: Vec<Result<i32, _>> = input.trim().split_terminator(" ").map(|s| s.parse::<i32>()).filter(|i| i.is_ok()).collect();
        v.into_iter().map(|e| e.unwrap()).collect()
    }

    fn count_matches(&self) -> i32 {
        let winning: HashSet<i32> = HashSet::from_iter(self.winning_numbers.iter().cloned());
        let my: HashSet<i32> = HashSet::from_iter(self.my_numbers.iter().cloned());
        let intersection = winning.intersection(&my);
        return intersection.count() as i32

    }

    fn score(&self) -> i32 {
        let winning_count:i32 = self.count_matches();
        if winning_count == 0 {
            0
        } else {
            (2 as i32).pow((winning_count - 1) as u32)
        }
    }
}
