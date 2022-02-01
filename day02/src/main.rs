use std::{fs, str::FromStr};

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let v = parse_input(content.as_str());
    println!("Part 1: {}", solve_1(&v));
    println!("Part 2: {}", solve_2(&v));
}

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(first) = s.chars().next() {
            Ok(match first {
                'f' => Direction::Forward,
                'u' => Direction::Up,
                'd' => Direction::Down,
                _ => return Err("unexpected input"),
            })
        } else {
            Err("unexpected input")
        }
    }
}

fn solve_1(input: &Vec<(Direction, i32)>) -> i32 {
    let (h, d) =
        input.iter().fold(
            (0, 0),
            |(horizontal, depth), (direction, amount)| match direction {
                Direction::Forward => (horizontal + amount, depth),
                Direction::Up => (horizontal, depth - amount),
                Direction::Down => (horizontal, depth + amount),
            },
        );
    h * d
}

fn solve_2(input: &Vec<(Direction, i32)>) -> i32 {
    // down X increases your aim by X units.
    // up X decreases your aim by X units.
    // forward X does two things:
    // It increases your horizontal position by X units.
    // It increases your depth by your aim multiplied by X.
    let (h, d, _) = input.iter().fold(
        (0, 0, 0),
        |(horizontal, depth, aim), (direction, amount)| match direction {
            Direction::Forward => (horizontal + amount, depth + aim * amount, aim),
            Direction::Up => (horizontal, depth, aim - amount),
            Direction::Down => (horizontal, depth, aim + amount),
        },
    );
    h * d
}

fn parse_input(s: &str) -> Vec<(Direction, i32)> {
    s.lines()
        .filter_map(|line| {
            let mut split = line.split_whitespace();
            split
                .next()
                .and_then(|d| Direction::from_str(d).ok())
                .zip(split.next().and_then(|n| n.parse::<i32>().ok()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{parse_input, solve_1, solve_2, Direction};

    #[test]
    fn parse_input_works() {
        let content = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        let actual = parse_input(content);
        let expected: Vec<(Direction, i32)> = vec![
            (Direction::Forward, 5),
            (Direction::Down, 5),
            (Direction::Forward, 8),
            (Direction::Up, 3),
            (Direction::Down, 8),
            (Direction::Forward, 2),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn small_input_solved_1() {
        let content = fs::read_to_string("small_input.txt").unwrap();
        let actual = solve_1(&parse_input(content.as_str()));
        let expected = 150;
        assert_eq!(actual, expected);
    }

    #[test]
    fn small_input_solved_2() {
        let content = fs::read_to_string("small_input.txt").unwrap();
        let actual = solve_2(&parse_input(content.as_str()));
        let expected = 900;
        assert_eq!(actual, expected);
    }
}
