// sample A = 5
// sample B = 12
// real A = 6841
// real B = 19258
use std::cmp::{max, min, Ordering};
use std::collections::HashMap;

use crate::common;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufReader, Lines};
use std::iter::Map;
use std::str::FromStr;
use std::thread::yield_now;

const FILENAME: &str = "inputs/05.txt";

struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p: Vec<&str> = s.split(",").collect();
        Ok(Point {
            x: p[0].parse().unwrap(),
            y: p[1].parse().unwrap(),
        })
    }
}
impl Point {
    fn left_than(&self, other: &Point) -> bool {
        self.x < other.x
    }

    fn above_than(&self, other: &Point) -> bool {
        self.y < other.y
    }
}
struct Line {
    a: Point,
    b: Point,
}

impl FromStr for Line {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p1: Vec<&str> = s.split(" -> ").collect();

        Ok(Line {
            a: Point::from_str(p1[0]).unwrap(),
            b: Point::from_str(p1[1]).unwrap(),
        })
    }
}

impl Line {
    fn points(&self, with_diag: bool) -> Vec<[i32; 2]> {
        let mut points: Vec<[i32; 2]> = Vec::new();
        if self.a.x == self.b.x {
            // особый случай - вертикальная палка
            let top = match self.a.above_than(&self.b) {
                true => &self.a,
                false => &self.b,
            };

            for y in 0..self.length() + 1 {
                points.push([self.a.x, top.y + y])
            }
        } else {
            // горизонтальная или наклонная линия
            let mut left: &Point;
            let mut right: &Point;
            if self.a.left_than(&self.b) {
                left = &self.a;
                right = &self.b;
            } else {
                left = &self.b;
                right = &self.a;
            }
            let slope = match left.y.cmp(&right.y) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            };
            if with_diag || slope == 0 {
                for i in 0..self.length() + 1 {
                    points.push([left.x + i, left.y + slope * i]);
                }
            }
        }

        points
    }

    fn length(&self) -> i32 {
        return max((self.a.x - self.b.x).abs(), (self.a.y - self.b.y).abs());
    }
}

pub fn solve_a() -> i32 {
    solve(false)
}

pub fn solve_b() -> i32 {
    solve(true)
}

fn solve(with_diag: bool) -> i32 {
    let lines = read_data();
    let mut field: HashMap<[i32; 2], i32> = HashMap::new();
    for line in lines.iter() {
        for point in line.points(with_diag) {
            if !field.contains_key(&point) {
                field.insert(point, 0);
            }
            let old = field.get(&point).unwrap();
            field.insert(point, old + 1);
        }
    }
    let counter = field.values().into_iter().filter(|v| **v >= 2).count();
    counter as i32
}

fn read_data() -> Vec<Line> {
    let mut lines = common::read_lines2(FILENAME);
    let steps: Vec<Line> = lines
        .map(|line| Line::from_str(line.unwrap().as_str()).expect("Line"))
        .into_iter()
        .collect();

    steps
}
