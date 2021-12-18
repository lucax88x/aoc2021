mod util;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    println!("Running AOC 2021");
    println!("Day 1");
    println!("Puzzle 1: {}", day1::puz1().to_string());
    println!("Puzzle 2: {}", day1::puz2().to_string());
    println!("Day 2");
    println!("Puzzle 1: {}", day2::puz1().to_string());
    println!("Puzzle 2: {}", day2::puz2().to_string());
    println!("Day 3");
    println!("Puzzle 1: {}", day3::puz1().to_string());
    println!("Puzzle 2: {}", day3::puz2().to_string());
    println!("Day 4");
    println!("Puzzle 1: {}", day4::puz1().to_string());
    println!("Puzzle 2: {}", day4::puz2().to_string());
    println!("Day 5");
    println!("Puzzle 1: {}", day5::puz1().to_string());
    println!("Puzzle 2: {}", day5::puz2().to_string());
}
