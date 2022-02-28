use crate::{
    calculate_output_value, determine_wiring, solve_1, solve_2, union_count, Notes, SignalEntry,
};
use std::{collections::HashMap, fs, str::FromStr};

#[test]
fn part_two() {
    let s = fs::read_to_string("input.txt").expect("where is input.txt");
    let expected = 915941;
    let notes = Notes::from_str(s.as_str()).expect("couldn't parse input.txt");
    let actual_res = solve_2(&notes);
    assert!(actual_res.is_ok());
    let actual = actual_res.unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn solve_2_works_small() {
    let s = fs::read_to_string("small_input.txt").expect("There should be a small_input.txt");
    let expected = 61229;
    let notes_res = Notes::from_str(s.as_str());
    assert!(notes_res.is_ok());
    let notes = notes_res.unwrap();
    let actual_res = solve_2(&notes);
    assert!(actual_res.is_ok());
    let actual = actual_res.unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn calculate_output_value_works() {
    let expected = 5353;
    let entry = SignalEntry {
        signal_patterns: [
            String::from("abcdefg"),
            String::from("bcdef"),
            String::from("acdfg"),
            String::from("abcdf"),
            String::from("abd"),
            String::from("abcdef"),
            String::from("bcdefg"),
            String::from("abef"),
            String::from("abcdeg"),
            String::from("ab"),
        ],
        output_digits: [
            String::from("bcdef"),
            String::from("abcdf"),
            String::from("bcdef"),
            String::from("abcdf"),
        ],
    };
    let actual_res = calculate_output_value(&entry);
    assert!(actual_res.is_ok());
    let actual = actual_res.unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn determine_wiring_works() {
    let signals = [
        String::from("abcdefg"),
        String::from("bcdef"),
        String::from("acdfg"),
        String::from("abcdf"),
        String::from("abd"),
        String::from("abcdef"),
        String::from("bcdefg"),
        String::from("abef"),
        String::from("abcdeg"),
        String::from("ab"),
    ];

    let eigth = String::from("abcdefg");
    let five = String::from("bcdef");
    let two = String::from("acdfg");
    let three = String::from("abcdf");
    let seven = String::from("abd");
    let nine = String::from("abcdef");
    let six = String::from("bcdefg");
    let four = String::from("abef");
    let zero = String::from("abcdeg");
    let one = String::from("ab");

    let expected = HashMap::from([
        (&zero, 0),
        (&one, 1),
        (&two, 2),
        (&three, 3),
        (&four, 4),
        (&five, 5),
        (&six, 6),
        (&seven, 7),
        (&eigth, 8),
        (&nine, 9),
    ]);
    let actual_res = determine_wiring(&signals);
    if let Err(ref report) = actual_res {
        dbg!(report);
    }
    assert!(actual_res.is_ok());
    let actual = actual_res.unwrap();
    assert_eq!(actual, expected)
}

#[test]
fn union_count_works() {
    let test_data = vec![
        (String::from("abdefg"), String::from("cf"), 1, "first"),
        (String::from("abc"), String::from("def"), 0, "second"),
        (String::from("abcdefg"), String::from("abcdefg"), 7, "third"),
    ];

    for (s1, s2, expected, msg) in test_data {
        let actual = union_count(&s1, &s2);
        assert_eq!(actual, expected, "{}", msg);
    }
}

#[test]
fn part_one() {
    let s = fs::read_to_string("input.txt").expect("where is input.txt");
    let expected = 310;
    let notes = Notes::from_str(s.as_str()).expect("couldn't parse input.txt");
    let actual = solve_1(&notes);
    assert_eq!(actual, expected);
}

#[test]
fn solve_1_works_small() {
    let s = fs::read_to_string("small_input.txt").expect("There should be a small_input.txt");
    let expected = 26;
    let notes_res = Notes::from_str(s.as_str());
    assert!(notes_res.is_ok());
    let notes = notes_res.unwrap();
    let actual = solve_1(&notes);
    assert_eq!(actual, expected);
}

#[test]
fn parse_signal_entry() {
    let s =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
    let expected = SignalEntry {
        signal_patterns: [
            String::from("be"),
            String::from("abcdefg"),
            String::from("bcdefg"),
            String::from("acdefg"),
            String::from("bceg"),
            String::from("cdefg"),
            String::from("abdefg"),
            String::from("bcdef"),
            String::from("abcdf"),
            String::from("bde"),
        ],
        output_digits: [
            String::from("abcdefg"),
            String::from("bcdef"),
            String::from("bcdefg"),
            String::from("bceg"),
        ],
    };
    let actual_res = SignalEntry::from_str(s);
    assert!(actual_res.is_ok());
    let actual = actual_res.unwrap();
    assert_eq!(actual, expected);
}
