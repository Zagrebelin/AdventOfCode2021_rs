// sample A = 37
// sample B = 168
// real A = 340052
// real B = 92948968

use crate::common;
use plotters::prelude::*;
use std::collections::HashMap;

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
    draw_chart(&crabs, &ans, begin, end, fname);

    ans.iter().map(|(_, value)| *value).min().unwrap()
}

fn distance_a(a: i32, b: i32) -> i32 {
    (a - b).abs()
}
fn distance_b(a: i32, b: i32) -> i32 {
    let d = (a - b).abs();
    (1 + d) * d / 2
}

fn draw_chart(crabs: &Vec<i32>, ans: &HashMap<i32, i32>, begin: i32, end: i32, fname: &str) {
    let mut cc: HashMap<i32, i32> = HashMap::new();
    for i in begin..=end {
        cc.insert(i, crabs.iter().filter(|c| **c == i).count() as i32);
    }

    let root = BitMapBackend::new(fname, (1024, 768)).into_drawing_area();
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .right_y_label_area_size(40)
        .margin(5)
        .caption("Dual Y-Axis Example", ("sans-serif", 50.0).into_font())
        .build_cartesian_2d(begin..end, (0.1f32..1e10f32).log_scale())
        .unwrap()
        .set_secondary_coord(begin..end, 0..10);

    chart
        .configure_mesh()
        // .disable_x_mesh()
        // .disable_y_mesh()
        .y_desc("Log Scale")
        .x_desc("Foo")
        // .y_label_formatter(&|x| format!("{:e}", x))
        .draw()
        .expect("A");

    chart
        .configure_secondary_axes()
        .y_desc("Linear Scale")
        .draw()
        .expect("b");
    chart
        .draw_series(LineSeries::new(
            (begin..=end).map(|x| {
                (
                    x,
                    match ans.get(&x) {
                        Some(v) => *v as f32,
                        None => 0.0,
                    },
                )
            }),
            &BLUE,
        ))
        .expect("c")
        .label("answer");

    chart
        .draw_secondary_series(LineSeries::new(
            (begin..=end).map(|x| {
                (
                    x,
                    match cc.get(&x) {
                        Some(v) => *v,
                        None => 0,
                    },
                )
            }),
            &RED,
        ))
        .expect("d")
        .label("crabs");

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.1))
        .draw()
        .expect("e");

    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
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
