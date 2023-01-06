use std::str::FromStr;

use itertools::Itertools;
use strum_macros::EnumString;

const INPUT: &str = include_str!("../../input/d10.txt");

fn main() {
    let insts = parse(INPUT);
    let sum = signal_strength_sum(&insts);
    println!("sum: {}", sum);
}

#[derive(Debug, PartialEq, Eq, Hash, EnumString)]
enum Inst {
    noop,
    addx(i32),
}

fn parse_row(s: &str) -> Option<Inst> {
    let mut parts = s.trim().split_whitespace();
    let t = Inst::from_str(parts.next()?).ok()?;
    let i = match t {
        Inst::addx(_) => { Inst::addx(parts.next()?.parse::<i32>().ok()?) }
        _ => t
    };
    Some(i)
}

fn parse(s: &str) -> Vec<Inst> {
    return s.lines()
        .flat_map(|l| parse_row(l))
        .collect_vec();
}

fn inc_cycle(cycle: &mut i32, reg: i32, sum: &mut i32) {
    if (*cycle - 20) % 40 == 0 {
        *sum += *cycle * reg;
        println!("cycle: {}, reg: {}, sum: {}", *cycle, reg, *sum);
    }
    *cycle += 1;
}

fn signal_strength_sum(insts: &Vec<Inst>) -> i32 {
    let mut cycle = 1;
    let mut reg = 1;
    let mut sum = 0;
    for i in insts {
        match i {
            Inst::noop => {
                inc_cycle(&mut cycle, reg, &mut sum);
            }
            Inst::addx(x) => {
                inc_cycle(&mut cycle, reg, &mut sum);
                inc_cycle(&mut cycle, reg, &mut sum);
                reg += x;
            }
        }
    }
    // inc_cycle(&mut cycle, reg, &mut sum);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = include_str!("../../input/d10_sample1.txt");
    const SAMPLE2: &str = include_str!("../../input/d10_sample2.txt");

    #[test]
    fn test1() {
        let insts = parse(SAMPLE1);
        let sum = signal_strength_sum(&insts);
        println!("{:#?}", insts);
        println!("sum: {}", sum);
    }

    #[test]
    fn test2() {
        let insts = parse(SAMPLE2);
        let sum = signal_strength_sum(&insts);
        println!("sum: {}", sum);
    }
}