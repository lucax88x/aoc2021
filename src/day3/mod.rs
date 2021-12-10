use crate::util;

pub fn puz1() -> i32 {
    let input = util::lines_from_file("src/day3/input");
    calculate_power_consumption(&input)
}

pub fn puz2() -> i32 {
    let input = util::lines_from_file("src/day3/input");
    todo(&input)
}

fn calculate_power_consumption(list_of_bits: &Vec<String>) -> i32 {
    let mut gamma: String = String::new();
    let mut epsilon: String = String::new();
 
    let bits_length = list_of_bits[0].len();

    for index in 0..bits_length {
        let mut zeroes = 0;
        let mut ones = 0;
        for bits in list_of_bits {
            if bits.chars().nth(index).unwrap() == '0' {
                zeroes += 1;
            } else {
                ones += 1;
            }
        }

        if zeroes >= ones {
            gamma += "0";
            epsilon += "1";
        } else {
            gamma += "1";
            epsilon += "0";
        }
    }

    bits_to_decimal(&gamma) * bits_to_decimal(&epsilon)
}

fn todo(coordinates: &Vec<String>) -> i32 {
    0
}

fn bits_to_decimal(bits: &str) -> i32 {
    let mut decimal = 0;
    
    for (index, bit) in bits.chars().enumerate() {
        let power = bits.len() - index - 1;
        if bit == '1' {
            decimal += 2_i32.pow(power as u32);
        }
    }
    decimal
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_input() -> Vec<String> {
        return [
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|n| n.to_string())
        .collect();
    }

    #[test]
    fn puzzle_1_example() {
        assert_eq!(calculate_power_consumption(&build_test_input()), 198);
    }

    #[test]
    fn puzzle_2_example() {
        assert_eq!(todo(&build_test_input()), 900);
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
