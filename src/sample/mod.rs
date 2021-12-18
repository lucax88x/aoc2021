use crate::util;

pub fn puz1() -> i32 {
    let input = util::lines_from_file("src/day4/input");
    fn1(&input)
}

pub fn puz2() -> i32 {
    let input = util::lines_from_file("src/day4/input");
    fn2(&input)
}

fn fn1(input: &Vec<String>) -> i32 {
    0
}

fn fn2(input: &Vec<String>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_input() -> Vec<String> {
        return [
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            "8  2 23  4 24",
            "21  9 14 16  7",
            "6 10  3 18  5",
            "1 12 20 15 19",
            "",
            "3 15  0  2 22",
            "9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            "2  0 12  3  7",
        ]
        .iter()
        .map(|n| n.to_string())
        .collect();
    }

    #[test]
    fn puzzle_1_example() {
        assert_eq!(fn1(&build_test_input()), 1);
    }

    #[test]
    fn puzzle_2_example() {
        assert_eq!(fn2(&build_test_input()), 1);
    }

    #[test]
    fn puzzle_1_answer() {
        assert_eq!(puz1(), 1);
    }

    #[test]
    fn puzzle_2_answer() {
        assert_eq!(puz2(), 1);
    }
}
