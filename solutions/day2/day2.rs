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
    let mut shape_score: i32 = 0;
    let mut win_score: i32 = 0;

    // read line by line
    for line in reader.by_ref().lines() {
        let split: Vec<&str> = line.as_ref().unwrap().split(" ").collect();
        // calculate win score
        if (split[0] == "A" && split[1] == "Y")
            || (split[0] == "B" && split[1] == "Z")
            || (split[0] == "C" && split[1] == "X")
        {
            win_score += 6;
        } else if (split[0] == "A" && split[1] == "X")
            || (split[0] == "B" && split[1] == "Y")
            || (split[0] == "C" && split[1] == "Z")
        {
            win_score += 3;
        }
        // calcualte shape score
        if split[1] == "X" {
            shape_score += 1;
        } else if split[1] == "Y" {
            shape_score += 2;
        } else {
            shape_score += 3;
        }
        // println!("win_score: {win_score} shape_score: {shape_score}");
    }
    let total: i32 = shape_score + win_score;
    println!("part1 result: {total}");
    Ok(())
}

fn part2() -> std::io::Result<()> {
    let file = File::open("./input.txt").expect("failed to open file");
    let mut reader = BufReader::new(file);
    let mut shape_score: i32 = 0;
    let mut win_score: i32 = 0;

    // read line by line
    for line in reader.by_ref().lines() {
        let split: Vec<&str> = line.as_ref().unwrap().split(" ").collect();
        // need to lose, so win score is 0
        if split[1] == "X" {
            win_score += 0;
            if split[0] == "A" {
                shape_score += 3;
            } else if split[0] == "B" {
                shape_score += 1;
            } else if split[0] == "C" {
                shape_score += 2;
            }
        // need to draw, so win score is 3
        } else if split[1] == "Y" {
            win_score += 3;
            if split[0] == "A" {
                shape_score += 1;
            } else if split[0] == "B" {
                shape_score += 2;
            } else if split[0] == "C" {
                shape_score += 3;
            }
        // need to win, so win score is 6
        } else if split[1] == "Z" {
            win_score += 6;
            if split[0] == "A" {
                shape_score += 2;
            } else if split[0] == "B" {
                shape_score += 3;
            } else if split[0] == "C" {
                shape_score += 1;
            }
        }
        println!("win_score: {win_score} shape_score: {shape_score}");
    }
    let total: i32 = shape_score + win_score;
    println!("part2 result: {total}");
    Ok(())
}
