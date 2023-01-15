extern crate core;

use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::iter::zip;
use std::str::FromStr;
use std::{fmt, i32};

use itertools::{enumerate, Itertools};

const INPUT: &str = include_str!("../../input/d13.txt");
const DIVIDERS: &str = include_str!("../../input/d13_dividers.txt");

fn main() {
    let packets = parse(INPUT);
    let s = write_packets(&packets);
    verify_input(&s, &INPUT.lines().collect_vec());
    let ordering = compare_packets(&packets);
    let sum: usize = ordering
        .iter()
        .enumerate()
        .filter(|(_, o)| **o == Less)
        .map(|(i, _)| i + 1)
        .sum();
    println!("sum: {}", sum);

    let key = compute_decoder_key(&packets);
    println!("key: {}", key);
}

fn verify_input(a: &Vec<String>, b: &Vec<&str>) {
    assert_eq!(a.len(), b.len());
    for (l, r) in zip(a, b) {
        assert_eq!(l, r);
    }
}

fn write_packets(packets: &Vec<(Value, Value)>) -> Vec<String> {
    let mut output = Vec::new();
    for (i, (l, r)) in enumerate(packets) {
        if i > 0 {
            output.push(format!(""));
        }
        output.push(format!("{}", l));
        output.push(format!("{}", r));
    }
    output
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Value {
    List(Vec<Value>),
    Integer(i32),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::List(vs) => {
                write!(f, "[")?;
                for (i, v) in enumerate(vs) {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Value::Integer(i) => write!(f, "{}", i),
        }
    }
}

fn parse(s: &str) -> Vec<(Value, Value)> {
    let gs = s.lines().map(|l| l.trim()).group_by(|l| l.is_empty());
    let mut pairs = Vec::new();
    for (_, lines) in &gs {
        let ls = lines.collect_vec();
        if ls.len() == 1 {
            assert!(ls[0].is_empty());
            continue;
        }
        assert_eq!(ls.len(), 2);
        let (l1, _) = parse_value(&ls[0]);
        let (l2, _) = parse_value(&ls[1]);
        pairs.push((l1, l2));
    }
    pairs
}

fn parse_value(s: &str) -> (Value, usize) {
    if s.chars().next().unwrap() == '[' {
        return parse_list(s);
    }
    for (i, c) in s.chars().enumerate() {
        if c == ',' || c == ']' {
            return (Value::Integer(i32::from_str(&s[0..i]).unwrap()), i);
        }
    }
    panic!()
}

fn parse_list(mut s: &str) -> (Value, usize) {
    let mut values = Vec::new();
    let mut cs = s.chars();
    assert_eq!(cs.next().unwrap(), '[');
    let mut nn: usize = 1;
    s = &s[1..];
    cs = s.chars();
    loop {
        let mut c = cs.next().unwrap();
        if c == ']' {
            return (Value::List(values), nn + 1);
        }
        let (value, mut n) = if c == '[' {
            parse_list(s)
        } else {
            parse_value(s)
        };
        values.push(value);
        c = cs.nth(n - 1).unwrap();
        if c == ',' {
            n += 1
        }
        nn += n;
        s = &s[n..];
        cs = s.chars();
    }
}

fn compare_packets(packets: &Vec<(Value, Value)>) -> Vec<Ordering> {
    packets
        .iter()
        .map(|(a, b)| a.partial_cmp(b).unwrap())
        .collect_vec()
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_value(self, other)
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn compare_value(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        (Value::Integer(ai), Value::Integer(bi)) => {
            if ai < bi {
                Less
            } else if ai > bi {
                Greater
            } else {
                Equal
            }
        }
        (Value::Integer(ai), Value::List(_)) => {
            compare_value(&Value::List(vec![Value::Integer(*ai)]), b)
        }
        (Value::List(_), Value::Integer(bi)) => {
            compare_value(a, &Value::List(vec![Value::Integer(*bi)]))
        }
        (Value::List(al), Value::List(bl)) => {
            for i in 0.. {
                if i == al.len() && i == bl.len() {
                    return Equal;
                }
                if i == al.len() {
                    return Less;
                }
                if i == bl.len() {
                    return Greater;
                }
                let o = compare_value(&al[i], &bl[i]);
                if o == Equal {
                    continue;
                }
                return o;
            }
            panic!();
        }
    }
}

fn compute_decoder_key(packets: &Vec<(Value, Value)>) -> usize {
    let mut all_packets = packets.iter().flat_map(|(a, b)| vec![a, b]).collect_vec();
    let dividers = parse(DIVIDERS);
    let d1 = &dividers[0].0;
    let d2 = &dividers[0].1;
    all_packets.push(d1);
    all_packets.push(d2);
    all_packets.sort();
    all_packets
        .iter()
        .enumerate()
        .filter(|(_, p)| **p == d1 || **p == d2)
        .map(|(i, _)| i + 1)
        .reduce(|a, b| a * b)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE1: &str = include_str!("../../input/d13_sample1.txt");

    #[test]
    fn test1() {
        let packets = parse(SAMPLE1);
        let s = write_packets(&packets);
        verify_input(&s, &SAMPLE1.lines().collect_vec());
        let in_order = compare_packets(&packets);
        assert_eq!(
            in_order,
            vec![Less, Less, Greater, Less, Greater, Less, Greater, Greater]
        );
    }

    #[test]
    fn test2() {
        let packets = parse(SAMPLE1);
        let key = compute_decoder_key(&packets);
        assert_eq!(key, 140);
    }
}
