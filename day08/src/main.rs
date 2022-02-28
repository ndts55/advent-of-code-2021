use eyre::eyre;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs,
    str::FromStr,
};

fn main() -> eyre::Result<()> {
    let s = fs::read_to_string("input.txt")?;
    let notes = Notes::from_str(s.as_str())?;
    let part_one = solve_1(&notes);
    println!("Part One: {part_one}");
    let part_two = solve_2(&notes)?;
    println!("Part Two: {part_two}");

    Ok(())
}

fn solve_1(notes: &Notes) -> u32 {
    let sizes = vec![2, 4, 3, 7];
    notes
        .entries
        .iter()
        .flat_map(|e| &e.output_digits)
        .map(|s| s.len())
        .filter(|l| sizes.contains(l))
        .count() as u32
}

fn solve_2(notes: &Notes) -> eyre::Result<usize> {
    notes
        .entries
        .iter()
        .map(calculate_output_value)
        .collect::<eyre::Result<Vec<_>>>()
        .map(|v| v.into_iter().sum())
}

fn calculate_output_value(entry: &SignalEntry) -> eyre::Result<usize> {
    let wiring: HashMap<&String, u8> = determine_wiring(&entry.signal_patterns)?;
    Ok(entry
        .output_digits
        .iter()
        .map(|d| wiring.get(d).ok_or(eyre!("Could not find {} in wiring", d)))
        .collect::<eyre::Result<Vec<_>>>()?
        .into_iter()
        .fold(0, |mut acc, d| {
            acc *= 10;
            acc += *d as usize;
            acc
        }))
}

fn determine_wiring(signals: &[String; 10]) -> eyre::Result<HashMap<&String, u8>> {
    let unique_lengths = signals.iter().fold(HashMap::new(), |mut acc, s| {
        match s.len() {
            2 => {
                acc.insert(1, s);
            }
            4 => {
                acc.insert(4, s);
            }
            3 => {
                acc.insert(7, s);
            }
            7 => {
                acc.insert(8, s);
            }
            _ => {}
        }
        acc
    });

    let f = |n| unique_lengths.get(&n).ok_or(eyre!("No pattern for {}", n));

    let one = *f(1)?;
    let four = *f(4)?;
    let seven = *f(7)?;
    let eight = *f(8)?;

    let mut wiring = HashMap::from([(one, 1), (four, 4), (seven, 7), (eight, 8)]);
    for pattern in signals {
        if wiring.contains_key(&pattern) {
            continue;
        }

        match pattern.len() {
            5 => {
                wiring.insert(
                    pattern,
                    if union_count(&pattern, &one) == one.len() {
                        3
                    } else if union_count(&pattern, &four) == 3 {
                        5
                    } else {
                        2
                    },
                );
            }
            6 => {
                wiring.insert(
                    pattern,
                    if union_count(&pattern, &one) == 1 {
                        6
                    } else if union_count(&pattern, &four) == four.len() {
                        9
                    } else {
                        0
                    },
                );
            }
            _ => return Err(eyre!("This one isn't right {}", pattern)),
        }
    }

    if wiring.len() == 10 {
        Ok(wiring)
    } else {
        Err(eyre!("Couldn't put together 10 entries for wiring."))
    }
}

fn union_count(s1: &String, s2: &String) -> usize {
    if s1 == s2 {
        return s1.len();
    }

    // We assume that the string has no duplicate letters and is sorted.
    s1.chars()
        .collect::<HashSet<char>>()
        .intersection(&s2.chars().collect::<HashSet<char>>())
        .count()
}

#[derive(Debug, PartialEq)]
struct SignalEntry {
    signal_patterns: [String; 10],
    output_digits: [String; 4],
}

impl FromStr for SignalEntry {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.split('|').collect();
        let pattern_str = v.get(0).ok_or(eyre!("Missing unique signal patterns."))?;
        let output_str = v.get(1).ok_or(eyre!("Missing output."))?;
        let signal_patterns = parse_unique_patterns(&pattern_str)?;
        let output_digits = parse_output(&output_str)?;

        Ok(Self {
            signal_patterns,
            output_digits,
        })
    }
}

fn parse_unique_patterns(s: &str) -> eyre::Result<[String; 10]> {
    let mut patterns = word_vec(s);

    if patterns.len() != 10 {
        return Err(eyre!(
            "Incorrect number of unique patterns: {}",
            patterns.len()
        ));
    }

    Ok([
        patterns.swap_remove(0),
        patterns.swap_remove(1),
        patterns.swap_remove(2),
        patterns.swap_remove(3),
        patterns.swap_remove(4),
        patterns.swap_remove(4),
        patterns.swap_remove(3),
        patterns.swap_remove(2),
        patterns.swap_remove(1),
        patterns.swap_remove(0),
    ])
}

fn parse_output(s: &str) -> eyre::Result<[String; 4]> {
    let mut digits = word_vec(s);
    if digits.len() != 4 {
        return Err(eyre!("Incorrect number of output digits: {}", digits.len()));
    }

    Ok([
        digits.swap_remove(0),
        digits.swap_remove(1),
        digits.swap_remove(1),
        digits.swap_remove(0),
    ])
}

fn word_vec(s: &str) -> Vec<String> {
    s.trim()
        .split_whitespace()
        .map(|w| w.chars().sorted().collect())
        .collect::<Vec<_>>()
}

#[derive(Debug, PartialEq)]
struct Notes {
    entries: Vec<SignalEntry>,
}

impl FromStr for Notes {
    type Err = eyre::Report;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entries = s
            .lines()
            .map(SignalEntry::from_str)
            .collect::<Result<Vec<_>, <SignalEntry as FromStr>::Err>>()?;
        Ok(Self { entries })
    }
}

#[cfg(test)]
mod tests;
