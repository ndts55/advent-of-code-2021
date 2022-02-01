use std::{collections::VecDeque, fs, str::FromStr};

fn main() {
    let mut input = Input::from_str(fs::read_to_string("input.txt").unwrap().as_str()).unwrap();
    let solution_1 = solve_1(&mut input);
    println!("Part 1: {solution_1}");
    let solution_2 = solve_2(&mut input);
    println!("Part 2: {solution_2}");
}

fn solve_1(input: &mut Input) -> u32 {
    for _ in 0..4 {
        if let Some(n) = input.numbers.pop_front() {
            for board in &mut input.boards {
                board.call_number(n);
            }
            // We don't check for bingo here, because we need at least 5 calls for a bingo.
        }
    }

    let mut number = 0;

    let winning_board_sum: u32 = loop {
        if let Some(n) = input.numbers.pop_front() {
            number = n;
            for board in &mut input.boards {
                board.call_number(number);
            }

            if let Some(winner) = input.boards.iter().find(|&board| board.has_bingo()) {
                break winner.remaining_sum();
            }
        } else {
            break 0; // No more numbers to call.
        }
    };

    winning_board_sum * number
}

fn solve_2(input: &mut Input) -> u32 {
    input.boards.retain(|board| !board.has_bingo()); // First, remove boards that already won.

    let mut number = 0;

    let last_winning_board_sum = loop {
        if let Some(n) = input.numbers.pop_front() {
            number = n;

            for board in &mut input.boards {
                board.call_number(number);
            }

            match input.boards.len() {
                0 => break 0, // We had multiple boards that won simultaneously.
                1 => {
                    if input.boards[0].has_bingo() {
                        break input.boards[0].remaining_sum(); // We have a last winner.
                    }
                }
                _ => input.boards.retain(|board| !board.has_bingo()), // Remove boards that won.
            }
        } else {
            break 0;
        }
    };

    last_winning_board_sum * number
}

#[derive(Debug, PartialEq)]
struct Board {
    rows: [Vec<u32>; 5],
    cols: [Vec<u32>; 5],
}

impl Board {
    fn call_number(&mut self, number: u32) {
        for row in &mut self.rows {
            row.retain(|n| *n != number);
        }
        for col in &mut self.cols {
            col.retain(|n| *n != number);
        }
    }

    fn has_bingo(&self) -> bool {
        self.rows.iter().any(Vec::is_empty) || self.cols.iter().any(Vec::is_empty)
    }

    fn remaining_sum(&self) -> u32 {
        self.rows.iter().flatten().sum()
    }
}

impl FromStr for Board {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows: [Vec<u32>; 5] = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        let mut cols: [Vec<u32>; 5] = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        let m = s.lines().map(|line| line.split_whitespace()).enumerate();
        for (row_count, row) in m {
            for (column_count, number_string) in row.enumerate() {
                let number = number_string
                    .parse::<u32>()
                    .map_err(|_| "Error parsing number string to u32")?;
                rows[row_count].push(number);
                cols[column_count].push(number);
            }
        }

        Ok(Board { rows, cols })
    }
}

#[derive(Debug, PartialEq)]
struct Input {
    numbers: VecDeque<u32>,
    boards: Vec<Board>,
}

impl FromStr for Input {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let numbers = lines
            .next()
            .ok_or("Found no line for number input.")?
            .split(',')
            .map(|ns| ns.parse::<u32>())
            .collect::<Result<VecDeque<u32>, _>>()
            .map_err(|_| "Error parsing number strings to u32.")?;

        lines.next(); // Discard empty line
        let start_of_boards = s.find("\n\n").ok_or("Could not find start of boards.")?;

        let boards: Vec<Board> = (&s[start_of_boards..])
            .trim()
            .split("\n\n")
            .map(|board_str| Board::from_str(board_str))
            .collect::<Result<Vec<Board>, Self::Err>>()?;

        Ok(Self { numbers, boards })
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::VecDeque, fs, str::FromStr, vec};

    use super::{solve_1, solve_2, Board, Input};

    #[test]
    fn solve_2_works() {
        let mut input =
            Input::from_str(fs::read_to_string("small_input.txt").unwrap().as_str()).unwrap();
        let expected = 1924;
        let actual = solve_2(&mut input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn solve_1_works() {
        let mut input =
            Input::from_str(fs::read_to_string("small_input.txt").unwrap().as_ref()).unwrap();
        let expected = 4512;
        let actual = solve_1(&mut input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn remaining_sum_works() {
        let board = Board {
            rows: [
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18, 5],
                vec![1, 12, 20, 15, 19],
            ],
            cols: [
                vec![22, 8, 21, 6, 1],
                vec![13, 2, 9, 10, 12],
                vec![17, 23, 14, 3, 20],
                vec![11, 4, 16, 18, 15],
                vec![0, 24, 7, 5, 19],
            ],
        };

        let expected = 300;
        let actual = board.remaining_sum();
        assert_eq!(actual, expected);
    }

    #[test]
    fn has_bingo_false_when_no_empty_vecs() {
        let board = Board {
            rows: [vec![1], vec![1], vec![1], vec![1], vec![1]],
            cols: [vec![2], vec![2], vec![2], vec![2], vec![2]],
        };

        assert!(!board.has_bingo());
    }

    #[test]
    fn has_bingo_true_when_at_least_one_empty_vec() {
        let board = Board {
            rows: [vec![1], vec![1], vec![1], vec![1], vec![1]],
            cols: [vec![], vec![2], vec![2], vec![2], vec![2]],
        };

        assert!(board.has_bingo());
    }

    #[test]
    fn call_number_removes_number_from_all_vecs() {
        let mut board = Board {
            rows: [vec![1], vec![1], vec![1], vec![1], vec![1]],
            cols: [vec![2], vec![2], vec![2], vec![2], vec![2]],
        };

        board.call_number(1);
        assert!(board.rows.iter().all(Vec::is_empty));
        assert!(!board.cols.iter().all(Vec::is_empty));

        board.call_number(2);
        assert!(board.cols.iter().all(Vec::is_empty));
    }

    #[test]
    fn call_number_no_change_when_number_is_not_on_board() {
        let mut board = Board {
            rows: [vec![1], vec![1], vec![1], vec![1], vec![1]],
            cols: [vec![2], vec![2], vec![2], vec![2], vec![2]],
        };

        board.call_number(3);

        assert!(!board.rows.iter().all(Vec::is_empty));
        assert!(!board.cols.iter().all(Vec::is_empty));
    }

    #[test]
    fn from_str_board_works() {
        let input =
            "22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19";
        let expected = Board {
            rows: [
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18, 5],
                vec![1, 12, 20, 15, 19],
            ],
            cols: [
                vec![22, 8, 21, 6, 1],
                vec![13, 2, 9, 10, 12],
                vec![17, 23, 14, 3, 20],
                vec![11, 4, 16, 18, 15],
                vec![0, 24, 7, 5, 19],
            ],
        };
        let actual = Board::from_str(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn from_str_input_works() {
        let input = fs::read_to_string("small_input.txt").unwrap();
        let expected = Input {
            numbers: VecDeque::from([
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ]),
            boards: vec![
                Board {
                    rows: [
                        vec![22, 13, 17, 11, 0],
                        vec![8, 2, 23, 4, 24],
                        vec![21, 9, 14, 16, 7],
                        vec![6, 10, 3, 18, 5],
                        vec![1, 12, 20, 15, 19],
                    ],
                    cols: [
                        vec![22, 8, 21, 6, 1],
                        vec![13, 2, 9, 10, 12],
                        vec![17, 23, 14, 3, 20],
                        vec![11, 4, 16, 18, 15],
                        vec![0, 24, 7, 5, 19],
                    ],
                },
                Board {
                    rows: [
                        vec![3, 15, 0, 2, 22],
                        vec![9, 18, 13, 17, 5],
                        vec![19, 8, 7, 25, 23],
                        vec![20, 11, 10, 24, 4],
                        vec![14, 21, 16, 12, 6],
                    ],
                    cols: [
                        vec![3, 9, 19, 20, 14],
                        vec![15, 18, 8, 11, 21],
                        vec![0, 13, 7, 10, 16],
                        vec![2, 17, 25, 24, 12],
                        vec![22, 5, 23, 4, 6],
                    ],
                },
                Board {
                    rows: [
                        vec![14, 21, 17, 24, 4],
                        vec![10, 16, 15, 9, 19],
                        vec![18, 8, 23, 26, 20],
                        vec![22, 11, 13, 6, 5],
                        vec![2, 0, 12, 3, 7],
                    ],
                    cols: [
                        vec![14, 10, 18, 22, 2],
                        vec![21, 16, 8, 11, 0],
                        vec![17, 15, 23, 13, 12],
                        vec![24, 9, 26, 6, 3],
                        vec![4, 19, 20, 5, 7],
                    ],
                },
            ],
        };

        let actual = Input::from_str(&input).unwrap();
        assert_eq!(actual, expected);
    }
}
