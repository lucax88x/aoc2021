use crate::util::lines_from_file;

pub fn puz1() -> i32 {
    let coordinates = lines_from_file("src/day2/input");
    navigate_ship(coordinates)
}

pub fn puz2() -> i32 {
    let coordinates = lines_from_file("src/day2/input");
    todo(coordinates)
}

fn navigate_ship(coordinates: Vec<String>) -> i32 {
    let forward_coordinates = coordinates.iter().filter(|c| c.starts_with("forward"));
    let up_coordinates = coordinates.iter().filter(|c| c.starts_with("up"));
    let down_coordinates = coordinates.iter().filter(|c| c.starts_with("down"));

    let horizontal: i32 = forward_coordinates
        .map(|c| {
            c.split_whitespace()
                .nth(1)
                .unwrap_or("0")
                .parse::<i32>()
                .unwrap_or(0)
        })
        .sum();

    let up: i32 = up_coordinates
        .map(|c| {
            c.split_whitespace()
                .nth(1)
                .unwrap_or("0")
                .parse::<i32>()
                .unwrap_or(0)
        })
        .sum();

    let down: i32 = down_coordinates
        .map(|c| {
            c.split_whitespace()
                .nth(1)
                .unwrap_or("0")
                .parse::<i32>()
                .unwrap_or(0)
        })
        .sum();

    horizontal * (down - up)
}

fn todo(coordinates: Vec<String>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_input() -> Vec<String> {
        return [
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]
        .iter()
        .map(|n| n.to_string())
        .collect();
    }

    #[test]
    fn puzzle_1_example() {
        assert_eq!(navigate_ship(build_test_input()), 150);
    }

    #[test]
    fn puzzle_2_example() {
        assert_eq!(todo(build_test_input()), 5);
    }

    #[test]
    fn puzzle_1_answer() {
        assert_eq!(puz1(), 1557);
    }

    #[test]
    fn puzzle_2_answer() {
        assert_eq!(puz2(), 1608);
    }
}
