// sample A = 5934
// sample B = 26984457539
// real A = 365131
// real B = 1650309278600
use crate::common;

const FILENAME: &str = "inputs/06.txt";

pub fn solve_a() -> u64 {
    solve(80)
}

pub fn solve_b() -> u64 {
    solve(256)
}

fn solve(days: i32) -> u64 {
    let mut fishes = read_data();
    for _ in 0..days + 1 {
        let news = fishes[0];
        for gen in 0..8 {
            fishes[gen] = fishes[gen + 1];
        }
        fishes[6] += news;
        fishes[8] = news;
    }
    fishes[0..8].iter().sum()
}

fn read_data() -> Vec<u64> {
    let mut lines = common::read_lines2(FILENAME);
    let mut ret: Vec<u64> = vec![0; 9];
    let steps: Vec<usize> = lines
        .nth(0)
        .unwrap()
        .unwrap()
        .split(",")
        .map(|l| l.parse().unwrap())
        .collect();
    for step in steps {
        ret[step] += 1;
    }

    ret
}
