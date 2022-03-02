use std::{fs, str::FromStr};

use crate::{neighbours, solve_1, solve_2, OctoGrid};

#[test]
fn solve_2_small_input() {
    let expected = 195;
    let octo_grid = small_grid();
    let actual = solve_2(&octo_grid);
    assert_eq!(actual, expected);
}

#[test]
fn solve_1_input() {
    let expected = 1688;
    let octo_grid = grid();
    let actual = solve_1(&octo_grid);
    assert_eq!(actual, expected);
}

#[test]
fn solve_1_small_input() {
    let expected = 1656;
    let octo_grid = small_grid();
    let actual = solve_1(&octo_grid);
    assert_eq!(actual, expected);
}

#[test]
fn iterator_flash_counter() {
    let expected_1 = Some(9);
    let expected_2 = Some(0);
    let expected_octopi_1 = vec![
        3, 4, 5, 4, 3, 4, 0, 0, 0, 4, 5, 0, 0, 0, 5, 4, 0, 0, 0, 4, 3, 4, 5, 4, 3,
    ];
    let expected_octopi_2 = vec![
        4, 5, 6, 5, 4, 5, 1, 1, 1, 5, 6, 1, 1, 1, 6, 5, 1, 1, 1, 5, 4, 5, 6, 5, 4,
    ];
    let grid = mini_grid();
    let mut counter = grid.flash_counter();
    assert_eq!(counter.next(), expected_1);
    assert_eq!(counter.octopi, expected_octopi_1);
    assert_eq!(counter.next(), expected_2);
    assert_eq!(counter.octopi, expected_octopi_2);
}

#[test]
fn neighbours_flash_counter() {
    let dim = 5;
    let test_data = vec![
        (0, vec![5, 6, 1], "zero"),
        (1, vec![6, 5, 7, 0, 2], "one"),
        (2, vec![7, 6, 8, 1, 3], "two"),
        (3, vec![8, 7, 9, 2, 4], "three"),
        (4, vec![9, 8, 3], "four"),
        (5, vec![0, 1, 10, 11, 6], "five"),
        (6, vec![1, 0, 2, 11, 10, 12, 5, 7], "six"),
        (7, vec![2, 1, 3, 12, 11, 13, 6, 8], "seven"),
        (8, vec![3, 2, 4, 13, 12, 14, 7, 9], "eight"),
        (9, vec![4, 3, 14, 13, 8], "nine"),
        (10, vec![5, 6, 15, 16, 11], "ten"),
        (11, vec![6, 5, 7, 16, 15, 17, 10, 12], "eleven"),
        (12, vec![7, 6, 8, 17, 16, 18, 11, 13], "twelve"),
        (13, vec![8, 7, 9, 18, 17, 19, 12, 14], "thirteen"),
        (14, vec![9, 8, 19, 18, 13], "fourteen"),
        (15, vec![10, 11, 20, 21, 16], "fifteen"),
        (16, vec![11, 10, 12, 21, 20, 22, 15, 17], "sixteen"),
        (17, vec![12, 11, 13, 22, 21, 23, 16, 18], "seventeen"),
        (18, vec![13, 12, 14, 23, 22, 24, 17, 19], "eighteen"),
        (19, vec![14, 13, 24, 23, 18], "nineteen"),
        (20, vec![15, 16, 21], "twenty"),
        (21, vec![16, 15, 17, 20, 22], "twentyone"),
        (22, vec![17, 16, 18, 21, 23], "twentytwo"),
        (23, vec![18, 17, 19, 22, 24], "twentythree"),
        (24, vec![19, 18, 23], "twentyfour"),
    ];

    for (idx, expected, msg) in test_data {
        let actual = neighbours(idx, dim);
        assert_eq!(actual, expected, "{}", msg);
    }
}

#[test]
fn mini_octo_grid_from_str() {
    let expected = OctoGrid {
        octopi: vec![
            1, 1, 1, 1, 1, 1, 9, 9, 9, 1, 1, 9, 1, 9, 1, 1, 9, 9, 9, 1, 1, 1, 1, 1, 1,
        ],
        dim: 5,
    };
    let s = fs::read_to_string("mini_input.txt").expect("where is mini_input.txt");
    let grid_res = OctoGrid::from_str(s.as_str());
    assert!(grid_res.is_ok());
    let actual = grid_res.unwrap();
    assert_eq!(actual, expected);
}

fn mini_grid() -> OctoGrid {
    let s = fs::read_to_string("mini_input.txt").expect("mini_input.txt missing");
    OctoGrid::from_str(s.as_str()).expect("can't parse mini_input.txt")
}

fn small_grid() -> OctoGrid {
    let s = fs::read_to_string("small_input.txt").expect("small_input.txt?");
    OctoGrid::from_str(s.as_str()).expect("can't parse small_input.txt")
}

fn grid() -> OctoGrid {
    let s = fs::read_to_string("input.txt").expect("input.txt?");
    OctoGrid::from_str(&s).expect("can't parse input.txt")
}
