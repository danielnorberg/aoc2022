use std::str::FromStr;

fn main() {
    let lines = aoc2022::try_read_lines("input/d04.txt");
    let mut n = 0;
    for line in lines {
        let ranges: Vec<Vec<i32>> = line.splitn(2, ',')
            .map(|s| s.splitn(2, '-').map(|s| i32::from_str(s).unwrap()).collect())
            .collect();
        let a = ranges.get(0).unwrap();
        let b = ranges.get(1).unwrap();
        let a_begin = a.get(0);
        let a_end = a.get(1);
        let b_begin = b.get(0);
        let b_end = b.get(1);
        if a_begin <= b_begin && b_end <= a_end ||
            b_begin <= a_begin && a_end <= b_end {
            n += 1;
        }
    }
    println!("{}", n)
}
