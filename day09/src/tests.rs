use std::fs;

use crate::{solve_1, solve_2, Matrix};

#[test]
fn solve_2_works() {
    let matrix = Matrix::from(slurp("small_input.txt"));
    let expected = 1134;
    let actual = solve_2(&matrix);
    assert_eq!(actual, expected);
}

#[test]
fn solve_1_works() {
    let matrix = Matrix::from(slurp("small_input.txt"));
    let expected = 15;
    let actual = solve_1(&matrix);
    assert_eq!(actual, expected);
}

#[test]
fn get_surrounding_from_matrix() {
    // 2199943210
    // 3987894921
    // 9856789892
    // 8767896789
    // 9899965678
    let matrix = Matrix::from(slurp("small_input.txt"));
    let expected = vec![
        // left, right, up, down
        (0, 0, vec![1, 3]),
        (3, 3, vec![6, 6, 8, 9]),
    ];
    for (x, y, surrounding) in expected {
        let mut actual_surrounding = matrix.get_surrounding(x, y);
        actual_surrounding.sort_unstable();
        assert_eq!(actual_surrounding, surrounding);
    }
}

#[test]
fn get_sample_elements_from_matrix() {
    let matrix = Matrix::from(slurp("small_input.txt"));
    let expected = vec![
        (0, 0, Some(2)),
        (0, 1, Some(3)),
        (1, 0, Some(1)),
        (0, 10, None),
        (5, 2, Some(8)),
    ];
    for (x, y, expected_item) in expected {
        assert_eq!(matrix.get(x, y), expected_item);
    }
}

#[test]
fn parse_matrix_small_input() {
    let expected = Matrix {
        data: vec![
            2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9, 8, 5, 6, 7, 8, 9, 8, 9,
            2, 8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9, 8, 9, 9, 9, 6, 5, 6, 7, 8,
        ],
        width: 10,
        height: 5,
    };
    let s = slurp("small_input.txt");
    let matrix = Matrix::from(s);
    assert_eq!(matrix, expected);
}

fn slurp(file: &str) -> String {
    fs::read_to_string(file).unwrap()
}
