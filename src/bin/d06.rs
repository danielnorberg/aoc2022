use itertools::Itertools as _;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

const INPUT: &str = include_str!("../../input/d06.txt");

fn main() {
    println!("{}", find_message(&INPUT, 4).unwrap());
    println!("{}", find_message_hist(&INPUT, 4).unwrap());
    println!("{}", find_message(&INPUT, 14).unwrap());
    println!("{}", find_message_hist(&INPUT, 14).unwrap());
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

fn find_message_hist(datastream: &str, message_len: usize) -> Option<usize> {
    let mut hist = HashMap::<char, usize>::new();
    let mut unique = 0;
    let chars = datastream.chars().collect_vec();
    for i in 0..chars.len() {
        let c = chars[i];
        match hist.entry(c) {
            Entry::Occupied(mut o) => {
                o.insert(o.get() + 1);
            }
            Entry::Vacant(v) => {
                unique += 1;
                v.insert(1);
            }
        }
        if i < message_len {
            continue;
        }
        let old_c = chars[i - message_len];
        match hist.entry(old_c) {
            Entry::Occupied(mut o) => {
                let n = o.get();
                if *n == 1 {
                    o.remove();
                    unique -= 1;
                } else {
                    o.insert(n - 1);
                }
            }
            Entry::Vacant(_) => {
                panic!();
            }
        }
        if unique == message_len {
            return Some(i + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(find_message("bvwbjplbgvbhsrlpgdmjqwftvncz", 4).unwrap(), 5);
        assert_eq!(find_message("nppdvjthqldpwncqszvftbrmjlhg", 4).unwrap(), 6);
        assert_eq!(
            find_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4).unwrap(),
            10
        );
        assert_eq!(
            find_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4).unwrap(),
            11
        );
    }

    #[test]
    fn test1_hist() {
        assert_eq!(
            find_message_hist("bvwbjplbgvbhsrlpgdmjqwftvncz", 4).unwrap(),
            5
        );
        assert_eq!(
            find_message_hist("nppdvjthqldpwncqszvftbrmjlhg", 4).unwrap(),
            6
        );
        assert_eq!(
            find_message_hist("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4).unwrap(),
            10
        );
        assert_eq!(
            find_message_hist("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4).unwrap(),
            11
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            find_message("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14).unwrap(),
            19
        );
        assert_eq!(
            find_message("bvwbjplbgvbhsrlpgdmjqwftvncz", 14).unwrap(),
            23
        );
        assert_eq!(
            find_message("nppdvjthqldpwncqszvftbrmjlhg", 14).unwrap(),
            23
        );
        assert_eq!(
            find_message("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14).unwrap(),
            29
        );
        assert_eq!(
            find_message("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14).unwrap(),
            26
        );
    }

    #[test]
    fn test2_hist() {
        assert_eq!(
            find_message_hist("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14).unwrap(),
            19
        );
        assert_eq!(
            find_message_hist("bvwbjplbgvbhsrlpgdmjqwftvncz", 14).unwrap(),
            23
        );
        assert_eq!(
            find_message_hist("nppdvjthqldpwncqszvftbrmjlhg", 14).unwrap(),
            23
        );
        assert_eq!(
            find_message_hist("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14).unwrap(),
            29
        );
        assert_eq!(
            find_message_hist("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14).unwrap(),
            26
        );
    }
}
