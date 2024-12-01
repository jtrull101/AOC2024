use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn solve() {
    let input = include_str!("../../input/day01.txt");
    let mut heap1: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut heap2: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let num1: i32 = parts.next().unwrap().parse().unwrap();
        let num2: i32 = parts.next().unwrap().parse().unwrap();
        heap1.push(Reverse(num1));
        heap2.push(Reverse(num2));
    }

    let mut sum = 0;
    while let (Some(Reverse(val1)), Some(Reverse(val2))) = (heap1.pop(), heap2.pop()) {
        sum += (val2 - val1).abs();
    }

    println!("Day 01: {}", sum);
}