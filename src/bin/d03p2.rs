use std::collections::HashSet;

use aoc2022::read_lines;

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

    let lines: Vec<String> = read_lines("input/d03.txt")
        .unwrap()
        .map(|l| l.unwrap())
        .collect();
    let groups = lines.chunks(3);
    for group in groups {
        let group_intersection: HashSet<char> = group
            .iter()
            .map(|g| HashSet::from_iter(g.chars()))
            .reduce(|acc, g| acc.intersection(&g).copied().collect())
            .unwrap();
        let badge = group_intersection.iter().next().copied().unwrap();
        sum_prio += prio(badge);
    }
    println!("{}", sum_prio);
}
