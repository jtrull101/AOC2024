use advent_of_code::day01;

fn main() {
    #[cfg(feature = "day01")]
    day01::run();

    #[cfg(not(feature = "day01"))]
    println!("No specific day feature enabled. Running default day 01.");
}