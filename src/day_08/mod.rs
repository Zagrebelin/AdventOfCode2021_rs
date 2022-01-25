// sample A = 26
// sample B =
// real A = 416
// real B =

use crate::common;
use plotters::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Lines};

const FILENAME: &str = "inputs/08.txt";

pub fn solve_a() -> i32 {
    let lines = read_data();
    let mut counter = 0;
    for line in lines {
        let part2 = line.split("|").last().unwrap();
        counter += part2
            .split_whitespace()
            .filter(|word| vec![2, 4, 3, 7].contains(&word.len()))
            .count();
    }

    counter as i32
}

pub fn solve_b() -> i32 {
    -1
}

fn read_data() -> Vec<String> {
    let mut lines = common::read_lines2(FILENAME);

    let steps: Vec<String> = lines.map(|l| l.unwrap()).collect();

    steps
}
