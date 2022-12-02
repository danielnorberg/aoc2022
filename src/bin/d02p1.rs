use aoc2022::read_lines;

// A X: Rock
// B Y: Paper
// C Z: Scissor

fn shape_score(shape: &str) -> i32 {
    match shape {
        "X" => { 1 }
        "Y" => { 2 }
        "Z" => { 3 }
        _ => { panic!() }
    }
}

fn win_score(their_shape: &str, our_shape: &str) -> i32 {
    match (their_shape, our_shape) {
        ("C", "X") => { 6 }
        ("A", "Y") => { 6 }
        ("B", "Z") => { 6 }
        ("A", "X") => { 3 }
        ("B", "Y") => { 3 }
        ("C", "Z") => { 3 }
        (_, _) => { 0 }
    }
}

fn main() {
    let lines = read_lines("input/d02.txt").unwrap();
    let mut score = 0;
    for line in lines {
        let s = line.unwrap();
        let shapes: Vec<&str> = s.split(" ").collect();
        score += shape_score(shapes[1]) + win_score(shapes[0], shapes[1]);
    }
    println!("{}", score);
}
