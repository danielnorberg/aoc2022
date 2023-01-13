use std::collections::VecDeque;
use std::i32;

use itertools::{enumerate, Itertools};

const INPUT: &str = include_str!("../../input/d12.txt");

fn main() {
    let mut game = parse(INPUT);

    {
        let start = game.start;
        let dst = bfs(&mut game, start).unwrap();
        println!("{:#?}", dst);
    }

    {
        let starts = find_starts(&game.map);
        let min_dst = starts.iter().flat_map(|start| {
            reset(&mut game);
            bfs(&mut game, *start)
        }).min().unwrap();
        println!("{:#?}", min_dst);
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Square {
    z: i32,
    v: bool,
    n: i32,
}

type Map = Vec<Vec<Square>>;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Game {
    map: Map,
    start: Point,
    end: Point,
    width: i32,
    height: i32,
}

fn reset(game: &mut Game) {
    for row in &mut game.map {
        for s in row {
            s.n = 0;
            s.v = false;
        }
    }
}

fn bfs(game: &mut Game, start: Point) -> Option<i32> {
    let mut q = VecDeque::<Point>::new();
    game.map[start.y as usize][start.x as usize].v = true;
    q.push_back(start);
    while !q.is_empty() {
        let s = q.pop_front().unwrap();
        let cz;
        let cn;
        {
            let cs = &game.map[s.y as usize][s.x as usize];
            cz = cs.z;
            cn = cs.n;
        }
        if s == game.end {
            return Some(cn);
        }
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let ny = s.y + dy;
            let nx = s.x + dx;
            if ny < 0 || ny >= game.height || nx < 0 || nx >= game.width {
                continue;
            }
            let ns = &mut game.map[ny as usize][nx as usize];
            if ns.v {
                continue;
            }
            if ns.z > cz + 1 {
                continue;
            }
            ns.n = cn + 1;
            ns.v = true;
            q.push_back(Point { x: nx, y: ny });
        }
    }
    None
}

fn parse(s: &str) -> Game {
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;
    let map = s.lines().enumerate()
        .map(|(y, r)| r.chars().enumerate().map(|(x, c)| {
            let z = if c == 'S' {
                start = Some(Point { x: x as i32, y: y as i32 });
                'a' as i32
            } else if c == 'E' {
                end = Some(Point { x: x as i32, y: y as i32 });
                'z' as i32
            } else {
                c as i32
            };
            Square { z: z - ('a' as i32), v: false, n: 0 }
        }).collect_vec())
        .collect_vec();
    let width = map[0].len() as i32;
    let height = map.len() as i32;
    Game { map, start: start.unwrap(), end: end.unwrap(), width, height }
}

fn find_starts(map: &Map) -> Vec<Point> {
    enumerate(map)
        .flat_map(|(y, r)|
            enumerate(r)
                .filter(|(_, s)| s.z == 0)
                .map(|(x, _)| Point { x: x as i32, y: y as i32 })
                .collect_vec())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = include_str!("../../input/d12_sample1.txt");

    #[test]
    fn test1() {
        let mut game = parse(SAMPLE1);
        let start = game.start;
        let dst = bfs(&mut game, start);
        println!("{:#?}", dst);
    }

    #[test]
    fn test2() {
        let mut game = parse(SAMPLE1);
        let starts = find_starts(&game.map);
        let min_dst = starts.iter().flat_map(|start| {
            reset(&mut game);
            bfs(&mut game, *start)
        }).min().unwrap();
        println!("{:#?}", min_dst);
    }
}