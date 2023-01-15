use aoc2022::read_lines;

// A Rock
// B Paper
// C Scissor

// X Lose
// Y Draw
// Z Win

fn shape_score(shape: &str) -> i32 {
    match shape {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => {
            panic!()
        }
    }
}

fn win_score(result: &str) -> i32 {
    match result {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => {
            panic!()
        }
    }
}

fn our_shape(their_shape: &str, result: &str) -> String {
    match (their_shape, result) {
        ("A", "X") => "C",
        ("A", "Z") => "B",
        ("B", "X") => "A",
        ("B", "Z") => "C",
        ("C", "X") => "B",
        ("C", "Z") => "A",
        (_, "Y") => their_shape,
        (_, _) => {
            panic!()
        }
    }
    .parse()
    .unwrap()
}

fn main() {
    let lines = read_lines("input/d02.txt").unwrap();
    let mut score = 0;
    for line in lines {
        let s = line.unwrap();
        let shapes: Vec<&str> = s.split(" ").collect();
        score += shape_score(&our_shape(shapes[0], shapes[1])) + win_score(shapes[1]);
    }
    println!("{}", score);
}
