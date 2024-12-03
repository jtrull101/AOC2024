use regex::Regex;

fn parse_input(input: &str) -> &str {
    input.trim()
}

fn calculate_sum(input: &str, re: &Regex, is_enabled: bool) -> i32 {
    let mut sum = 0;
    if is_enabled {
        for cap in re.captures_iter(input) {
            let num1: i32 = cap[1].parse().unwrap();
            let num2: i32 = cap[2].parse().unwrap();
            sum += num1 * num2;
        }
    }
    sum
}

pub fn solve_part1() {
    let input = include_str!("../../input/day03.txt");
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let line = parse_input(input);
    let sum = calculate_sum(line, &re, true);
    println!("Day 03: Part 1: {}", sum);
}

pub fn solve_part2() {
    let input = include_str!("../../input/day03.txt");
    let re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();

    let line = parse_input(input);
    let mut sum = 0;
    let mut is_enabled = true;

    for cap in re.captures_iter(line) {
        match cap.get(0).unwrap().as_str() {
            "do()" => is_enabled = true,
            "don't()" => is_enabled = false,
            _ => {
                if is_enabled {
                    let num1: i32 = cap[2].parse().unwrap();
                    let num2: i32 = cap[3].parse().unwrap();
                    sum += num1 * num2;
                }
            }
        }
    }

    println!("Day 03: Part 2: {}", sum);
}