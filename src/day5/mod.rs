use crate::util;

pub fn puz1() -> i32 {
    let input = util::lines_from_file("src/day5/input");
    fn1(&input)
}

pub fn puz2() -> i32 {
    let input = util::lines_from_file("src/day5/input");
    fn2(&input)
}

#[derive(Debug)]
struct CoordinateLine {
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize,
}

struct Map {
    points: Vec<Vec<usize>>,
}

impl Map {
    fn new(width: usize, height: usize) -> Map {
        let points = vec![vec![0; width]; height];
        Map { points }
    }

    fn draw_line(&mut self, coordinate_line: CoordinateLine) {
        let (left_x, right_x) = if coordinate_line.x1 > coordinate_line.x2 {
            (coordinate_line.x2, coordinate_line.x1)
        } else {
            (coordinate_line.x1, coordinate_line.x2)
        };

        let (left_y, right_y) = if coordinate_line.y1 > coordinate_line.y2 {
            (coordinate_line.y2, coordinate_line.y1)
        } else {
            (coordinate_line.y1, coordinate_line.y2)
        };

        if left_x == right_x {
            for y in left_y..right_y + 1 {
                self.draw_point(left_x, y);
            }
        } else if left_y == right_y {
            for x in left_x..right_x + 1 {
                self.draw_point(x, left_y);
            }
        }
    }

    fn draw_point(&mut self, x: usize, y: usize) {
        self.points[y][x] += 1;
    }

    fn count_safe_points(&self) -> i32 {
        let mut counter = 0;
        for row in &self.points {
            for point in row {
                if point.ge(&2) {
                    counter += 1;
                }
            }
        }
        counter
    }
}

fn fn1(input: &Vec<String>) -> i32 {
    let mut coordinate_lines: Vec<CoordinateLine> = Vec::new();

    for line in input {
        let mut split = line.split_whitespace();

        let left_parts = split
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let right_parts = split
            .nth(1)
            .unwrap()
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let x1 = left_parts[0];
        let x2 = right_parts[0];
        let y1 = left_parts[1];
        let y2 = right_parts[1];

        if x1 == x2 || y1 == y2 {
            coordinate_lines.push(CoordinateLine { x1, x2, y1, y2 });
        }
    }

    println!("{:?}", coordinate_lines.len());

    let size = coordinate_lines
        .iter()
        .flat_map(|cl| vec![cl.x1, cl.x2, cl.y1, cl.y2])
        .max()
        .unwrap();

    let mut map = Map::new(size + 1, size + 1);

    for coordinate_line in coordinate_lines {
        map.draw_line(coordinate_line)
    }

    map.count_safe_points()
}

fn fn2(input: &Vec<String>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_input() -> Vec<String> {
        return [
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ]
        .iter()
        .map(|n| n.to_string())
        .collect();
    }

    #[test]
    fn puzzle_1_example() {
        assert_eq!(fn1(&build_test_input()), 5);
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
