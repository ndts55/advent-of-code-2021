use std::{convert::TryFrom, fs, str::FromStr};

use crate::{
    solve_1_and_2,
    BType::*,
    Bracket::{self, *},
    BracketLine, BracketMatrix,
};

#[test]
fn part_one_and_two() {
    let expected = (399153, 2995077699);
    let matrix = BracketMatrix::from_str(
        fs::read_to_string("input.txt")
            .expect("where is input.txt?")
            .as_str(),
    )
    .expect("can't parse big input.");
    let actual = solve_1_and_2(&matrix);
    assert_eq!(actual, expected);
}

#[test]
fn solve_1_and_2_works_small_input() {
    let expected = (26397, 288957);
    let matrix = BracketMatrix::from_str(
        fs::read_to_string("small_input.txt")
            .expect("where is small input?.")
            .as_str(),
    )
    .expect("can't parse small input.");
    let actual = solve_1_and_2(&matrix);
    assert_eq!(actual, expected);
}

#[test]
fn check_validity_works() {
    let test_data: Vec<(&str, Option<Result<Vec<Bracket>, Bracket>>, &str)> = vec![
        (
            "{([(<{}[<>[]}>{[]{[(<()>",
            Some(Err(Closing(Curly))),
            "first",
        ),
        (
            "[[<[([]))<([[{}[[()]]]",
            Some(Err(Closing(Round))),
            "second",
        ),
        (
            "[{[{({}]{}}([{[{{{}}([]",
            Some(Err(Closing(Square))),
            "third",
        ),
        (
            "[<(<(<(<{}))><([]([]()",
            Some(Err(Closing(Round))),
            "fourth",
        ),
        ("<{([([[(<>()){}]>(<<{{", Some(Err(Closing(Angle))), "fifth"),
        (
            "[({(<(())[]>[[{[]{<()<>>",
            Some(Ok(vec![
                Closing(Curly),
                Closing(Curly),
                Closing(Square),
                Closing(Square),
                Closing(Round),
                Closing(Curly),
                Closing(Round),
                Closing(Square),
            ])),
            "sixth",
        ),
        (
            "[(()[<>])]({[<{<<[]>>(",
            Some(Ok(vec![
                Closing(Round),
                Closing(Curly),
                Closing(Angle),
                Closing(Square),
                Closing(Curly),
                Closing(Round),
            ])),
            "seventh",
        ),
        (
            "(((({<>}<{<{<>}{[]{[]{}",
            Some(Ok(vec![
                Closing(Curly),
                Closing(Curly),
                Closing(Angle),
                Closing(Curly),
                Closing(Angle),
                Closing(Round),
                Closing(Round),
                Closing(Round),
                Closing(Round),
            ])),
            "eighth",
        ),
        (
            "{<[[]]>}<{[{[{[]{()[[[]",
            Some(Ok(vec![
                Closing(Square),
                Closing(Square),
                Closing(Curly),
                Closing(Curly),
                Closing(Square),
                Closing(Curly),
                Closing(Square),
                Closing(Curly),
                Closing(Angle),
            ])),
            "ninth",
        ),
        (
            "<{([{{}}[<[[[<>{}]]]>[]]",
            Some(Ok(vec![
                Closing(Square),
                Closing(Round),
                Closing(Curly),
                Closing(Angle),
            ])),
            "tenth",
        ),
    ];
    for (s, expected, msg) in test_data {
        let bracket_line = BracketLine::from_str(s).expect("Unsupported character in s");
        let actual = bracket_line.check_validity();
        assert_eq!(actual, expected, "{}", msg);
    }
}

#[test]
fn bracket_matrix_from_str_success() {
    let expected_data: Vec<BracketLine> = vec![
        BracketLine {
            data: vec![
                Opening(Square),
                Opening(Round),
                Opening(Curly),
                Opening(Round),
                Opening(Angle),
                Opening(Round),
                Opening(Round),
                Closing(Round),
                Closing(Round),
                Opening(Square),
                Closing(Square),
                Closing(Angle),
                Opening(Square),
                Opening(Square),
                Opening(Curly),
                Opening(Square),
                Closing(Square),
                Opening(Curly),
                Opening(Angle),
                Opening(Round),
                Closing(Round),
                Opening(Angle),
                Closing(Angle),
                Closing(Angle),
            ],
        },
        BracketLine {
            data: vec![
                Opening(Square),
                Opening(Round),
                Opening(Round),
                Closing(Round),
                Opening(Square),
                Opening(Angle),
                Closing(Angle),
                Closing(Square),
                Closing(Round),
                Closing(Square),
                Opening(Round),
                Opening(Curly),
                Opening(Square),
                Opening(Angle),
                Opening(Curly),
                Opening(Angle),
                Opening(Angle),
                Opening(Square),
                Closing(Square),
                Closing(Angle),
                Closing(Angle),
                Opening(Round),
            ],
        },
        BracketLine {
            data: vec![
                Opening(Curly),
                Opening(Round),
                Opening(Square),
                Opening(Round),
                Opening(Angle),
                Opening(Curly),
                Closing(Curly),
                Opening(Square),
                Opening(Angle),
                Closing(Angle),
                Opening(Square),
                Closing(Square),
                Closing(Curly),
                Closing(Angle),
                Opening(Curly),
                Opening(Square),
                Closing(Square),
                Opening(Curly),
                Opening(Square),
                Opening(Round),
                Opening(Angle),
                Opening(Round),
                Closing(Round),
                Closing(Angle),
            ],
        },
        BracketLine {
            data: vec![
                Opening(Round),
                Opening(Round),
                Opening(Round),
                Opening(Round),
                Opening(Curly),
                Opening(Angle),
                Closing(Angle),
                Closing(Curly),
                Opening(Angle),
                Opening(Curly),
                Opening(Angle),
                Opening(Curly),
                Opening(Angle),
                Closing(Angle),
                Closing(Curly),
                Opening(Curly),
                Opening(Square),
                Closing(Square),
                Opening(Curly),
                Opening(Square),
                Closing(Square),
                Opening(Curly),
                Closing(Curly),
            ],
        },
        BracketLine {
            data: vec![
                Opening(Square),
                Opening(Square),
                Opening(Angle),
                Opening(Square),
                Opening(Round),
                Opening(Square),
                Closing(Square),
                Closing(Round),
                Closing(Round),
                Opening(Angle),
                Opening(Round),
                Opening(Square),
                Opening(Square),
                Opening(Curly),
                Closing(Curly),
                Opening(Square),
                Opening(Square),
                Opening(Round),
                Closing(Round),
                Closing(Square),
                Closing(Square),
                Closing(Square),
            ],
        },
        BracketLine {
            data: vec![
                Opening(Square),
                Opening(Curly),
                Opening(Square),
                Opening(Curly),
                Opening(Round),
                Opening(Curly),
                Closing(Curly),
                Closing(Square),
                Opening(Curly),
                Closing(Curly),
                Closing(Curly),
                Opening(Round),
                Opening(Square),
                Opening(Curly),
                Opening(Square),
                Opening(Curly),
                Opening(Curly),
                Opening(Curly),
                Closing(Curly),
                Closing(Curly),
                Opening(Round),
                Opening(Square),
                Closing(Square),
            ],
        },
        BracketLine {
            data: vec![
                Opening(Curly),
                Opening(Angle),
                Opening(Square),
                Opening(Square),
                Closing(Square),
                Closing(Square),
                Closing(Angle),
                Closing(Curly),
                Opening(Angle),
                Opening(Curly),
                Opening(Square),
                Opening(Curly),
                Opening(Square),
                Opening(Curly),
                Opening(Square),
                Closing(Square),
                Opening(Curly),
                Opening(Round),
                Closing(Round),
                Opening(Square),
                Opening(Square),
                Opening(Square),
                Closing(Square),
            ],
        },
        BracketLine {
            data: vec![
                Opening(Square),
                Opening(Angle),
                Opening(Round),
                Opening(Angle),
                Opening(Round),
                Opening(Angle),
                Opening(Round),
                Opening(Angle),
                Opening(Curly),
                Closing(Curly),
                Closing(Round),
                Closing(Round),
                Closing(Angle),
                Opening(Angle),
                Opening(Round),
                Opening(Square),
                Closing(Square),
                Opening(Round),
                Opening(Square),
                Closing(Square),
                Opening(Round),
                Closing(Round),
            ],
        },
        BracketLine {
            data: vec![
                Opening(Angle),
                Opening(Curly),
                Opening(Round),
                Opening(Square),
                Opening(Round),
                Opening(Square),
                Opening(Square),
                Opening(Round),
                Opening(Angle),
                Closing(Angle),
                Opening(Round),
                Closing(Round),
                Closing(Round),
                Opening(Curly),
                Closing(Curly),
                Closing(Square),
                Closing(Angle),
                Opening(Round),
                Opening(Angle),
                Opening(Angle),
                Opening(Curly),
                Opening(Curly),
            ],
        },
        BracketLine {
            data: vec![
                Opening(Angle),
                Opening(Curly),
                Opening(Round),
                Opening(Square),
                Opening(Curly),
                Opening(Curly),
                Closing(Curly),
                Closing(Curly),
                Opening(Square),
                Opening(Angle),
                Opening(Square),
                Opening(Square),
                Opening(Square),
                Opening(Angle),
                Closing(Angle),
                Opening(Curly),
                Closing(Curly),
                Closing(Square),
                Closing(Square),
                Closing(Square),
                Closing(Angle),
                Opening(Square),
                Closing(Square),
                Closing(Square),
            ],
        },
    ];
    let actual_res = BracketMatrix::from_str(
        fs::read_to_string("small_input.txt")
            .expect("small_input.txt could not be read.")
            .as_str(),
    );
    assert!(actual_res.is_ok(), "Inp is not ok.");
    let actual = actual_res.unwrap();
    assert_eq!(
        actual.data.len(),
        expected_data.len(),
        "Lengths of outer vectors must be equal."
    );
    for (av, ev) in actual.data.into_iter().zip(expected_data.into_iter()) {
        assert_eq!(
            av.data.len(),
            ev.data.len(),
            "Lengths of inner vectors must be pairwise equal."
        );
        for (a, e) in av.data.into_iter().zip(ev.data.into_iter()) {
            assert_eq!(a, e, "Brackets must be equal");
        }
    }
}

#[test]
fn bracket_try_from_char_success() {
    let s = "([{<)]}>";
    let expected: Vec<eyre::Result<Bracket>> = vec![
        Ok(Opening(Round)),
        Ok(Opening(Square)),
        Ok(Opening(Curly)),
        Ok(Opening(Angle)),
        Ok(Closing(Round)),
        Ok(Closing(Square)),
        Ok(Closing(Curly)),
        Ok(Closing(Angle)),
    ];
    let actual: Vec<eyre::Result<Bracket>> = s.chars().map(|c| Bracket::try_from(c)).collect();
    assert_eq!(
        actual.len(),
        expected.len(),
        "Vectors need to be of the same length."
    );
    for (a, e) in actual.into_iter().zip(expected.into_iter()) {
        assert!(a.is_ok() && e.is_ok());
        assert_eq!(a.unwrap(), e.unwrap());
    }
}
