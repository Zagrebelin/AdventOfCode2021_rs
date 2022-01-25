// sample A = 26397
// sample B = 288957
// real A = 316851
// real B = 2182912364
use crate::common;

use maplit::hashmap;
use std::collections::{HashMap, VecDeque};
use std::str::Chars;

const FILENAME: &str = "inputs/10.txt";
enum StringType {
    Incomplete(VecDeque<char>),
    Invalid(char),
    Ok,
}

pub fn solve_a() -> u32 {
    let lines = read_data();
    let mut ret = 0;
    for line in lines {
        if let StringType::Invalid(c) = parse_line(&line) {
            ret += match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            }
        }
    }

    ret
}

pub fn solve_b() -> u64 {
    let lines = read_data();
    let mut scores: Vec<u64> = Vec::new();

    for line in lines {
        if let StringType::Incomplete(mut rest) = parse_line(&line) {
            scores.push(fix_incomplete_line(&mut rest));
        }
    }

    scores.sort();
    scores[scores.len() / 2]
}

fn parse_line(line: &String) -> StringType {
    let chars: Vec<char> = line.chars().collect();
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
                    st.push_back(*c);
                    return StringType::Invalid(*c);
                }
            }
            _ => {}
        }
    }
    match st.is_empty() {
        true => StringType::Ok,
        false => StringType::Incomplete(st),
    }
}

fn fix_incomplete_line(mut st: &mut VecDeque<char>) -> u64 {
    let mut score = 0;

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

    score
}

fn read_data() -> Vec<String> {
    let mut lines = common::read_lines2(FILENAME);

    let steps: Vec<String> = lines.map(|l| l.unwrap()).collect();

    steps
}
