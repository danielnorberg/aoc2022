use std::collections::HashSet;
use std::ops;
use std::str::FromStr;

use itertools::Itertools;
use strum_macros::EnumString;

const INPUT: &str = include_str!("../../input/d09.txt");

fn main() {
    let moves = parse(INPUT);
    let n = count_tail_positions(&moves, 2);
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

impl ops::Add<Pos> for Pos {
    type Output = Pos;

    fn add(self, _rhs: Pos) -> Pos {
        Pos { x: self.x + _rhs.x, y: self.y + _rhs.y }
    }
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

fn count_tail_positions(moves: &Vec<Move>, n_knots: usize) -> usize {
    let mut knots = vec![Pos { x: 0, y: 0 }; n_knots];
    let mut positions = HashSet::<Pos>::new();
    for m in moves {
        for _ in 0..m.n {
            let head = &mut knots[0];
            head_step(head, &m.d);
            for i in 0..n_knots - 1 {
                let k1 = knots[i];
                let k2 = &mut knots[i + 1];
                knot_step(k1, k2);
            }
            positions.insert(*knots.last().unwrap());
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

fn knot_step(h: Pos, t: &mut Pos) {
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
        let n = count_tail_positions(&moves, 2);
        assert_eq!(13, n);
    }
}