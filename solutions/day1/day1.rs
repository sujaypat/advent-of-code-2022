use std::cmp;
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
    let mut curr_total: i32 = 0;
    let mut max_total: i32 = 0;

    // read line by line
    for line in reader.by_ref().lines() {
        // update the max if the line is empty
        if line.as_ref().unwrap().is_empty() {
            max_total = cmp::max(curr_total, max_total);
            curr_total = 0
        // accumulate this elf's total
        } else {
            curr_total += line?.parse::<i32>().unwrap();
        }
    }
    println!("part1 result: {max_total}");
    Ok(())
}

fn part2() -> std::io::Result<()> {
    let file = File::open("./input.txt").expect("failed to open file");
    let mut reader = BufReader::new(file);
    let mut curr_total: i32 = 0;
    let mut v: Vec<i32> = Vec::new();

    // read line by line
    for line in reader.by_ref().lines() {
        // push the latest total to the vector
        if line.as_ref().unwrap().is_empty() {
            v.push(curr_total);
            curr_total = 0
        // accumulate this elf's total
        } else {
            curr_total += line?.parse::<i32>().unwrap();
        }
    }
    // sort, reverse, slice, and sum the last 3 values
    v.sort();
    let sum3: i32 = v.iter().rev().take(3).sum();
    println!("part2 result: {sum3}");
    Ok(())
}
