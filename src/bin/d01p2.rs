use aoc2022::read_lines;
use std::collections::BinaryHeap;

fn main() {
    let lines = read_lines("input/d01.txt").unwrap();
    let mut heap = BinaryHeap::new();
    let mut sum_cals = 0;
    for line in lines {
        let s = line.unwrap();
        let r = s.trim();
        if r.is_empty() {
            heap.push(sum_cals);
            sum_cals = 0;
            continue;
        }
        let c: i32 = r.parse().unwrap();
        sum_cals += c;
    }
    let sum_top_three = heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap();
    println!("{}", sum_top_three);
}
