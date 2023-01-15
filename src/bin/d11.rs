use crate::Operand::{Constant, Old};
use crate::Operation::{Mul, Plus};
use itertools::Itertools;
use std::collections::VecDeque;
use std::i32;
use std::str::FromStr;

const INPUT: &str = include_str!("../../input/d11.txt");

fn main() {
    {
        let mut monkeys = parse(INPUT);
        play(&mut monkeys, 20, 3);
    }

    {
        let mut monkeys = parse(INPUT);
        play(&mut monkeys, 10000, 1);
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Operand {
    Old,
    Constant(i64),
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Operation {
    Mul(Operand, Operand),
    Plus(Operand, Operand),
}

impl Operand {
    pub(crate) fn value(&self, old: i64) -> i64 {
        match self {
            Old => old,
            Constant(c) => *c,
        }
    }
}

impl Operation {
    pub(crate) fn worry_level(&self, old: i64) -> i64 {
        match self {
            Mul(a, b) => a.value(old) * b.value(old),
            Plus(a, b) => a.value(old) + b.value(old),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Monkey {
    id: i32,
    items: VecDeque<i64>,
    operation: Operation,
    test_divisible_by: i64,
    if_true_throw_to_monkey: usize,
    if_false_throw_to_monkey: usize,
    inspections: i64,
}

fn parse_monkey<'a, I>(s: I) -> Option<Monkey>
where
    I: IntoIterator<Item = &'a str>,
{
    let mut i = s.into_iter();
    Some(Monkey {
        id: i32::from_str(
            i.next()?
                .trim()
                .strip_prefix("Monkey ")?
                .strip_suffix(":")?
                .trim(),
        )
        .ok()?,
        items: i
            .next()?
            .trim()
            .strip_prefix("Starting items: ")?
            .split(", ")
            .flat_map(|s| i64::from_str(s.trim()))
            .collect(),
        operation: {
            let mut tokens = i
                .next()?
                .trim()
                .strip_prefix("Operation: new =")?
                .split_whitespace();
            let a = operand(tokens.next()?)?;
            let o = tokens.next()?;
            let b = operand(tokens.next()?)?;
            match o {
                "*" => Mul(a, b),
                "+" => Plus(a, b),
                _ => panic!(),
            }
        },
        test_divisible_by: i64::from_str(i.next()?.trim().strip_prefix("Test: divisible by ")?)
            .ok()?,
        if_true_throw_to_monkey: usize::from_str(
            i.next()?.trim().strip_prefix("If true: throw to monkey ")?,
        )
        .ok()?,
        if_false_throw_to_monkey: usize::from_str(
            i.next()?
                .trim()
                .strip_prefix("If false: throw to monkey ")?,
        )
        .ok()?,
        inspections: 0,
    })
}

fn operand(s: &str) -> Option<Operand> {
    match s {
        "old" => Some(Old),
        s => Some(Constant(i64::from_str(s).ok()?)),
    }
}

fn parse(s: &str) -> Vec<Monkey> {
    let gs = s.lines().map(|l| l.trim()).group_by(|l| l.is_empty());
    let mut monkeys = Vec::new();
    for (_, lines) in &gs {
        if let Some(monkey) = parse_monkey(lines) {
            monkeys.push(monkey);
        }
    }
    monkeys
}

fn round(monkeys: &mut Vec<Monkey>, worry_level_divisor: i64) {
    let test_divisor_product = monkeys
        .iter()
        .map(|m| m.test_divisible_by)
        .reduce(|a, b| a * b)
        .unwrap();
    for i in 0..monkeys.len() {
        let n = monkeys[i].items.len();
        for _ in 0..n {
            let monkey = &mut monkeys[i];
            monkey.inspections += 1;
            let item = monkey.items.pop_front().unwrap();
            let new_item =
                (monkey.operation.worry_level(item) / worry_level_divisor) % test_divisor_product;
            let next_monkey = if new_item.clone() % monkey.test_divisible_by == 0 {
                monkey.if_true_throw_to_monkey
            } else {
                monkey.if_false_throw_to_monkey
            };
            // println!("monkey {} throws {} to {}", monkey.id, new_item, next_monkey);
            monkeys[next_monkey].items.push_back(new_item);
        }
    }
}

fn play(monkeys: &mut Vec<Monkey>, rounds: usize, worry_level_divisor: i64) -> i64 {
    for _ in 0..rounds {
        round(monkeys, worry_level_divisor);
    }
    monkeys.iter().for_each(|monkey| {
        println!(
            "Monkey {} inspected items {} times.",
            monkey.id, monkey.inspections
        )
    });
    monkeys.sort_by_key(|m| -m.inspections);
    let business = monkeys
        .iter()
        .take(2)
        .map(|m| m.inspections)
        .reduce(|a, b| a * b)
        .unwrap();
    println!("Monkey business: {}", business);
    business
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = include_str!("../../input/d11_sample1.txt");
    // const SAMPLE2: &str = include_str!("../../input/d10_sample2.txt");

    #[test]
    fn test1() {
        let mut monkeys = parse(SAMPLE1);
        let business = play(&mut monkeys, 20, 3);
        assert_eq!(business, 10605);
    }

    #[test]
    fn test2() {
        let mut monkeys = parse(SAMPLE1);
        let business = play(&mut monkeys, 10000, 1);
        assert_eq!(business, 2713310158);
    }
}
