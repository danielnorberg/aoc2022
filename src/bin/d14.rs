extern crate core;


use std::collections::HashMap;
use std::fmt;
use std::iter::zip;
use std::ops::AddAssign;
use std::str::FromStr;

use itertools::{enumerate, Itertools};

use crate::Contents::{Rock, Sand, SandEntry};

const INPUT: &str = include_str!("../../input/d14.txt");

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub(crate) fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

#[derive(Debug)]
enum Contents {
    Rock,
    Sand,
    SandEntry,
}

struct Path {
    points: Vec<Point>,
}

#[derive(Debug)]
struct Map {
    cells: HashMap<Point, Contents>,
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, p) in enumerate(&self.points) {
            if i > 0 {
                write!(f, " -> ")?;
            }
            write!(f, "{}", p)?;
        }
        Ok(())
    }
}


fn main() {
    let paths = parse(INPUT);
    print_paths(&paths);
    let mut map = draw_map(&paths);
    println!();
    print_map(&map);
    let sand = pour_sand(&mut map);
    println!();
    print_map(&map);
    println!("sand: {}", sand);
}

fn parse(s: &str) -> Vec<Path> {
    s.lines().map(|l| parse_path(l.trim())).collect_vec()
}

fn parse_path(s: &str) -> Path {
    let points = s.split(" -> ")
        .map(|p| p
            .split_once(',').unwrap())
        .map(|(x, y)|
            Point {
                x: i32::from_str(x).unwrap(),
                y: i32::from_str(y).unwrap(),
            })
        .collect_vec();
    Path { points }
}

fn print_paths(paths: &Vec<Path>) {
    paths.iter().for_each(|p| println!("{}", p))
}


fn draw_map(paths: &Vec<Path>) -> Map {
    let mut left = i32::MAX;
    let mut right = i32::MIN;
    let mut top = i32::MAX;
    let mut bottom = i32::MIN;
    let mut cells = HashMap::new();
    cells.insert(Point::new(500, 0), SandEntry);
    for path in paths {
        for (a, b) in zip(
            &path.points[0..path.points.len() - 1],
            &path.points[1..path.points.len()]) {
            left = [left, a.x, b.x].into_iter().min().unwrap();
            right = [right, a.x, b.x].into_iter().max().unwrap();
            top = [top, a.y, b.y, 0].into_iter().min().unwrap();
            bottom = [bottom, a.y, b.y].into_iter().max().unwrap();
            let d = Point {
                x: (b.x - a.x).signum(),
                y: (b.y - a.y).signum(),
            };
            let mut s = *a;
            cells.insert(s, Rock);
            while s != *b {
                s += d;
                cells.insert(s, Rock);
            }
        }
    }
    Map { cells, left, right, top, bottom }
}

fn print_map(map: &Map) {
    for y in map.top..map.bottom + 1 {
        for x in map.left..map.right + 1 {
            let p = Point { x, y };
            match &map.cells.get(&p) {
                None => print!("."),
                Some(Rock) => print!("#"),
                Some(Sand) => print!("o"),
                Some(SandEntry) => print!("+"),
            }
        }
        println!()
    }
}

fn add_sand(map: &mut Map, mut p: Point) -> bool {
    loop {
        let down = Point { x: p.x, y: p.y + 1 };
        let down_left = Point { x: p.x - 1, y: p.y + 1 };
        let down_right = Point { x: p.x + 1, y: p.y + 1 };
        if map.cells.get(&down).is_none() {
            p = down;
        } else if map.cells.get(&down_left).is_none() {
            p = down_left;
        } else if map.cells.get(&down_right).is_none() {
            p = down_right;
        } else {
            break;
        }
        if p.y >= map.bottom {
            return false;
        }
    }
    map.cells.insert(p, Sand);
    true
}

fn pour_sand(mut map: &mut Map) -> i32 {
    for i in 0.. {
        let rested = add_sand(&mut map, Point::new(500, 0));
        if !rested {
            return i;
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = include_str!("../../input/d14_sample1.txt");

    #[test]
    fn test1() {
        let paths = parse(SAMPLE1);
        print_paths(&paths);
        let mut map = draw_map(&paths);
        println!();
        print_map(&map);
        let sand = pour_sand(&mut map);
        println!();
        print_map(&map);
        assert_eq!(sand, 24);
    }
}