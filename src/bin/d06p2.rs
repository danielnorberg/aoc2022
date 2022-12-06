use itertools::Itertools as _;

const INPUT: &str = include_str!("../../input/d06.txt");

fn main() {
    let datastream = INPUT;
    let n = find_message(&datastream, 14).unwrap();
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
        assert_eq!(find_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14).unwrap(), 19);
        assert_eq!(find_message("bvwbjplbgvbhsrlpgdmjqwftvncz", 14).unwrap(), 23);
        assert_eq!(find_message("nppdvjthqldpwncqszvftbrmjlhg", 14).unwrap(), 23);
        assert_eq!(find_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14).unwrap(), 29);
        assert_eq!(find_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14).unwrap(), 26);
    }
}