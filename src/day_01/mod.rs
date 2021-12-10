use crate::common;

#[allow(dead_code)]
pub fn solve() {
    let mut v: Vec<i32> = Vec::new();

    let lines = common::read_lines("inputs/01a.txt").expect("Error reading file");
    for line in lines {
        if let Ok(s) = line {
            let cur: i32 = s.parse().expect("Wrong file format");
            v.push(cur);
        }
    }

    let mut cnt1 = 0;
    for pos in 1..v.len() {
        let prev: i32 = v[pos - 1];
        let cur: i32 = v[pos];
        if cur > prev {
            cnt1 += 1;
        }
    }
    println!("1 {} ", cnt1);

    let mut cnt2 = 0;
    for pos in 1..v.len() - 2 {
        let prev: i32 = v[pos - 1..=pos + 1].iter().sum();
        let cur: i32 = v[pos..=pos + 2].iter().sum();
        if cur > prev {
            cnt2 += 1;
        }
    }
    println!("2 {} ", cnt2);
}
