use aoc2022::try_read_lines;
use std::cmp::max;

fn main() {
    let lines = try_read_lines("input/d01.txt");
    let mut max_cals = 0;
    let mut sum_cals = 0;
    for line in lines {
        let r = line.trim();
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
