extern crate core;

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

    pub fn sum_size_le(&self, max_size: usize) -> usize {
        let this = self.size();
        let dirs_sum = self.dirs.iter().map(|(_, dir)| dir.borrow().sum_size_le(max_size)).sum();
        if this < max_size {
            dirs_sum + this
        } else {
            dirs_sum
        }
    }

    pub fn find_min_size_ge(&self, min_size: usize) -> Option<usize> {
        let this = Some(self.size()).filter(|s| s >= &min_size);
        let min_dir = self.dirs.iter()
            .map(|(_, dir)| dir.borrow().find_min_size_ge(min_size))
            .filter(|o| o.is_some())
            .map(|o| o.unwrap())
            .min();
        [this, min_dir].iter().flat_map(|n| n.iter()).min().map(|c| *c)
    }
}

fn parse(input: &str) -> DirRc {
    lazy_static! {
        static ref FILE_RE: Regex = Regex::new(r"^(?P<size>\d+) (?P<name>.*)$").unwrap();
    }
    let lines = input.trim().lines().map(|l| l.trim().to_string());

    let root = Dir::new_rc();
    let mut path = Vec::<DirRc>::new();
    let mut dir = root.clone();

    for line in lines {
        if line == "$ cd /" {
            dir = root.clone();
            path.clear();
        } else if line == "$ cd .." {
            dir = path.pop().unwrap().clone();
        } else if line.starts_with("$ cd ") {
            let dirname = &line[5..];
            let next_dir = match dir.as_ref().borrow_mut().dirs.entry(String::from(dirname)) {
                Entry::Occupied(o) => { o.get().clone() }
                Entry::Vacant(v) => { v.insert(Dir::new_rc()).clone() }
            };
            path.push(dir);
            dir = next_dir;
        } else if line.starts_with("$ ls") {
            continue;
        } else if line.starts_with("dir ") {
            let dirname = &line[4..];
            match (*dir.as_ref().borrow_mut()).dirs.entry(String::from(dirname)) {
                Entry::Occupied(o) => { o.get() }
                Entry::Vacant(v) => { v.insert(Dir::new_rc()) }
            };
        } else {
            let caps = FILE_RE.captures(&line);
            match caps {
                Some(c) => {
                    let size = usize::from_str(c.name("size").unwrap().as_str()).unwrap();
                    let name = c.name("name").unwrap().as_str();
                    match (*dir.as_ref().borrow_mut()).files.entry(String::from(name)) {
                        Entry::Occupied(o) => { o.get() }
                        Entry::Vacant(v) => { v.insert(size) }
                    };
                }
                None => { panic!(); }
            }
        }
    }
    root
}

fn main() {
    let root = parse(INPUT);
    println!("part1: {}", root.borrow().sum_size_le(100000));
    println!("part2: {}", find_dir_to_delete(root));
}

fn find_dir_to_delete(root: DirRc) -> usize {
    let total_disk_space: usize = 70000000;
    let disk_space_required: usize = 30000000;
    let disk_space_used = root.borrow().size();
    let disk_space_free = total_disk_space - disk_space_used;
    let additional_disk_space_required = disk_space_required - disk_space_free;
    root.borrow().find_min_size_ge(additional_disk_space_required).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../input/d07_sample.txt");

    #[test]
    fn test1() {
        let root = parse(SAMPLE);
        // println!("{:#?}", root);
        assert_eq!(root.borrow().sum_size_le(100000), 95437);
    }

    #[test]
    fn test2() {
        let root = parse(SAMPLE);
        find_dir_to_delete(root);
    }
}