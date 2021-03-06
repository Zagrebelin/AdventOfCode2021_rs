use crate::common;

#[allow(dead_code)]
pub fn solve_a() -> i32 {
    let mut position = 0;
    let mut depth = 0;
    let lines = common::read_lines("inputs/02a.txt").expect("Error reading input file");
    for line in lines {
        if let Ok(s) = line {
            let parts: Vec<&str> = s.split_ascii_whitespace().collect();
            let distance: i32 = parts[1].parse().expect("Wrong file format");
            match parts[0] {
                "forward" => {
                    position += distance;
                }
                "down" => {
                    depth += distance;
                }
                "up" => {
                    depth -= distance;
                }
                _ => {}
            }
        }
    }
    return position * depth;
}

#[allow(dead_code)]
pub fn solve_b() -> i32 {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;
    let lines = common::read_lines("inputs/02a.txt").expect("Error reading input file");
    for line in lines {
        if let Ok(s) = line {
            let parts: Vec<&str> = s.split_ascii_whitespace().collect();
            let distance: i32 = parts[1].parse().expect("Wrong file format");
            match parts[0] {
                "forward" => {
                    position += distance;
                    depth += aim * distance;
                }
                "down" => {
                    aim += distance;
                }
                "up" => {
                    aim -= distance;
                }
                _ => {}
            }
        }
    }
    return position * depth;
}
