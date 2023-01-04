use std::collections::HashSet;
use std::str::FromStr;

use itertools::Itertools;
use strum_macros::EnumString;

const INPUT: &str = include_str!("../../input/d09.txt");

fn main() {
    let moves = parse(INPUT);
    let n = count_positions(&moves);
    println!("positions: {}", n);
}

#[derive(Debug, PartialEq, Eq, Hash, EnumString)]
enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Move {
    d: Direction,
    n: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

fn parse_row(s: &str) -> Option<Move> {
    let mut parts = s.trim().split_whitespace();
    let d = Direction::from_str(parts.next()?).ok()?;
    let n = parts.next()?.parse::<i32>().ok()?;
    Some(Move { d, n })
}

fn parse(s: &str) -> Vec<Move> {
    return s.lines()
        .flat_map(|l| parse_row(l))
        .collect_vec();
}

fn count_positions(moves: &Vec<Move>) -> usize {
    let mut head = Pos { x: 0, y: 0 };
    let mut tail = Pos { x: 0, y: 0 };
    let mut positions = HashSet::<Pos>::new();
    for m in moves {
        for _ in 0..m.n {
            head_step(&mut head, &m.d);
            tail_step(&mut tail, &head);
            positions.insert(tail);
        }
    }
    positions.len()
}

fn head_step(h: &mut Pos, d: &Direction) {
    match d {
        Direction::U => h.y += 1,
        Direction::D => h.y -= 1,
        Direction::L => h.x -= 1,
        Direction::R => h.x += 1,
    };
}

fn tail_step(t: &mut Pos, h: &Pos) {
    if (h.x - t.x).abs() > 1 || (h.y - t.y).abs() > 1 {
        t.x += (h.x - t.x).signum();
        t.y += (h.y - t.y).signum();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../input/d09_sample.txt");

    #[test]
    fn test1() {
        let moves = parse(SAMPLE);
        println!("moves: {:#?}", moves);
        let n = count_positions(&moves);
        println!("positions: {}", n);
    }
}