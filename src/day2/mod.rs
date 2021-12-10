use crate::util;

pub fn puz1() -> i32 {
    let input = util::lines_from_file("src/day2/input");
    navigate_ship(&input)
}

pub fn puz2() -> i32 {
    let input = util::lines_from_file("src/day2/input");
    navigate_ship_after_reading_manual(&input)
}

fn navigate_ship(coordinates: &Vec<String>) -> i32 {
    let forward_coordinates = filter_coordinates_by_prefix(&coordinates, "forward");
    let up_coordinates = filter_coordinates_by_prefix(&coordinates, "up");
    let down_coordinates = filter_coordinates_by_prefix(&coordinates, "down");

    let horizontal = extract_units_and_sum(&forward_coordinates);
    let up = extract_units_and_sum(&up_coordinates);
    let down = extract_units_and_sum(&down_coordinates);

    let position = horizontal;
    let depth = down - up;

    position * depth
}

fn navigate_ship_after_reading_manual(coordinates: &Vec<String>) -> i32 {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for coordinate in coordinates {
        let unit = coordinate
            .split_whitespace()
            .nth(1)
            .unwrap_or("0")
            .parse::<i32>()
            .unwrap_or(0);

        if coordinate.starts_with("up") {
            aim -= unit;
        } else if coordinate.starts_with("down") {
            aim += unit;
        } else if coordinate.starts_with("forward") {
            position += unit;
            depth += aim * unit;
        } else {
            panic!("Unknown command: {}", coordinate);
        }
    }

    position * depth
}

// TODO: ask explanations about lifetimes
fn filter_coordinates_by_prefix<'a>(coordinates: &'a Vec<String>, prefix: &str) -> Vec<&'a String> {
    coordinates
        .iter()
        .filter(|c| c.starts_with(prefix))
        .collect::<Vec<&String>>()
}

fn extract_units_and_sum(coordinates: &Vec<&String>) -> i32 {
    coordinates
        .iter()
        .map(|c| {
            c.split_whitespace()
                .nth(1)
                .unwrap_or("0")
                .parse::<i32>()
                .unwrap_or(0)
        })
        .sum()
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
        assert_eq!(navigate_ship(&build_test_input()), 150);
    }

    #[test]
    fn puzzle_2_example() {
        assert_eq!(navigate_ship_after_reading_manual(&build_test_input()), 900);
    }

    #[test]
    fn puzzle_1_answer() {
        assert_eq!(puz1(), 1694130);
    }

    #[test]
    fn puzzle_2_answer() {
        assert_eq!(puz2(), 1698850445);
    }
}
