use aoc2022::read_lines;
use std::collections::HashSet;

const LOWER_A: i32 = 'a' as i32;
const LOWER_Z: i32 = 'z' as i32;
const UPPER_A: i32 = 'A' as i32;
const UPPER_Z: i32 = 'Z' as i32;

fn prio(item: char) -> i32 {
    let v = item as i32;
    if (LOWER_A <= v) && (v <= LOWER_Z) {
        v + 1 - LOWER_A
    } else if (UPPER_A <= v) && (v <= UPPER_Z) {
        v + 27 - UPPER_A
    } else {
        panic!();
    }
}

fn main() {
    assert_eq!(16, prio('p'));
    assert_eq!(38, prio('L'));
    assert_eq!(42, prio('P'));
    assert_eq!(22, prio('v'));
    assert_eq!(20, prio('t'));
    assert_eq!(19, prio('s'));

    let mut sum_prio = 0;

    let lines = read_lines("input/d03.txt").unwrap().map(|l| l.unwrap());
    for line in lines {
        let (left, right) = line.split_at(line.len() / 2);
        let left_chars: HashSet<char> = left.chars().collect();
        let right_chars: HashSet<char> = right.chars().collect();
        let intersection = left_chars.intersection(&right_chars);
        for item in intersection {
            sum_prio += prio(*item);
        }
    }
    println!("{}", sum_prio);
}
