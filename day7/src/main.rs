use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).expect("Should have been able to read the file");
    let lines: Vec<&str> = input.split_terminator("\n").collect();

    // Key highest match count, second highest match count
    // Ex: 5 of a kind is (5, 0)
    // ex: 4 of a kind is (4, 1)
    // ex: Full house is (3, 2)
    // ex: Three of a kind is (3, 1)
    // Value is rank: 5 of a kind = 7
    let types = HashMap::from([
        ((5, 0), 7),
        ((4, 1), 6),
        ((3, 2), 5),
        ((3, 1), 4),
        ((3, 0), 3),
        ((2, 2), 2),
        ((2, 1), 1),
        ((1, 1), 0)
    ]);

    let part1_result = part1(&lines, &types, false);
    print!("Part 1: {part1_result}\n");
    let part2_result = part1(&lines, &types, true);
    print!("Part 2: {part2_result}\n");
}

fn part1(lines: &Vec<&str>, types: &HashMap<(i64, i64), i64>, is_j_joker: bool) -> i64 {
    let mut hands: Vec<((i64, i64), i64)> = lines.iter().map(|l| {
        let mut tokens = l.split(" ");
        let hand = tokens.next().unwrap();
        let bid = tokens.next().map(|t| t.parse::<i64>().unwrap()).unwrap();
        (hand, bid)
    }).map(|(hand, bid)| {
        let hand_type = to_type(hand, is_j_joker);
        let hand_card_strength = to_strength(hand, is_j_joker);
        ((types[&hand_type], hand_card_strength), bid)
    }).collect();
    hands.sort();
    hands.iter().enumerate().map(|(i, ((_, _), bid))| {
       (i+1) as i64 * bid
    }).sum()
}

fn to_type(hand: &str, is_j_joker: bool) -> (i64, i64) {
    let mut char_counts: HashMap<char, i64> = HashMap::new();
    let mut jokers = 0;
    for c in hand.chars() {
        if is_j_joker && c == 'J' {
            jokers += 1;
        } else {
            *char_counts.entry(c).or_insert(0) += 1
        }
    }
    let mut counts: Vec<&i64> = char_counts.values().collect::<Vec<&i64>>();
    counts.sort();
    let highest = if counts.is_empty() {
        hand.len() as i64
    } else { 
        *counts[counts.len() - 1] + jokers
    };
    let second_highest = if counts.len() < 2 {
        0
    } else {
        *counts[counts.len() - 2]
    };
    (highest,  second_highest)
}

// Converts hand to hex and then parses to i64 to allow sorting
fn to_strength(hand: &str, is_j_joker: bool) -> i64 {
    let hex = hand.chars().map(|c| {
        match c {
            'T' => {'A'}
            'J' if is_j_joker => {'1'}
            'J' if !is_j_joker => {'B'}
            'Q' => {'C'}
            'K' => {'D'}
            'A' => {'E'}
            c => {c}
        }
    }).collect::<String>();
    i64::from_str_radix(&hex, 16).unwrap()
}
