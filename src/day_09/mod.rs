// sample A = 15
// sample B = 1134
// real A = 480
// real B = 1045660

use crate::common;
use plotters::prelude::*;
use std::collections::{HashSet, VecDeque};
use std::io::{BufWriter, Write};

const FILENAME: &str = "inputs/09.txt";

pub fn solve_a() -> u32 {
    let data = read_data();
    let mut ret: u32 = 0;
    for x in 0..data.len() {
        for y in 0..data[x].len() {
            let cell = data[x][y];
            if get_neighbor_coords(&data, x, y)
                .iter()
                .map(|c| data[c[0]][c[1]])
                .all(|c| c > cell)
            {
                ret += cell + 1;
            }
        }
    }

    ret
}

pub fn solve_b() -> u32 {
    let data = read_data();
    let mut basins: Vec<u32> = Vec::new();

    for x in 0..data.len() {
        for y in 0..data[x].len() {
            basins.push(bazin_size(&data, x, y));
        }
    }
    basins.sort();
    basins.reverse();
    basins.iter().take(3).map(|i| *i).product()
}

pub fn get_neighbor_coords(data: &Vec<Vec<u32>>, x: usize, y: usize) -> Vec<[usize; 2]> {
    let mut ret: Vec<[usize; 2]> = Vec::new();
    if x > 0 {
        ret.push([x - 1, y]);
    }
    if y > 0 {
        ret.push([x, y - 1]);
    }
    if x + 1 < data.len() {
        ret.push([x + 1, y]);
    }
    if y + 1 < data[0].len() {
        ret.push([x, y + 1]);
    }

    ret
}

pub fn bazin_size(data: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let mut ret = 1;
    let mut todo: VecDeque<[usize; 2]> = VecDeque::new();
    let mut seen: HashSet<[usize; 2]> = HashSet::new();
    let mut basin: Vec<[usize; 2]> = Vec::new();
    todo.push_back([x, y]);
    while !todo.is_empty() {
        // достаём координаты coords.
        // находим соседей, которые выше её
        // если сосед не в seen, то добавляем его в todo и делаем ret++
        let coord = todo.pop_front().unwrap();
        let cell = data[coord[0]][coord[1]];
        let neighborhood_coords = get_neighbor_coords(&data, coord[0], coord[1]);
        for neih_coords in neighborhood_coords {
            let neighbor = data[neih_coords[0]][neih_coords[1]];
            if neighbor > cell && neighbor != 9 && !seen.contains(&neih_coords) {
                seen.insert(neih_coords);
                todo.push_back(neih_coords);
                basin.push(neih_coords);
                ret += 1;
            }
        }
    }

    if ret > 100 {
        build_basin_html(&data, &basin, x, y);
    }

    ret
}

fn build_basin_html(data: &Vec<Vec<u32>>, basin: &Vec<[usize; 2]>, x: usize, y: usize) {
    let mut f = std::fs::OpenOptions::new()
        .append(false)
        .write(true)
        .create(true)
        .open("outputs/09.html")
        .expect("Open file");
    let mut f = BufWriter::new(f);
    f.write_all("<html><body>".as_bytes());
    for c1 in 0..data.len() {
        for c2 in 0..data[c1].len() {
            let color = if c1 == x && c2 == y {
                "red"
            } else if basin.contains(&[c1, c2]) {
                "pink"
            } else {
                "gray"
            };
            f.write_all(("<span style='color:".to_owned() + color + "'>").as_bytes());
            f.write_all(data[c1][c2].to_string().as_bytes());
            f.write_all("</span>".as_bytes());
        }
        f.write_all("<br />".as_bytes());
    }
    f.write_all("</body></html>".as_bytes());
}

fn read_data() -> Vec<Vec<u32>> {
    let mut lines = common::read_lines2(FILENAME);

    let steps: Vec<Vec<u32>> = lines
        .map(|l| {
            l.unwrap()
                .as_str()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    steps
}
