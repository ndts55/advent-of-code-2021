use std::fs;

use crate::{herd_deadlock, CucumberPositions, Herds};

#[test]
#[ignore]
fn part_one() {
    let expected = 482;
    let s = fs::read_to_string("input.txt").unwrap();
    let mut herds = Herds::from(s.as_str());
    let actual = herd_deadlock(&mut herds);
    assert_eq!(actual, expected);
}

#[test]
fn herd_deadlock_after_58_iterations() {
    let expected = 58;
    let mut herds = small_herds();
    let actual = herd_deadlock(&mut herds);
    assert_eq!(actual, expected);
}

#[test]
fn small_input_first_5_steps() {
    let expected_positions = vec![
        CucumberPositions::from(small_input().as_str()),
        CucumberPositions::from("....>.>v.>\nv.v>.>v.v.\n>v>>..>v..\n>>v>v>.>.v\n.>v.v...v.\nv>>.>vvv..\n..v...>>..\nvv...>>vv.\n>.v.v..v.v\n"),
        CucumberPositions::from(">.v.v>>..v\nv.v.>>vv..\n>v>.>.>.v.\n>>v>v.>v>.\n.>..v....v\n.>v>>.v.v.\nv....v>v>.\n.vv..>>v..\nv>.....vv."),
        CucumberPositions::from("v>v.v>.>v.\nv...>>.v.v\n>vv>.>v>..\n>>v>v.>.v>\n..>....v..\n.>.>v>v..v\n..v..v>vv>\nv.v..>>v..\n.v>....v.."),
        CucumberPositions::from("v>..v.>>..\nv.v.>.>.v.\n>vv.>>.v>v\n>>.>..v>.>\n..v>v...v.\n..>>.>vv..\n>.v.vv>v.v\n.....>>vv.\nvvv>...v.."),
        CucumberPositions::from("vv>...>v>.\nv.v.v>.>v.\n>.v.>.>.>v\n>v>.>..v>>\n..v>v.v...\n..>.>>vvv.\n.>...v>v..\n..v.v>>v.v\nv.v.>...v."),
    ];
    let mut herds = small_herds();
    for CucumberPositions {
        east: expected_east,
        south: expected_south,
    } in expected_positions
    {
        let m = herds.next();
        assert!(m.is_some(), "next() returned None");
        let CucumberPositions {
            east: actual_east,
            south: actual_south,
        } = m.unwrap();
        assert_eq!(
            actual_east.len(),
            expected_east.len(),
            "east lengths differ"
        );
        assert_eq!(
            actual_south.len(),
            expected_south.len(),
            "south lengths differ"
        );
        for ee in expected_east.iter() {
            assert!(actual_east.contains(ee), "actual east is missing an item");
            dbg!(ee);
        }
        for ae in actual_east.iter() {
            assert!(
                expected_east.contains(ae),
                "expected east is missing an item"
            );
            dbg!(ae);
        }
        for es in expected_south.iter() {
            assert!(actual_south.contains(es), "actual south is missing an item");
            dbg!(es);
        }
        for as_ in actual_south.iter() {
            assert!(
                expected_south.contains(as_),
                "expected south is missing an item"
            );
            dbg!(as_);
        }
    }
}

#[test]
fn parse_small_input_works() {
    let expected = Herds {
        cucumbers: CucumberPositions {
            east: vec![
                (4, 0),
                (5, 0),
                (9, 0),
                (3, 1),
                (4, 1),
                (0, 2),
                (1, 2),
                (3, 2),
                (5, 2),
                (0, 3),
                (1, 3),
                (3, 3),
                (4, 3),
                (6, 3),
                (1, 4),
                (0, 5),
                (2, 5),
                (3, 5),
                (5, 6),
                (7, 6),
                (5, 7),
                (6, 7),
                (9, 8),
            ],
            south: vec![
                (0, 0),
                (7, 0),
                (8, 0),
                (1, 1),
                (2, 1),
                (6, 1),
                (7, 1),
                (4, 2),
                (9, 2),
                (2, 3),
                (8, 3),
                (0, 4),
                (2, 4),
                (4, 4),
                (5, 4),
                (7, 4),
                (6, 5),
                (1, 6),
                (2, 6),
                (8, 6),
                (0, 7),
                (2, 7),
                (7, 7),
                (9, 7),
                (4, 8),
                (7, 8),
            ],
        },
        max_x: 9,
        max_y: 8,
    };
    let s = small_input();
    let actual = Herds::from(s.as_str());
    assert_eq!(actual.cucumbers, expected.cucumbers, "cucumbers differ");
    assert_eq!(actual.max_x, expected.max_x, "widths differ");
    assert_eq!(actual.max_y, expected.max_y, "heights differ");
}

fn small_input() -> String {
    fs::read_to_string("small_input.txt").unwrap()
}

fn small_herds() -> Herds {
    Herds::try_from(small_input().as_str()).unwrap()
}
