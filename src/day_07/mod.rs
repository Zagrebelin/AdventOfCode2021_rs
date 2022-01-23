// sample A = 37
// sample B = 168
// real A = 340052
// real B = 92948968

use crate::common;
use std::collections::HashMap;
use std::fs::read;

const FILENAME: &str = "inputs/07.txt";

pub fn solve_a() -> i32 {
    solve(distance_a, "outputs/07_a.png")
}

pub fn solve_b() -> i32 {
    solve(distance_b, "outputs/07_b.png")
}

pub fn solve(fit_function: fn(i32, i32) -> i32, fname: &str) -> i32 {
    let crabs = read_data();
    let begin = *crabs.iter().min().unwrap();
    let end = *crabs.iter().max().unwrap();
    let mut ans: HashMap<i32, i32> = HashMap::new();
    for pos in begin..end + 1 {
        ans.insert(pos, crabs.iter().map(|c| fit_function(pos, *c)).sum());
    }
    draw_chart(&crabs, &ans, fname);

    ans.iter().map(|(key, value)| *value).min().unwrap();
}

fn distance_a(a: i32, b: i32) -> i32 {
    (a - b).abs()
}
fn distance_b(a: i32, b: i32) -> i32 {
    let d = (a - b).abs();
    (1 + d) * d / 2
}

fn read_data() -> Vec<i32> {
    let mut lines = common::read_lines2(FILENAME);

    let steps: Vec<i32> = lines
        .nth(0)
        .unwrap()
        .unwrap()
        .split(",")
        .map(|l| l.parse().unwrap())
        .collect();
    steps
}
