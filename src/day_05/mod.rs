// sample A = 5
// sample B = 12
// real A = 6841
// real B = 19258
extern crate regex;

use regex::{Captures, Regex};
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

struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl FromStr for Line {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p1: Vec<&str> = s.split(" -> ").flat_map(|p| p.split(",")).collect();

        Ok(Line {
            x1: p1[0].parse().unwrap(),
            y1: p1[1].parse().unwrap(),
            x2: p1[2].parse().unwrap(),
            y2: p1[3].parse().unwrap(),
        })
    }
}

impl Line {
    fn points(&self, with_diag: bool) -> Vec<[i32; 2]> {
        let mut points: Vec<[i32; 2]> = Vec::new();
        if self.x1 == self.x2 {
            let miny = min(self.y1, self.y2);
            let maxy = max(self.y1, self.y2);
            for y in miny..maxy + 1 {
                points.push([self.x1, y])
            }
        } else if self.y1 == self.y2 {
            let minx = min(self.x1, self.x2);
            let maxx = max(self.x1, self.x2);
            for x in minx..maxx + 1 {
                points.push([x, self.y1])
            }
        } else if with_diag {
            let mut left_x = 0;
            let mut left_y = 0;
            let mut slope = 0;
            if self.x1 < self.x2 {
                left_x = self.x1;
                left_y = self.y1;
                slope = match self.y1.cmp(&self.y2) {
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                    Ordering::Greater => -1,
                }
            } else {
                left_x = self.x2;
                left_y = self.y2;
                slope = match self.y2.cmp(&self.y1) {
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                    Ordering::Greater => -1,
                }
            }
            let length = (self.x1 - self.x2).abs();
            for i in 0..length + 1 {
                points.push([left_x + i, left_y + slope * i]);
            }
        }

        points
    }
}

pub fn solve_a() -> i32 {
    let lines = read_data();
    let mut field: HashMap<[i32; 2], i32> = HashMap::new();
    for line in lines.iter() {
        for point in line.points(false) {
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

pub fn solve_b() -> i32 {
    println!("Before read");
    let lines = read_data();
    println!("After read");
    let mut field: HashMap<[i32; 2], i32> = HashMap::new();
    for line in lines.iter() {
        for point in line.points(true) {
            if !field.contains_key(&point) {
                field.insert(point, 0);
            }
            let old = field.get(&point).unwrap();
            field.insert(point, old + 1);
        }
    }
    println!("Count");
    let counter = field.values().into_iter().filter(|v| **v >= 2).count();
    println!("Done");
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
