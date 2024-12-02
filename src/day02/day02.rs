
// each line indicates a report of individual numbers
//  need to identify if numbers are either:
//  1. all increasing or all decreasing
//  2. two adjacent numbers differ by at least one and at most three
// collate the number of all SAFE reports and return the count

pub fn solve() {
    let input = include_str!("../../input/day02.txt");
    let mut count = 0;
    for line in input.lines() {
        let mut prev = 0;
        let mut safe = true;
        let mut adj = false;
        for c in line.chars() {
            let n = c.to_digit(10).unwrap();
            if n < prev {
                safe = false;
                break;
            }
            if n == prev {
                adj = true;
            }
            prev = n;
        }
        if safe && adj {
            count += 1;
        }
    }
    println!("Day 02: Part 1: {}", count);
}