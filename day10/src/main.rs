use eyre::{eyre, Report};
use std::{convert::TryFrom, fs, str::FromStr};

fn main() -> eyre::Result<()> {
    let s = fs::read_to_string("input.txt")?;
    let matrix = BracketMatrix::from_str(s.as_str())?;
    let (part_one, part_two) = solve_1_and_2(&matrix);
    println!("Part One: {part_one}");
    println!("Part Two: {part_two}");

    Ok(())
}

fn solve_1_and_2(matrix: &BracketMatrix) -> (u64, u64) {
    let (part_1, part_2) = matrix
        .data
        .iter()
        .filter_map(BracketLine::check_validity)
        .fold((0, Vec::new()), |(p1, mut p2), r| match r {
            Ok(bs) => {
                p2.push(completion_points(&bs));
                return (p1, p2);
            }
            Err(b) => (p1 + syntax_points(&b), p2),
        });

    (part_1, median(part_2))
}

fn median(mut numbers: Vec<u64>) -> u64 {
    let idx = numbers.len() / 2;
    numbers.sort_unstable();
    numbers.get(idx).cloned().unwrap_or_default()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum BType {
    Round,
    Square,
    Curly,
    Angle,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Bracket {
    Opening(BType),
    Closing(BType),
}

fn syntax_points(bracket: &Bracket) -> u64 {
    match bracket {
        Bracket::Opening(_) => 0,
        Bracket::Closing(bt) => match bt {
            BType::Round => 3,
            BType::Square => 57,
            BType::Curly => 1197,
            BType::Angle => 25137,
        },
    }
}

fn completion_points(brackets: &Vec<Bracket>) -> u64 {
    let mut score = 0;
    for bracket in brackets {
        score *= 5;
        score += match bracket {
            Bracket::Opening(_) => 0,
            Bracket::Closing(bt) => match bt {
                BType::Round => 1,
                BType::Square => 2,
                BType::Curly => 3,
                BType::Angle => 4,
            },
        }
    }

    score
}

impl TryFrom<char> for Bracket {
    type Error = Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '(' => Self::Opening(BType::Round),
            '[' => Self::Opening(BType::Square),
            '{' => Self::Opening(BType::Curly),
            '<' => Self::Opening(BType::Angle),
            ')' => Self::Closing(BType::Round),
            ']' => Self::Closing(BType::Square),
            '}' => Self::Closing(BType::Curly),
            '>' => Self::Closing(BType::Angle),
            c => return Err(eyre!("Wrong character {}", c)),
        })
    }
}

#[derive(Debug, PartialEq)]
struct BracketLine {
    data: Vec<Bracket>,
}

impl BracketLine {
    fn check_validity(&self) -> Option<Result<Vec<Bracket>, Bracket>> {
        let mut stack = Vec::new();
        for bracket in self.data.iter() {
            match bracket {
                Bracket::Opening(bt) => stack.push(*bt),
                br @ Bracket::Closing(bt) => {
                    if let Some(most_recent) = stack.pop() {
                        if *bt != most_recent {
                            return Some(Err(*br));
                        }
                    } else {
                        return Some(Err(*br));
                    }
                }
            }
        }

        if stack.len() == 0 {
            None
        } else {
            Some(Ok(stack
                .into_iter()
                .rev()
                .map(|b| Bracket::Closing(b))
                .collect()))
        }
    }
}

impl FromStr for BracketLine {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .chars()
            .map(Bracket::try_from)
            .collect::<eyre::Result<_>>()?;

        Ok(Self { data })
    }
}

#[derive(Debug, PartialEq)]
struct BracketMatrix {
    data: Vec<BracketLine>,
}

impl FromStr for BracketMatrix {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(BracketLine::from_str)
            .collect::<eyre::Result<_>>()?;

        Ok(Self { data })
    }
}

#[cfg(test)]
mod tests;
