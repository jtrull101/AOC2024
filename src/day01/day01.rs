use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Reverse;


pub fn solve_part1() {
    let (mut heap1, mut heap2) = setup_heaps();

    let mut sum = 0;
    while let (Some(Reverse(val1)), Some(Reverse(val2))) = (heap1.pop(), heap2.pop()) {
        sum += (val2 - val1).abs();
    }

    println!("Day 01: Part 1: {}", sum);
}


pub fn solve_part2() {
    let (mut heap1, heap2) = setup_heaps();
    let mut found_values: HashMap<i32, i32> = HashMap::new();

    while let Some(Reverse(val1)) = heap1.pop() {
        let mut similarity_score = 0;
        for &Reverse(val2) in heap2.iter() {
            if val1 == val2 {
                similarity_score += val1;
            }
        }
        found_values.insert(val1, similarity_score);
    }

    let sum: i32 = found_values.values().sum();
    println!("Day 01: Part 2: {}", sum);
}

/**
 * Read the input file and create two heaps.
 * The heaps are sorted in ascending order.
 */
fn setup_heaps() -> (BinaryHeap<Reverse<i32>>, BinaryHeap<Reverse<i32>>) {
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

    (heap1, heap2)
}
