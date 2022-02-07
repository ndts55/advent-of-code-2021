use std::{collections::HashMap, fs, num::ParseIntError, str::FromStr};

fn main() {
    let s = fs::read_to_string("input.txt").unwrap();
    let mut lanternfishes = Lanternfishes::from_str(s.as_str()).unwrap();
    let part_one = solve_part_1(&mut lanternfishes);
    println!("Part One: {part_one}");
    let part_two = solve_part_2(&mut lanternfishes);
    println!("Part Two: {part_two}");
}

fn solve_part_1(lanternfishes: &mut Lanternfishes) -> <Lanternfishes as Iterator>::Item {
    lanternfishes.nth(80).unwrap_or_default()
}

fn solve_part_2(lanternfishes: &mut Lanternfishes) -> <Lanternfishes as Iterator>::Item {
    lanternfishes.nth(256 - 80 - 1).unwrap_or_default()
}

#[derive(Debug, PartialEq)]
struct Lanternfishes {
    fishes: HashMap<u8, u128>,
}

impl Iterator for Lanternfishes {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        // todo!()
        // Calculate number of fishes `fish_count`.
        // Calculate new number of fishes by internal timers (next HashMap).
        // Return `fish_count`.
        let fish_count = self.fishes.values().into_iter().sum();

        self.fishes = self
            .fishes
            .iter()
            .fold(HashMap::new(), |mut hm, (timer, count)| {
                match timer {
                    0 => {
                        // 0 becomes 6...
                        *(hm.entry(6)).or_default() += count;
                        // ...and adds an 8.
                        *(hm.entry(8)).or_default() += count;
                    }
                    n => {
                        // Every other value is decremented.
                        *(hm.entry(n - 1)).or_default() += count;
                    }
                }

                hm
            });

        Some(fish_count)
    }
}

impl FromStr for Lanternfishes {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fishes = s
            .split(',')
            .map(|nstr| nstr.parse::<u8>())
            .collect::<Result<Vec<u8>, Self::Err>>()?
            .into_iter()
            .fold(HashMap::new(), |mut hm, n| {
                *(hm.entry(n)).or_default() += 1;

                hm
            });

        Ok(Self { fishes })
    }
}

#[cfg(test)]
mod tests;
