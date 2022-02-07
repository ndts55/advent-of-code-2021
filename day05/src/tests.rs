use std::{fs, str::FromStr};

use crate::{parse_input, solve_part_one, solve_part_two, Point, Vents};

#[test]
fn solve_part_two_small_input() {
    let expected = 12;
    let s = fs::read_to_string("small_input.txt").unwrap();
    let inp = parse_input(s.as_str()).unwrap();
    let actual = solve_part_two(&inp);
    assert_eq!(actual, expected);
}

#[test]
fn solve_part_two_input() {
    let expected = 21140;
    let s = fs::read_to_string("input.txt").unwrap();
    let vents = parse_input(s.as_str()).unwrap();
    let actual = solve_part_two(&vents);
    assert_eq!(actual, expected);
}

#[test]
fn solve_part_one_small_input() {
    let expected = 5;
    let s = fs::read_to_string("small_input.txt").unwrap();
    let inp = parse_input(s.as_str()).unwrap();
    let actual = solve_part_one(&inp);
    assert_eq!(actual, expected);
}

#[test]
fn solve_part_one_input() {
    let expected = 7269;
    let s = fs::read_to_string("input.txt").unwrap();
    let vents = parse_input(s.as_str()).unwrap();
    let actual = solve_part_one(&vents);
    assert_eq!(actual, expected);
}

#[test]
fn ventline_impl_iterator_works_diagonal() {
    let expected = vec![
        Point { x: 8, y: 0 },
        Point { x: 7, y: 1 },
        Point { x: 6, y: 2 },
        Point { x: 5, y: 3 },
        Point { x: 4, y: 4 },
        Point { x: 3, y: 5 },
        Point { x: 2, y: 6 },
        Point { x: 1, y: 7 },
        Point { x: 0, y: 8 },
    ];
    let actual: Vec<Point> = Vents::new(Point { x: 8, y: 0 }, Point { x: 0, y: 8 })
        .points()
        .into_iter()
        .collect();

    assert_eq!(actual, expected);
}

#[test]
fn ventline_impl_iterator_works_horizontal() {
    let expected = vec![
        Point { x: 0, y: 9 },
        Point { x: 1, y: 9 },
        Point { x: 2, y: 9 },
        Point { x: 3, y: 9 },
        Point { x: 4, y: 9 },
        Point { x: 5, y: 9 },
    ];
    let actual: Vec<Point> = Vents::new(Point { x: 0, y: 9 }, Point { x: 5, y: 9 })
        .points()
        .into_iter()
        .collect();

    assert_eq!(actual, expected);
}

#[test]
fn ventline_impl_iterator_works_vertical() {
    let expected = vec![
        Point { x: 4, y: 4 },
        Point { x: 4, y: 3 },
        Point { x: 4, y: 2 },
        Point { x: 4, y: 1 },
        Point { x: 4, y: 0 },
    ];
    let actual: Vec<Point> = Vents::new(Point { x: 4, y: 4 }, Point { x: 4, y: 0 })
        .points()
        .into_iter()
        .collect();

    assert_eq!(actual, expected);
}

#[test]
fn parse_point_success() {
    let expected = Point { x: 2, y: 4 };
    let actual_res = Point::from_str("2,4 ");
    assert!(actual_res.is_ok());
    let actual = actual_res.unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn parse_point_failure() {
    for inp in vec!["asdf", "2,", ",4 ", "abc,2", "2,abc "] {
        assert!(Point::from_str(inp).is_err());
    }
}

#[test]
fn parse_ventline_success() {
    let expected = Vents::new(Point { x: 0, y: 9 }, Point { x: 5, y: 9 });
    let actual_res = Vents::from_str("0,9 -> 5,9");
    assert!(actual_res.is_ok());
    let actual = actual_res.unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn parse_ventline_failure() {
    for inp in vec!["0,9 to 5,9", "0,9 -> five,9", "9 -> 5,9", "blub"] {
        assert!(Vents::from_str(inp).is_err());
    }
}

#[test]
fn parse_small_input_works() {
    let expected = vec![
        Vents::new(Point { x: 0, y: 9 }, Point { x: 5, y: 9 }),
        Vents::new(Point { x: 8, y: 0 }, Point { x: 0, y: 8 }),
        Vents::new(Point { x: 9, y: 4 }, Point { x: 3, y: 4 }),
        Vents::new(Point { x: 2, y: 2 }, Point { x: 2, y: 1 }),
        Vents::new(Point { x: 7, y: 0 }, Point { x: 7, y: 4 }),
        Vents::new(Point { x: 6, y: 4 }, Point { x: 2, y: 0 }),
        Vents::new(Point { x: 0, y: 9 }, Point { x: 2, y: 9 }),
        Vents::new(Point { x: 3, y: 4 }, Point { x: 1, y: 4 }),
        Vents::new(Point { x: 0, y: 0 }, Point { x: 8, y: 8 }),
        Vents::new(Point { x: 5, y: 5 }, Point { x: 8, y: 2 }),
    ];
    let s = fs::read_to_string("small_input.txt").unwrap();
    let actual_res = parse_input(s.as_str());
    assert!(actual_res.is_ok());
    let actual = actual_res.unwrap();
    assert_eq!(actual, expected);
}
