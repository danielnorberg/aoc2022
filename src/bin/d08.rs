use bit_vec::BitVec;
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/d08.txt");

fn main() {
    let grid = parse(INPUT);
    let (_, total_visible) = find_visible_trees(&grid);
    println!("part 1: {}", total_visible);
}

fn find_visible_trees(grid: &Vec<Vec<i32>>) -> (Vec<BitVec>, usize) {
    let mut visible = grid.iter().map(|r| BitVec::from_elem(r.len(), false)).collect_vec();

    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    for r in 0..height {
        mark(&grid, &mut visible, r, 0, 0, 1, height, width);
        mark(&grid, &mut visible, r, width - 1, 0, -1, height, width);
    }
    for c in 0..width {
        mark(&grid, &mut visible, 0, c, 1, 0, height, width);
        mark(&grid, &mut visible, height - 1, c, -1, 0, height, width);
    }

    let total_visible = visible.iter().map(|r| r.iter().filter(|c| *c).count()).sum::<usize>();
    (visible, total_visible)
}

fn mark(grid: &Vec<Vec<i32>>, visible: &mut Vec<BitVec>, mut ri: i32, mut ci: i32, dr: i32, dc: i32, height: i32, width: i32) {
    let mut max_tree = -1;
    while ri >= 0 && ri < height as i32 && ci >= 0 && ci < width as i32 {
        let tree = grid[ri as usize][ci as usize];
        if tree > max_tree {
            max_tree = tree;
            visible[ri as usize].set(ci as usize, true);
        }
        ri += dr;
        ci += dc;
    }
}

fn parse(s: &str) -> Vec<Vec<i32>> {
    let mut grid = Vec::<Vec<i32>>::new();
    for l in s.lines() {
        let mut row = Vec::<i32>::new();
        for c in l.trim().chars() {
            row.push(c.to_digit(10).unwrap() as i32);
        }
        grid.push(row);
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../input/d08_sample.txt");

    fn print_grid(grid: &Vec<Vec<i32>>) {
        grid.iter().for_each(|r| println!("{:?}", r));
    }

    #[test]
    fn test1() {
        let grid = parse(SAMPLE);
        println!("grid: ");
        print_grid(&grid);
        let (visible, total_visible) = find_visible_trees(&grid);
        println!("visible: {:#?}", visible);
        assert_eq!(total_visible, 21);
    }
}