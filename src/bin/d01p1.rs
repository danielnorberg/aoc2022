use std::cmp::max;
use aoc2022::read_lines;

fn main() {
    let lines = read_lines("input/d01.txt").unwrap().lines();
    let mut max_cals = 0;
    let mut sum_cals = 0;
    for line in lines {
        let s = line.unwrap();
        let r = s.trim();
        if r.is_empty() {
            sum_cals = 0;
            continue;
        }
        let c: i32 = r.parse().unwrap();
        sum_cals += c;
        max_cals = max(max_cals, sum_cals);
    }
    println!("{}", max_cals);
}
