#[cfg(feature = "day02")]
use advent_of_code::day02;

use advent_of_code::day01;

fn main() {
    #[cfg(feature = "day01")]
    day01::run();

    #[cfg(feature = "day02")]
    day02::run();

    #[cfg(not(any(feature = "day01", feature = "day02")))]
    {
        day01::run();
        day02::run();
    }
}