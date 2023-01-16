extern crate core;

use std::cmp::{max, min};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt;
use std::ops::AddAssign;
use std::str::FromStr;

use itertools::{enumerate, Itertools};
use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &str = include_str!("../../input/d15.txt");

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Sensor {
    point: Point,
    beacon: Point,
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
enum Item {
    Sensor,
    Beacon,
    Nothing,
}

#[derive(Debug)]
struct Map {
    rows: HashMap<i32, HashMap<i32, Item>>,
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x={}, y={}", self.x, self.y)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

fn main() {
    {
        let sensors = parse(INPUT);
        println!("{:#?}", &sensors);
        let n = count_row(&sensors, 2000000);
        println!("{}", n);
    }
}

fn parse(s: &str) -> Vec<Sensor> {
    s.lines()
        .map(|l| parse_sensor(l.trim()).unwrap())
        .collect_vec()
}

fn parse_sensor(s: &str) -> Option<Sensor> {
    lazy_static! {
        static ref SENSOR_RE: Regex = Regex::new(r"^Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)$").unwrap();
    }
    // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    let c = SENSOR_RE.captures(s)?;
    Some(Sensor {
        point: Point {
            x: i32::from_str(c.name("sx")?.as_str()).ok()?,
            y: i32::from_str(c.name("sy")?.as_str()).ok()?,
        },
        beacon: Point {
            x: i32::from_str(c.name("bx")?.as_str()).ok()?,
            y: i32::from_str(c.name("by")?.as_str()).ok()?,
        },
    })
}

fn draw_map(sensors: &Vec<Sensor>) -> Map {
    let mut map = Map {
        rows: HashMap::new(),
        left: i32::MAX,
        right: i32::MIN,
        top: i32::MAX,
        bottom: i32::MIN,
    };
    for s in sensors {
        insert(&mut map, s.point.x, s.point.y, Item::Sensor);
        insert(&mut map, s.beacon.x, s.beacon.y, Item::Beacon);
        // Draw non-beacon cells
        let dst = manhattan_distance(&s.point, &s.beacon);
        for dy in -dst..dst + 1 {
            let y = s.point.y + dy;
            let rx = dst - dy.abs();
            for dx in -rx..rx + 1 {
                let x = s.point.x + dx;
                insert(&mut map, x, y, Item::Nothing);
            }
        }
    }
    map
}

fn count_row(sensors: &Vec<Sensor>, row_nr: i32) -> i32 {
    let mut beacons = sensors
        .iter()
        .filter(|s| s.beacon.y == row_nr)
        .map(|s| s.beacon.x)
        .unique()
        .sorted()
        .collect_vec();
    beacons.reverse();
    let mut intersections = Vec::<(i32, i32)>::new();
    for s in sensors {
        let dst = manhattan_distance(&s.point, &s.beacon);
        let min_y = s.point.y - dst;
        let max_y = s.point.y + dst;
        let intersects_row = min_y <= row_nr && row_nr <= max_y;
        if !intersects_row {
            continue;
        }
        let dy = (s.point.y - row_nr).abs();
        let rx = dst - dy;
        let rl = s.point.x - rx;
        let rr = s.point.x + rx;
        intersections.push((rl, rr));
    }
    intersections.sort();
    if intersections.is_empty() {
        return 0;
    }
    intersections.push((i32::MAX, i32::MAX));
    let mut n = 0;
    let mut r = intersections[0];
    for i in &intersections[1..] {
        if i.0 <= r.1 {
            r.1 = max(r.1, i.1);
        } else {
            let mut bn = 0;
            while !beacons.is_empty() {
                let bx = *beacons.last().unwrap();
                if bx >= r.0 && bx <= r.1 {
                    bn += 1;
                    beacons.pop();
                }
            }
            let rn = 1 + r.1 - r.0 - bn;
            n += rn;
            r = *i;
        }
    }
    n
}

fn insert(map: &mut Map, x: i32, y: i32, item: Item) {
    map.left = min(map.left, x);
    map.right = max(map.right, x);
    map.top = min(map.top, y);
    map.bottom = max(map.bottom, y);
    map.rows
        .entry(y)
        .or_insert_with(HashMap::new)
        .entry(x)
        .or_insert(item);
}

fn manhattan_distance(a: &Point, b: &Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn print_map(map: &Map) {
    for y in map.top..map.bottom + 1 {
        let r = map.rows.get(&y);
        print!("{:>6} ", y);
        for x in map.left..map.right + 1 {
            match r.map(|r| r.get(&x)).flatten() {
                Some(Item::Sensor) => print!("S"),
                Some(Item::Beacon) => print!("B"),
                Some(Item::Nothing) => print!("#"),
                None => print!("."),
            }
        }
        println!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = include_str!("../../input/d15_sample1.txt");

    #[test]
    fn test1() {
        let sensors = parse(SAMPLE1);
        println!("{:#?}", &sensors);
        let mut map = draw_map(&sensors);
        print_map(&map);
        let n10 = count_row(&sensors, 10);
        assert_eq!(n10, 26);
        let n11 = count_row(&sensors, 11);
        assert_eq!(n11, 28);
    }
}
