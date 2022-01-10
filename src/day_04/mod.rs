// sample A = 4512
// sample B = 1924
// real A = 63424
// real B = 10992 .. ...

use crate::common;
use std::fs::File;
use std::io::{BufReader, Lines};

const FILENAME: &str = "inputs/04_test.txt";
const SIZE: usize = 5;

struct Board {
    cells: Vec<i8>,
    marked: Vec<bool>,
}

impl Board {
    pub fn do_step(&mut self, s: i8) {
        for row in 0..SIZE {
            for col in 0..SIZE {
                let cell_idx = row * SIZE + col;
                if self.cells[cell_idx] == s {
                    self.marked[cell_idx] = true;
                }
            }
        }
    }

    pub fn is_winner(&self) -> bool {
        for start in 0..5 {
            // 0 5 10 15 20, 1 6 11 16 21 ...
            if self.marked.iter().skip(start).step_by(5).all(|m| *m) {
                return true;
            }
            // 0 1 2 3 4, 5 6 7 8 9, ...
            if self.marked.iter().skip(start * 5).take(5).all(|m| *m) {
                return true;
            }
        }
        false
    }

    pub fn score(&self, last_step: i32) -> i32 {
        self.sum_unmarked() * last_step
    }

    fn sum_unmarked(&self) -> i32 {
        let mut ret: i32 = 0;
        for idx in 0..25 {
            if !self.marked[idx] {
                ret += self.cells[idx] as i32
            }
        }
        ret
    }
}

pub fn solve_a() -> i32 {
    let mut lines = common::read_lines2(FILENAME);
    // moves - first line splitted by comma
    let steps: Vec<i8> = lines
        .nth(0)
        .unwrap()
        .ok()
        .unwrap()
        .split(',')
        .map(|part| part.parse().ok().unwrap())
        .collect();

    let mut boards: Vec<Board> = Vec::new();
    loop {
        let may_be_board = read_board(&mut lines);
        match may_be_board {
            Some(board) => boards.push(board),
            None => break,
        }
    }

    for step in steps {
        for board in boards.iter_mut() {
            board.do_step(step);
            if board.is_winner() {
                return board.score(step as i32);
            }
        }
    }

    0
}
pub fn solve_b() -> i32 {
    let mut lines = common::read_lines2(FILENAME);
    let mut boards_count = 0;
    // moves - first line splitted by comma
    let steps: Vec<i8> = lines
        .nth(0)
        .unwrap()
        .ok()
        .unwrap()
        .split(',')
        .map(|part| part.parse().ok().unwrap())
        .collect();
    let mut boards: Vec<Board> = Vec::new();
    loop {
        let may_be_board = read_board(&mut lines);
        match may_be_board {
            Some(board) => {
                boards.push(board);
                boards_count += 1
            }
            None => break,
        }
    }

    let mut last_looser: Option<&Board>;
    for step in steps {
        boards.iter_mut().for_each(|board| board.do_step(step));
        let losers: Vec<&Board> = boards.iter().filter(|b| !b.is_winner()).collect();
        if losers.len() == 1 {
            last_looser = Some(losers[0]);
        }
        if last_looser.is_some() && last_looser.unwrap().is_winner() {
            return last_looser.unwrap().score(step as i32);
        }
        println!("{}", losers.len());
    }

    0
}

fn read_board(lines: &mut Lines<BufReader<File>>) -> Option<Board> {
    let mut ret: String = String::from("");

    for l in lines.skip(1).take(5) {
        if let Ok(line) = l {
            ret.push_str(&line);
            ret.push_str(" ");
        }
    }
    return if ret.len() == 0 {
        None
    } else {
        Option::from(Board {
            marked: vec![false; SIZE * SIZE],
            cells: ret
                .split_whitespace()
                .map(|p| p.parse().ok().unwrap())
                .collect(),
        })
    };
}
