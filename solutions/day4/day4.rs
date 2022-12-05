use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

fn main() {
    part1().unwrap();
    part2().unwrap();
}

fn part1() -> std::io::Result<()> {
    let file = File::open("./input.txt").expect("failed to open file");
    let mut reader = BufReader::new(file);
    let mut total: u32 = 0;
    let mut s1: i32 = 0;
    let mut s2: i32 = 0;
    let mut e1: i32 = 0;
    let mut e2: i32 = 0;

    // read line by line
    for line in reader.by_ref().lines() {
        let split: Vec<&str> = line.as_ref().unwrap().split(",").collect();
        let (start1, end1) = split[0].split_at(split[0].find("-").unwrap());
        let (start2, end2) = split[1].split_at(split[1].find("-").unwrap());
        s1 = start1.parse::<i32>().unwrap();
        s2 = start2.parse::<i32>().unwrap();
        e1 = end1[1..].parse::<i32>().unwrap();
        e2 = end2[1..].parse::<i32>().unwrap();
        if (s1 <= s2 && e1 >= e2) || (s1 >= s2 && e1 <= e2) {
            total += 1;
        }
    }
    println!("part1 result: {total}");
    Ok(())
}

fn part2() -> std::io::Result<()> {
    let file = File::open("./input.txt").expect("failed to open file");
    let mut reader = BufReader::new(file);
    let mut total: u32 = 0;
    let mut s1: i32 = 0;
    let mut s2: i32 = 0;
    let mut e1: i32 = 0;
    let mut e2: i32 = 0;

    // read line by line
    for line in reader.by_ref().lines() {
        let split: Vec<&str> = line.as_ref().unwrap().split(",").collect();
        let (start1, end1) = split[0].split_at(split[0].find("-").unwrap());
        let (start2, end2) = split[1].split_at(split[1].find("-").unwrap());
        s1 = start1.parse::<i32>().unwrap();
        s2 = start2.parse::<i32>().unwrap();
        e1 = end1[1..].parse::<i32>().unwrap();
        e2 = end2[1..].parse::<i32>().unwrap();
        if (s1 <= s2 && e1 >= e2)
            || (s1 >= s2 && e1 <= e2)
            || (s1 <= s2 && e1 >= s2)
            || (s1 <= e2 && e1 >= e2)
        {
            total += 1;
        }
    }
    println!("part1 result: {total}");
    Ok(())
}
