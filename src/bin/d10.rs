use std::str::FromStr;

use itertools::Itertools;
use strum_macros::EnumString;

const INPUT: &str = include_str!("../../input/d10.txt");

fn main() {
    let insts = parse(INPUT);
    let (sum, rows) = execute(&insts);
    println!("sum: {}", sum);
    println!("{}", rows.join("\n"));
}

#[derive(Debug, PartialEq, Eq, Hash, EnumString)]
#[strum(ascii_case_insensitive)]
enum Inst {
    Noop,
    Addx(i32),
}

fn parse_row(s: &str) -> Option<Inst> {
    let mut parts = s.trim().split_whitespace();
    let t = Inst::from_str(parts.next()?).ok()?;
    let i = match t {
        Inst::Addx(_) => { Inst::Addx(parts.next()?.parse::<i32>().ok()?) }
        _ => t
    };
    Some(i)
}

fn parse(s: &str) -> Vec<Inst> {
    return s.lines()
        .flat_map(|l| parse_row(l))
        .collect_vec();
}

fn execute(insts: &Vec<Inst>) -> (i32, Vec<String>) {
    let mut cycle = 1;
    let mut reg = 1;
    let mut sum = 0;
    let mut col = 0;
    let mut row = String::new();
    let mut rows = Vec::<String>::new();
    for i in insts {
        let (cycles, result) = match i {
            Inst::Noop => {
                (1, reg)
            }
            Inst::Addx(x) => {
                (2, reg + x)
            }
        };
        for _ in 0..cycles {
            if (cycle - 20) % 40 == 0 {
                sum += cycle * reg;
            }
            if reg - 1 <= col && col <= reg + 1 {
                row.push('#');
            } else {
                row.push('.');
            }
            cycle += 1;
            col += 1;
            if col >= COLS {
                col = 0;
                rows.push(row);
                row = String::new();
                println!();
            }
        }
        reg = result;
    }
    (sum, rows)
}

const COLS: i32 = 40;

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = include_str!("../../input/d10_sample1.txt");
    const SAMPLE2: &str = include_str!("../../input/d10_sample2.txt");

    #[test]
    fn test1() {
        let insts = parse(SAMPLE1);
        let (sum, rows) = execute(&insts);
        println!("{:#?}", insts);
        println!("sum: {}", sum);
    }

    #[test]
    fn test2() {
        let insts = parse(SAMPLE2);
        let (sum, rows) = execute(&insts);
        println!("sum: {}", sum);
        println!("{}", rows.join("\n"));
    }
}