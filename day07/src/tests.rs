use std::fs;

use crate::{solve_part_1, solve_part_2, Crabs};

#[test]
fn part_2() {
    let expected = 95519693;
    let crabs = big_crabs();
    let actual = solve_part_2(&crabs);
    assert_eq!(actual, expected);
}

#[test]
fn part_2_small() {
    let expected = 168;
    let crabs = small_crabs();
    let actual = solve_part_2(&crabs);
    assert_eq!(actual, expected);
}

#[test]
fn additive_fuel_cost_for_alignment_works() {
    let expected = vec![(5, 168), (2, 206)];
    let crabs = small_crabs();
    for (alignment, expected_cost) in expected.into_iter() {
        let actual_cost = crabs.additive_fuel_cost_for_alignment(alignment);
        assert_eq!(actual_cost, expected_cost);
    }
}

#[test]
fn mean_works() {
    let expected = 5;
    let crabs = small_crabs();
    let m_actual = crabs.mean();
    assert!(m_actual.is_some(), "mean() returns None");
    let actual = m_actual.unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn part_1() {
    let expected = 352707;
    let crabs = big_crabs();
    let actual = solve_part_1(&crabs);
    assert_eq!(actual, expected);
}

#[test]
fn part_1_small() {
    let expected = 37;
    let crabs = small_crabs();
    let actual = solve_part_1(&crabs);
    assert_eq!(actual, expected);
}

#[test]
fn simple_fuel_cost_for_alignment_works() {
    let expected = vec![(1, 41), (2, 37), (3, 39), (10, 71)];
    let crabs = small_crabs();
    for (alignment, expected_cost) in expected.into_iter() {
        let actual_cost = crabs.simple_fuel_cost_for_alignment(alignment);
        assert_eq!(actual_cost, expected_cost);
    }
}

#[test]
fn median_works() {
    let expected = 2;
    let crabs = small_crabs();
    let m_actual = crabs.median();
    assert!(m_actual.is_some(), "median() returns None");
    let actual = m_actual.unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn crab_positions_from_small_input() {
    let expected = Crabs {
        positions: vec![0, 1, 1, 2, 2, 2, 4, 7, 14, 16],
    };
    let actual = small_crabs();
    assert_eq!(actual, expected);
}

fn small_crabs() -> Crabs {
    Crabs::from(fs::read_to_string("small_input.txt").unwrap())
}

fn big_crabs() -> Crabs {
    Crabs::from(fs::read_to_string("input.txt").unwrap())
}
