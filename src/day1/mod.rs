use crate::util;

pub fn puz1() -> i32 {
    let measurements = util::lines_from_file("src/day1/input");
    count_increments_with_lines(measurements)
}

pub fn puz2() -> i32 {
    let measurements = util::lines_from_file("src/day1/input");
    count_increments_with_sliding_windows(measurements)
}

fn count_increments_with_lines(measurements: Vec<String>) -> i32 {
    count_increments(util::lines_to_ints(measurements))
}

fn count_increments_with_sliding_windows(measurements: Vec<String>) -> i32 {
    let mut sliding_windows: Vec<i32> = Vec::new();
    let parsed_measurements = util::lines_to_ints(measurements);

    for index in 0..parsed_measurements.len() - 2 {
        sliding_windows.push(parsed_measurements.iter().skip(index).take(3).sum());
    }

    count_increments(sliding_windows)
}

fn count_increments(measurements: Vec<i32>) -> i32 {
    let mut count = 0;
    let mut previous_depth: Option<i32> = Option::None;
    for measurement in measurements {
        if let Some(previous_depth) = previous_depth {
            if measurement > previous_depth {
                count += 1;
            }
        }
        previous_depth = Some(measurement);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_input() -> Vec<String> {
        return [
            "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
        ]
        .iter()
        .map(|n| n.to_string())
        .collect();
    }

    #[test]
    fn puzzle_1_example() {
        assert_eq!(count_increments_with_lines(build_test_input()), 7);
    }

    #[test]
    fn puzzle_2_example() {
        assert_eq!(count_increments_with_sliding_windows(build_test_input()), 5);
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
