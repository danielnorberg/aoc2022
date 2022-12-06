use itertools::Itertools as _;

const INPUT: &str = include_str!("../../input/d06.txt");

fn main() {
    let datastream = INPUT;
    let n = find_message(&datastream, 4).unwrap();
    println!("{}", n);
}

fn find_message(datastream: &str, message_len: usize) -> Option<usize> {
    for i in 0..datastream.len() - message_len {
        let n = i + message_len;
        let window = &datastream[i..n];
        if window.chars().unique().count() == message_len {
            return Some(n);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(find_message("bvwbjplbgvbhsrlpgdmjqwftvncz", 4).unwrap(), 5);
        assert_eq!(find_message("nppdvjthqldpwncqszvftbrmjlhg", 4).unwrap(), 6);
        assert_eq!(find_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4).unwrap(), 10);
        assert_eq!(find_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4).unwrap(), 11);
    }
}