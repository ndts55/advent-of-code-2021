use std::{collections::HashMap, fs, str::FromStr};

use crate::Lanternfishes;

#[test]
fn part_one() {
    let expected = 387413;
    let mut lanternfishes = input_lanternfishes().unwrap();
    let actual = lanternfishes.nth(80).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn part_two() {
    let expected = 1738377086345;
    let mut lanternfishes = input_lanternfishes().unwrap();
    let actual = lanternfishes.nth(256).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn small_input_after_18_days() {
    let expected = 26;
    let mut lanternfishes = small_input_lanternfishes().unwrap();
    let m_actual = lanternfishes.nth(18);
    assert!(m_actual.is_some());
    let actual = m_actual.unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn small_input_after_80_days() {
    let expected = 5934;
    let mut lanternfishes = small_input_lanternfishes().unwrap();
    let m_actual = lanternfishes.nth(80);
    assert!(m_actual.is_some());
    let actual = m_actual.unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn small_input_after_256_days() {
    let expected = 26984457539;
    let mut lanternfishes = small_input_lanternfishes().unwrap();
    let m_actual = lanternfishes.nth(256);
    assert!(m_actual.is_some());
    let actual = m_actual.unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn small_input_after_80_then_256_days() {
    let expected_80 = 5934;
    let expected_256 = 26984457539;
    let mut lanternfishes = small_input_lanternfishes().unwrap();
    let actual_80 = lanternfishes.nth(80).unwrap();
    assert_eq!(actual_80, expected_80);
    let actual_256 = lanternfishes.nth(256 - 80 - 1).unwrap();
    assert_eq!(actual_256, expected_256);
}

#[test]
fn lanternfishes_iterator_works() {
    let expected_1 = Lanternfishes {
        fishes: HashMap::from([(0, 1), (1, 1), (2, 2), (3, 1)]),
    };
    let expected_2 = Lanternfishes {
        fishes: HashMap::from([(0, 1), (1, 2), (2, 1), (6, 1), (8, 1)]),
    };
    let expected_3 = Lanternfishes {
        fishes: HashMap::from([(0, 2), (1, 1), (5, 1), (6, 1), (7, 1), (8, 1)]),
    };
    let mut actual = small_input_lanternfishes().unwrap();
    let zeroth_day = actual.next().unwrap();
    assert_eq!(5, zeroth_day);
    assert_eq!(actual, expected_1);
    let first_day = actual.next().unwrap();
    assert_eq!(5, first_day);
    assert_eq!(actual, expected_2);
    let second_day = actual.next().unwrap();
    assert_eq!(6, second_day);
    assert_eq!(actual, expected_3);
}

#[test]
fn parse_small_input_works() {
    // 3,4,3,1,2
    let expected = Lanternfishes {
        fishes: HashMap::from([(1, 1), (2, 1), (3, 2), (4, 1)]),
    };
    let actual_res = small_input_lanternfishes();
    assert!(actual_res.is_ok());
    let actual = actual_res.unwrap();
    assert_eq!(actual, expected);
}

fn small_input_lanternfishes() -> Result<Lanternfishes, <Lanternfishes as FromStr>::Err> {
    let s = fs::read_to_string("small_input.txt").unwrap();
    Lanternfishes::from_str(s.as_str())
}

fn input_lanternfishes() -> Result<Lanternfishes, <Lanternfishes as FromStr>::Err> {
    let s = fs::read_to_string("input.txt").unwrap();
    Lanternfishes::from_str(s.as_str())
}
