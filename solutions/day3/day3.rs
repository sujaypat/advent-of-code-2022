use std::collections::hash_set::Intersection;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::iter::FromIterator;

fn main() {
    part1().unwrap();
    part2().unwrap();
}

fn part1() -> std::io::Result<()> {
    let file = File::open("./input.txt").expect("failed to open file");
    let mut reader = BufReader::new(file);
    let mut total: u32 = 0;

    // read line by line
    for line in reader.by_ref().lines() {
        // split the string in half
        let len = line.as_ref().unwrap().chars().count();
        let split = line.as_ref().unwrap().split_at(len / 2);
        // create 2 sets
        let set0: HashSet<char> = HashSet::from_iter(split.0.chars());
        let set1: HashSet<char> = HashSet::from_iter(split.1.chars());
        // parse the intersection and add the value to the total
        let common: Intersection<char, _> = set0.intersection(&set1);
        for c in common {
            if c.is_uppercase() {
                total += *c as u32 - 'A' as u32 + 27;
            } else {
                total += *c as u32 - 'a' as u32 + 1;
            }
            // println!("common char: {} total: {}", c, total);
        }
    }
    println!("part1 result: {total}");
    Ok(())
}

fn part2() -> std::io::Result<()> {
    let file = File::open("./input.txt").expect("failed to open file");
    let mut reader = BufReader::new(file);
    let mut total: u32 = 0;
    let mut skip_lines: u32 = 3;
    let mut v: Vec<HashSet<char>> = Vec::new();

    // read line by line
    for line in reader.by_ref().lines() {
        if skip_lines > 0 {
            // push 3 lines into the vector
            println!("pushed {}", line.as_ref().unwrap());
            v.push(HashSet::from_iter(line.as_ref().unwrap().chars()));
            skip_lines -= 1;
        }
        if skip_lines <= 0 {
            // create 3 sets
            let int1: HashSet<_> = v
                .pop()
                .unwrap()
                .intersection(&v.pop().unwrap())
                .copied()
                .collect();
            // parse the intersection and add the value to the total
            let set2: HashSet<_> = v.pop().unwrap();
            let common: Intersection<char, _> = int1.intersection(&set2);
            for c in common {
                if c.is_uppercase() {
                    total += *c as u32 - 'A' as u32 + 27;
                } else {
                    total += *c as u32 - 'a' as u32 + 1;
                }
                println!("common char: {} total: {}", c, total);
            }
            skip_lines = 3
        }
    }
    println!("part1 result: {total}");
    Ok(())
}
