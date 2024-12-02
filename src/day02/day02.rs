fn parse_input() -> Vec<Vec<u32>> {
    let input = include_str!("../../input/day02.txt");
    input.lines()
        .map(|line| line.split_whitespace().map(|num_str| num_str.parse().unwrap()).collect())
        .collect()
}

fn is_valid_report(numbers: &[u32]) -> bool {
    let mut prev = None;
    let mut direction = None; // None: not determined, Some(true): increasing, Some(false): decreasing

    for &n in numbers {
        if let Some(prev_num) = prev {
            let diff = (n as i32 - prev_num as i32).abs();
            if diff < 1 || diff > 3 {
                return false;
            }
            // first figure out the intended direction of the line, then enforce that direction
            match direction {
                None => {
                    if n > prev_num {
                        direction = Some(true);
                    } else if n < prev_num {
                        direction = Some(false);
                    }
                }
                Some(true) => {
                    if n <= prev_num {
                        return false;
                    }
                }
                Some(false) => {
                    if n >= prev_num {
                        return false;
                    }
                }
            }
        }
        prev = Some(n);
    }
    true
}

pub fn solve_part1() {
    let reports = parse_input();
    let count = reports.iter().filter(|&report| is_valid_report(report)).count();
    println!("Day 02: Part 1: {}", count);
}

pub fn solve_part2() {
    let reports = parse_input();
    let mut count = 0;

    for report in reports {
        let mut valid = false;

        for i in 0..report.len() {
            let mut modified_report = report.clone();
            modified_report.remove(i);

            if is_valid_report(&modified_report) {
                valid = true;
                break;
            }
        }

        if valid {
            count += 1;
        }
    }

    println!("Day 02: Part 2: {}", count);
}