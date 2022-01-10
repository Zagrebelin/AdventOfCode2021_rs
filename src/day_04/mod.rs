// sample A = 4512
// sample B = 1924
// real A = 63424
// real B = 10992 .. ...

/*
Игра "Бинго"

    Есть набор досок (Board)
    есть набор ходов (steps)
    ходы применяются к доскам (do_step)
    доска может быть выигравшей (is_winner)
 */

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

    pub fn score(&self, last_step: i8) -> i32 {
        self.sum_unmarked() * (last_step as i32)
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
    /* найти первого победителя */
    let (steps, mut boards) = read_data();

    for step in steps {
        boards.iter_mut().for_each(|board| board.do_step(step));
        let may_be_winner = boards.iter().filter(|board| board.is_winner()).nth(0);
        if let Some(winner) = may_be_winner {
            return winner.score(step);
        }
    }

    -1
}

// только одна мутабельная ссылка в каждый момент времени
// сколько угодно немутабельных ссылок
// нельзя миксовать мутабельные и немутабельный ссылки

pub fn solve_b() -> i32 {
    /* найти последнего победителя */
    let (steps, mut boards) = read_data();

    let mut may_by_last_winner: Option<&Board> = None;
    for step in steps {
        boards.iter_mut().for_each(|board| board.do_step(step));
        // вектор не победителей. Если он там только один, то запоминаем его как
        // последнего победителя
        let losers: Vec<&Board> = boards.iter().filter(|b| !b.is_winner()).collect();
        if losers.len() == 1 {
            may_by_last_winner = Some(losers[0]);
        }
        if let Some(last_winner) = may_by_last_winner {
            if last_winner.is_winner() {
                return last_winner.score(step);
            }
        }
    }

    -1
}
fn read_data() -> (Vec<i8>, Vec<Board>) {
    let mut lines = common::read_lines2(FILENAME);

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
    (steps, boards)
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
