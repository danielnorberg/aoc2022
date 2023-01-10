use std::iter::Map;
use std::str::{FromStr, Lines};

use itertools::{Group, Itertools};
use strum_macros::EnumString;

const INPUT: &str = include_str!("../../input/d11.txt");

fn main() {
    let monkeys = parse(INPUT);
    println!("monkeys: {:#?}", monkeys);
}


#[derive(Debug, PartialEq, Eq, Hash)]
enum Operand {
    Old,
    Constant(i32),
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Operation {
    Mul(Operand, Operand),
    Plus(Operand, Operand),
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Monkey {
    id: i32,
    items: Vec<i32>,
    operation: Operation,
    test_divisible_by: i32,
    if_true_throw_to_monkey: i32,
    if_false_throw_to_monkey: i32,
}

fn parse_monkey(s: Group<bool, Map<Lines, fn(&str) -> &str>, fn(&&str) -> bool>) -> Option<Monkey> {
    None
    // let mut parts = s.trim().split_whitespace();
    // let t = Inst::from_str(parts.next()?).ok()?;
    // let i = match t {
    //     Inst::Addx(_) => { Inst::Addx(parts.next()?.parse::<i32>().ok()?) }
    //     _ => t
    // };
    // Some(i)
}

fn parse(s: &str) -> Vec<Monkey> {
    let gs = s.lines()
        .map(|l| l.trim())
        .group_by(|l| l.is_empty());

    let mut monkeys = Vec::new();
    for (_, lines) in &gs {
        monkeys.push(parse_monkey(lines));
    }

    return gs
        .map(|x| Monkey {
            id: 0,
            items: vec![],
            operation: Operation::Mul(Operand::Old, Operand::Old),
            test_divisible_by: 0,
            if_true_throw_to_monkey: 0,
            if_false_throw_to_monkey: 0,
        })
        .collect_vec();
}

const COLS: i32 = 40;

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = include_str!("../../input/d10_sample1.txt");
    // const SAMPLE2: &str = include_str!("../../input/d10_sample2.txt");

    #[test]
    fn test1() {
        let monkeys = parse(SAMPLE1);
    }
}