use crate::util;

pub fn puz1() -> i32 {
    let input = util::lines_from_file("src/day4/input");
    win_bingo(&input)
}

pub fn puz2() -> i32 {
    let input = util::lines_from_file("src/day4/input");
    let_last_bingo_win(&input)
}

fn win_bingo(input: &Vec<String>) -> i32 {
    let bingos = build_bingos(input);
    let extracted_numbers = extract_numbers(input);

    for extracted_number_index in 0..extracted_numbers.len() {
        for bingo in &bingos {
            let result = check_if_bingo_won(&bingo, &extracted_numbers, extracted_number_index);
            if result > 0 {
                return result;
            }
        }
    }

    0
}

fn let_last_bingo_win(input: &Vec<String>) -> i32 {
    let bingos = build_bingos(input);
    let extracted_numbers = extract_numbers(input);
    let mut winning_bingos: Vec<usize> = vec![bingos.len() - 1];

    for extracted_number_index in 0..extracted_numbers.len() {
        for bingo_index in 0..bingos.len() {
            if winning_bingos.contains(&bingo_index) {
                continue;
            }

            let bingo = &bingos[bingo_index];
            let result = check_if_bingo_won(&bingo, &extracted_numbers, extracted_number_index);
            if result > 0 {
                winning_bingos.push(bingo_index);

                if winning_bingos.len() == bingos.len() {
                    return result;
                }
            }
        }
    }

    0
}

fn check_if_bingo_won_on_direction(
    horizontal_check: bool,
    bingo: &Vec<Vec<i32>>,
    extracted_numbers: &Vec<i32>,
    extracted_number_index: usize,
) -> i32 {
    let to_check_numbers = extracted_numbers
        .iter()
        .take(extracted_number_index + 1)
        .map(|c| *c)
        .collect::<Vec<i32>>();

    for left_index in 0..5 {
        let mut present = true;
        for right_index in 0..5 {
            if horizontal_check {
                if !to_check_numbers.contains(&bingo[left_index][right_index]) {
                    present = false;
                    break;
                }
            } else {
                if !to_check_numbers.contains(&bingo[right_index][left_index]) {
                    present = false;
                    break;
                }
            }
        }

        if present {
            let mut sum_of_unmarked_numbers = 0;
            for row in 0..5 {
                for col in 0..5 {
                    if !to_check_numbers.contains(&bingo[row][col]) {
                        sum_of_unmarked_numbers += bingo[row][col];
                    }
                }
            }

            let winning_number = extracted_numbers[extracted_number_index];
            return winning_number * sum_of_unmarked_numbers;
        }
    }

    0
}

fn check_if_bingo_won(
    bingo: &Vec<Vec<i32>>,
    extracted_numbers: &Vec<i32>,
    extracted_number_index: usize,
) -> i32 {
    let horizontal_check =
        check_if_bingo_won_on_direction(true, bingo, &extracted_numbers, extracted_number_index);

    if horizontal_check > 0 {
        return horizontal_check;
    } else {
        let vertical_check = check_if_bingo_won_on_direction(
            false,
            bingo,
            &extracted_numbers,
            extracted_number_index,
        );

        if vertical_check > 0 {
            return vertical_check;
        }
    }
    0
}

fn build_bingos(input: &Vec<String>) -> Vec<Vec<Vec<i32>>> {
    let mut bingos: Vec<Vec<Vec<i32>>> = Vec::new();

    for grid_index in (2..input.len()).step_by(6) {
        let mut grid: Vec<Vec<i32>> = vec![vec![0; 5]; 5];
        for grid_row_index in 0..5 {
            let grid_row = &input[grid_index + grid_row_index];

            let grid_numbers = grid_row
                .split_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            for grid_col_index in 0..5 {
                grid[grid_row_index][grid_col_index] = grid_numbers[grid_col_index];
            }
        }

        bingos.push(grid);
    }

    bingos
}

fn extract_numbers(input: &Vec<String>) -> Vec<i32> {
    return input
        .iter()
        .nth(0)
        .unwrap()
        .split(',')
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
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
        assert_eq!(win_bingo(&build_test_input()), 4512);
    }

    #[test]
    fn puzzle_2_example() {
        assert_eq!(let_last_bingo_win(&build_test_input()), 1924);
    }

    #[test]
    fn puzzle_1_answer() {
        assert_eq!(puz1(), 51776);
    }

    #[test]
    fn puzzle_2_answer() {
        assert_eq!(puz2(), 16830);
    }
}
