extern crate core;

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;

const INPUT: &str = include_str!("../../input/d07.txt");

#[derive(Debug)]
struct Dir {
    files: HashMap<String, usize>,
    dirs: HashMap<String, Rc<RefCell<Dir>>>,
}

type DirRc = Rc<RefCell<Dir>>;

impl Dir {
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            dirs: HashMap::new(),
        }
    }

    pub fn new_rc() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Dir::new()))
    }

    pub fn size(&self) -> usize {
        let files: usize = self.files.iter().map(|(_, size)| size).sum();
        let dirs: usize = self.dirs.iter().map(|(_, dir)| dir.borrow().size()).sum();
        files + dirs
    }

    pub fn find_size_le(&self, max_size: usize) -> usize {
        let this = self.size();
        let dirs_sum = self.dirs.iter().map(|(_, dir)| dir.borrow().find_size_le(max_size)).sum();
        if this < max_size {
            dirs_sum + this
        } else {
            dirs_sum
        }
    }
}

fn parse(input: &str) -> DirRc {
    lazy_static! {
        static ref FILE_RE: Regex = Regex::new(r"^(?P<size>\d+) (?P<name>.*)$").unwrap();
    }
    let lines = input.trim().lines().map(String::from).collect::<Vec<String>>();
    let mut i: usize = 0;
    let root = Dir::new_rc();
    let mut path = Vec::<DirRc>::new();
    let mut dir = root.clone();
    while i < lines.len() {
        let line = lines.get(i).unwrap().trim();
        i += 1;
        if line == "$ cd /" {
            dir = root.clone();
            path.clear();
        } else if line == "$ cd .." {
            dir = path.pop().unwrap().clone();
        } else if line.starts_with("$ cd ") {
            let dirname = &line[5..];
            let next_dir = match (*dir.borrow_mut().as_ref().borrow_mut()).dirs.entry(String::from(dirname)) {
                Entry::Occupied(o) => { o.get().clone() }
                Entry::Vacant(v) => { v.insert(Dir::new_rc()).clone() }
            };
            path.push(dir);
            dir = next_dir;
        } else if line.starts_with("$ ls") {} else if line.starts_with("dir ") {
            let dirname = &line[4..];
            match (*dir.borrow_mut().as_ref().borrow_mut()).dirs.entry(String::from(dirname)) {
                Entry::Occupied(o) => { o.get() }
                Entry::Vacant(v) => { v.insert(Dir::new_rc()) }
            };
        } else {
            let caps = FILE_RE.captures(line);
            match caps {
                Some(c) => {
                    let size = usize::from_str(c.name("size").unwrap().as_str()).unwrap();
                    let name = c.name("name").unwrap().as_str();
                    match (*dir.borrow_mut().as_ref().borrow_mut()).files.entry(String::from(name)) {
                        Entry::Occupied(o) => { o.get() }
                        Entry::Vacant(v) => { v.insert(size) }
                    };
                }
                None => { panic!(); }
            }
        }

        // println!("{}: {}: {:#?}", i, line, root);
        // println!("");
    }
    root
}

fn main() {
    let root = parse(INPUT);
    println!("{}", root.borrow().find_size_le(100000))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../input/d07_sample.txt");

    #[test]
    fn test() {
        let root = parse(SAMPLE);
        // println!("{:#?}", root);
        assert_eq!(root.borrow().find_size_le(100000), 95437);
    }
}