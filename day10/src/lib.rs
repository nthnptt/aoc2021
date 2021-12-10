use std::{collections::VecDeque, ops::Add};

#[derive(Debug, PartialEq)]
pub enum Validation {
    Valid,
    Incomplete,
    Corrupted(char),
}

const OPENER: [char; 4] = ['(', '{', '[', '<'];
const CLOSER: [char; 4] = [')', '}', ']', '>'];

pub fn to_score(char: char) -> u32 {
    match char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

pub fn repair_to_score(line: String) -> u128 {
    const MULTIPLIER: u128 = 5;
    let mut result: u128 = 0;
    for char in line.chars() {
        result *= MULTIPLIER;
        result += match char {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        }
    }
    result
}

pub fn repair(input: String) -> String {
    let mut char_queue: VecDeque<usize> = VecDeque::new();
    for char in input.chars() {
        match OPENER.into_iter().position(|e| e == char) {
            Some(position) => char_queue.push_front(position),
            None => {
                if CLOSER[char_queue.pop_front().unwrap()] != char {
                    break;
                }
            }
        }
    }

    char_queue
        .into_iter()
        .fold(String::new(), |mut acc, index| {
            acc.push(CLOSER[index]);
            acc
        })
}

pub fn check(input: String) -> Validation {
    let mut char_queue: VecDeque<usize> = VecDeque::new();
    let mut result = Validation::Valid;
    for char in input.chars() {
        match OPENER.into_iter().position(|e| e == char) {
            Some(position) => char_queue.push_front(position),
            None => {
                if CLOSER[char_queue.pop_front().unwrap()] != char {
                    return Validation::Corrupted(char);
                }
            }
        }
    }
    if !char_queue.is_empty() {
        result = Validation::Incomplete
    }

    result
}
