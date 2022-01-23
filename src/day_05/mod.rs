// sample A = 5
// sample B = 12
// real A = 6841
// real B = 19258
use std::cmp::{max, Ordering};
use std::collections::HashMap;
use std::str::FromStr;

use crate::common;

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
        let start: &Point;
        let step_x: i32;
        let step_y: i32;

        if self.a.x == self.b.x {
            start = self.top_point();
            step_x = 0;
            step_y = 1;
        } else {
            start = self.left_point();
            step_x = 1;
            step_y = match start.y.cmp(&self.right_point().y) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            };
        }

        let is_straight = (step_x == 0) ^ (step_y == 0);

        let points = match with_diag || is_straight {
            true => (0..self.length() + 1)
                .map(|i| [start.x + step_x * i, start.y + step_y * i])
                .collect(),
            false => vec![],
        };

        points
    }

    fn left_point(&self) -> &Point {
        match self.a.left_than(&self.b) {
            true => &self.a,
            false => &self.b,
        }
    }
    fn right_point(&self) -> &Point {
        match self.a.left_than(&self.b) {
            true => &self.b,
            false => &self.a,
        }
    }
    fn top_point(&self) -> &Point {
        match self.a.above_than(&self.b) {
            true => &self.a,
            false => &self.b,
        }
    }

    fn length(&self) -> i32 {
        return max((self.a.x - self.b.x).abs(), (self.a.y - self.b.y).abs());
    }
}

pub fn solve_a() -> i32 {
    solve(false, "outputs/05_a.png")
}

pub fn solve_b() -> i32 {
    solve(true, "outputs/05_b.png")
}

fn solve(with_diag: bool, fname: &str) -> i32 {
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
    // let counter = field.values().into_iter().filter(|v| **v >= 2).count();
    let counter = field.values().into_iter().max().unwrap();
    save(&field, fname);

    *counter as i32
}

fn save(field: &HashMap<[i32; 2], i32>, fname: &str) {
    //! An example of generating julia fractals.
    let imgx = field.iter().map(|key| key.0[0]).max().unwrap() as u32;
    let imgy = field.iter().map(|key| key.0[1]).max().unwrap() as u32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = match field.get(&[x as i32, y as i32]) {
            None => image::Rgb([0, 0, 0]),
            Some(_) => image::Rgb([150u8, 150u8, 150u8]),
        };
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save(fname).expect("Save inmage");
}

fn read_data() -> Vec<Line> {
    let lines = common::read_lines2(FILENAME);
    let steps: Vec<Line> = lines
        .map(|line| Line::from_str(line.unwrap().as_str()).expect("Line"))
        .into_iter()
        .collect();

    steps
}
