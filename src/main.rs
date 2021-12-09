mod util;
mod day1;
mod day2;

fn main() {
    println!("Running AOC 2021");
    println!("Result of day 1 puzzle 1 is: {}", day1::puz1().to_string());
    println!("Result of day 1 puzzle 2 is: {}", day1::puz2().to_string());
    println!("Result of day 2 puzzle 1 is: {}", day2::puz1().to_string());
    println!("Result of day 2 puzzle 2 is: {}", day2::puz2().to_string());
}
