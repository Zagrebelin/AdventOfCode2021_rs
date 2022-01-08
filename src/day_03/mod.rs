use crate::common;
use std::borrow::Borrow;
use std::cmp::Ordering;

const LENGTH: usize = 12;
const FILENAME: &str = "inputs/03a.txt";
const ZERO: char = '0';
const ONE: char = '1';

#[allow(dead_code)]
pub fn solve_a() -> i32 {
    let numbers = read_file();
    let mut gamma = 0;
    let mut epsilon = 0;
    for pos in 0..LENGTH {
        let (zeroes, ones) = count_ones_zeroes(&numbers, pos);
        if ones > zeroes {
            gamma += 1 << (LENGTH - pos - 1);
        } else {
            epsilon += 1 << (LENGTH - pos - 1);
        }
    }
    gamma * epsilon
}

#[allow(dead_code)]
pub fn solve_b() -> i32 {
    let numbers = read_file();

    // oxygen generator - most common
    let mut slice = numbers.clone();
    for position in 0..LENGTH {
        let (zeroes, ones) = count_ones_zeroes(&slice, position);
        let x = match zeroes.cmp(&ones) {
            Ordering::Less | Ordering::Equal => ONE,
            Ordering::Greater => ZERO,
        };
        slice = filter_by_position(slice, position, x);
        if slice.len() == 1 {
            break;
        }
    }
    let oxy = binary_to_dec(&slice[0]);

    // co2 generator - less common
    slice = numbers.clone();
    for position in 0..LENGTH {
        let (zeroes, ones) = count_ones_zeroes(&slice, position);
        let x = match zeroes.cmp(&ones) {
            Ordering::Less | Ordering::Equal => ZERO,
            Ordering::Greater => ONE,
        };
        slice = filter_by_position(slice, position, x);
        if slice.len() == 1 {
            break;
        }
    }
    let co2 = binary_to_dec(&slice[0]);

    oxy * co2
}

fn filter_by_position(slice: Vec<String>, position: usize, x: char) -> Vec<String> {
    let mut sl2: Vec<String> = Vec::new();
    for i in slice {
        if get_digit(&i, position) == x {
            sl2.push(i);
        }
    }
    sl2
}

fn binary_to_dec(s: &String) -> i32 {
    i32::from_str_radix(&s, 2).expect(":(")
}

fn read_file() -> Vec<String> {
    let mut numbers: Vec<String> = Vec::new();
    let lines = common::read_lines(FILENAME).expect("Error reading input file");
    for s in lines {
        if let Ok(line) = s {
            numbers.push(line);
        }
    }
    numbers
}

fn count_ones_zeroes(slice: &Vec<String>, pos: usize) -> (usize, usize) {
    let mut zeroes = 0;
    let mut ones = 0;
    for s in slice.iter() {
        let i = get_digit(s, pos);
        if i == ZERO {
            zeroes += 1;
        } else if i == ONE {
            ones += 1;
        }
    }

    (zeroes, ones)
}

fn get_digit(i: &String, pos: usize) -> char {
    i.chars().nth(pos).expect(":(").borrow().clone()
}

// sample A = 198
// sample B = 230
// real A = 845186
// real B = 4636702
