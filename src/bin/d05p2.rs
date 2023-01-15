use std::ops::{Index, IndexMut};
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    let lines = aoc2022::try_read_lines("input/d05.txt");
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut instructions: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        } else if line.chars().next().unwrap() == 'm' {
            parse_instructions(&mut instructions, line)
        } else {
            parse_stacks(&mut stacks, line)
        }
    }

    for i in 0..stacks.len() {
        stacks.index_mut(i).reverse();
    }

    for instruction in instructions {
        let n = *instruction.get(0).unwrap();
        let from = *instruction.get(1).unwrap() as usize;
        let to = *instruction.get(2).unwrap() as usize;
        let from_stack = stacks.index_mut(from - 1);
        let mut crates: Vec<char> = Vec::new();
        for _ in 0..n {
            let c = from_stack.pop().unwrap();
            crates.push(c);
        }
        let to_stack = stacks.index_mut(to - 1);
        for c in crates.iter().rev() {
            to_stack.push(*c);
        }
    }

    for i in 0..stacks.len() {
        let stack = stacks.index(i);
        print!("{}", stack.index(stack.len() - 1));
    }
    println!();
}

fn parse_instructions(instructions: &mut Vec<Vec<i32>>, line: String) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    }
    let caps = RE.captures(line.as_str()).unwrap();
    let instruction = caps
        .iter()
        .skip(1)
        .map(|c| i32::from_str(c.unwrap().as_str()).unwrap())
        .collect();
    instructions.push(instruction)
}

fn parse_stacks(stacks: &mut Vec<Vec<char>>, line: String) {
    let chars: Vec<char> = line.chars().collect();
    let mut j = 0;
    let mut i = 0;
    while i < chars.len() {
        if *chars.get(i).unwrap() == '[' {
            assert_eq!(*chars.get(i + 2).unwrap(), ']');
            let c = chars.get(i + 1).unwrap();
            while stacks.len() < j + 1 {
                stacks.push(Vec::new());
            }
            stacks.index_mut(j).push(*c);
        }
        j += 1;
        i += 4;
    }
}
