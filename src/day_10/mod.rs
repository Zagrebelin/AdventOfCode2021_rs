// sample A = 26397
// sample B = 288957
// real A = 316851
// real B = 2182912364
use crate::common;

use maplit::hashmap;
use std::collections::{HashMap, VecDeque};
use std::str::Chars;

const FILENAME: &str = "inputs/10.txt";

pub fn solve_a() -> u32 {
    let lines = read_data();
    let mut ret = 0;
    for line in lines {
        ret += found_broken_line(&line.chars().collect());
    }

    ret
}

pub fn solve_b() -> u64 {
    let lines = read_data();
    let mut scores: Vec<u64> = Vec::new();

    let mut ret = 0;
    for line in lines {
        if let Some(score) = fix_incomplete_line(&line.chars().collect()) {
            scores.push(score);
        }
    }

    scores.sort();
    scores[scores.len() / 2]
}

fn found_broken_line(cs: &Vec<char>) -> u32 {
    let mut st: VecDeque<char> = VecDeque::new();
    let pairs: HashMap<char, char> = hashmap![
        '[' => ']',
        '(' => ')',
        '<' => '>',
        '{' => '}'
    ];

    for c in cs.iter() {
        match *c {
            '[' | '(' | '{' | '<' => {
                st.push_back(*c);
            }
            ']' | ')' | '}' | '>' => {
                let from_st = st.pop_back().unwrap();
                if pairs.get(&from_st).unwrap() != c {
                    return match *c {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => 0,
                    };
                }
            }
            _ => {}
        }
    }
    0
}

fn fix_incomplete_line(cs: &Vec<char>) -> Option<u64> {
    let mut score = 0;
    let mut st: VecDeque<char> = VecDeque::new();
    let pairs: HashMap<char, char> = hashmap![
        '[' => ']',
        '(' => ')',
        '<' => '>',
        '{' => '}'
    ];

    for c in cs.iter() {
        match *c {
            '[' | '(' | '{' | '<' => {
                st.push_back(*c);
            }
            ']' | ')' | '}' | '>' => {
                let from_st = st.pop_back().unwrap();
                if pairs.get(&from_st).unwrap() != c {
                    return None;
                }
            }
            _ => {}
        }
    }

    while !st.is_empty() {
        let c = st.pop_back().unwrap();
        score = score * 5
            + match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0,
            };
    }

    return Some(score);
}

fn read_data() -> Vec<String> {
    let mut lines = common::read_lines2(FILENAME);

    let steps: Vec<String> = lines
        .map(|l| {
            l.unwrap()
            // .as_str()
            // .chars()
            // .collect()
        })
        .collect();

    steps
}
