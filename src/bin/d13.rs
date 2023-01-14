use std::{fmt, i32};
use std::fmt::Write;
use std::iter::zip;
use std::str::FromStr;

use itertools::{enumerate, Itertools};

use crate::Order::{CORRECT, EQUAL, INCORRECT};

const INPUT: &str = include_str!("../../input/d13.txt");

fn main() {
    let packets = parse(INPUT);
    let s = write_packets(&packets);
    verify_input(&s, &INPUT.lines().collect_vec());
    let in_order = compare(&packets);
    println!("{:#?}", in_order);
    let sum: usize = in_order.iter().enumerate()
        .filter(|(_, o)| **o == CORRECT)
        .map(|(i, _)| i + 1)
        .sum();
    println!("sum: {}", sum);
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

#[derive(Debug, PartialEq, Eq, Hash)]
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
    let gs = s.lines()
        .map(|l| l.trim())
        .group_by(|l| l.is_empty());
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

#[derive(Debug, PartialEq, Eq, Hash)]
enum Order {
    CORRECT,
    INCORRECT,
    EQUAL,
}

fn compare(packets: &Vec<(Value, Value)>) -> Vec<Order> {
    packets.iter().map(|(a, b)| compare_value(a, b)).collect_vec()
}

fn compare_value(a: &Value, b: &Value) -> Order {
    match (a, b) {
        (Value::Integer(ai), Value::Integer(bi)) => if ai < bi {
            CORRECT
        } else if ai > bi {
            INCORRECT
        } else {
            EQUAL
        },
        (Value::Integer(ai), Value::List(_)) =>
            compare_value(
                &Value::List(vec![Value::Integer(*ai)]), b),
        (Value::List(_), Value::Integer(bi)) =>
            compare_value(
                a, &Value::List(vec![Value::Integer(*bi)])),
        (Value::List(al), Value::List(bl)) => {
            for i in 0.. {
                if i == al.len() && i == bl.len() {
                    return EQUAL;
                }
                if i == al.len() {
                    return CORRECT;
                }
                if i == bl.len() {
                    return INCORRECT;
                }
                let o = compare_value(&al[i], &bl[i]);
                if o == EQUAL {
                    continue;
                }
                return o;
            }
            panic!();
        }
    }
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
        let in_order = compare(&packets);
        assert_eq!(in_order, vec![CORRECT, CORRECT, INCORRECT, CORRECT, INCORRECT, CORRECT, INCORRECT, INCORRECT]);
    }
}