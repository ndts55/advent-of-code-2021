use eyre::eyre;
use std::{collections::HashSet, fs, str::FromStr};

fn main() -> eyre::Result<()> {
    let s = fs::read_to_string("input.txt")?;
    let octo_grid = OctoGrid::from_str(&s)?;
    let part_one = solve_1(&octo_grid);
    println!("Part One {part_one}");
    let part_two = solve_2(&octo_grid);
    println!("Part Two {part_two}");

    Ok(())
}

fn solve_2(octo_grid: &OctoGrid) -> u32 {
    let target_flash_count = octo_grid.octopi.len() as u32;
    octo_grid
        .flash_counter()
        .enumerate()
        .find_map(|(step_count, flash_count)| {
            if flash_count == target_flash_count {
                Some(step_count + 1)
            } else {
                None
            }
        })
        .unwrap_or_default() as u32
}

fn solve_1(octo_grid: &OctoGrid) -> u32 {
    octo_grid.flash_counter().take(100).sum()
}

#[derive(Debug, PartialEq)]
struct FlashCounter {
    octopi: Vec<u8>,
    dim: usize,
}

fn neighbours(idx: usize, dim: usize) -> Vec<usize> {
    let mut nbs = Vec::new();
    // Pre-calculate edge conditions.
    let is_top_row = idx / dim == 0;
    let is_bot_row = idx / dim + 1 == dim;
    let is_left_col = idx % dim == 0;
    let is_right_col = idx % dim + 1 == dim;
    // Add entries to nbs.
    // a b c
    // d e f
    // g h i
    // We are calculating the neighbours to e.
    if !is_top_row {
        // b
        nbs.push(idx - dim);

        if !is_left_col {
            // a
            nbs.push(idx - dim - 1);
        }

        if !is_right_col {
            // c
            nbs.push(idx - dim + 1);
        }
    }

    if !is_bot_row {
        // h
        nbs.push(idx + dim);

        if !is_left_col {
            // g
            nbs.push(idx + dim - 1);
        }

        if !is_right_col {
            // i
            nbs.push(idx + dim + 1);
        }
    }

    if !is_left_col {
        // d
        nbs.push(idx - 1);
    }

    if !is_right_col {
        // f
        nbs.push(idx + 1);
    }

    nbs
}

impl Iterator for FlashCounter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // 1. increase all by 1
        for energy_level in self.octopi.iter_mut() {
            *energy_level += 1;
        }

        // 2. flash anything over 9 and increase surrounding by 1
        // 3. Repeat 2. until all are below 9
        // Accumulate a running total of the number of flashes
        // Octopi can't flash more than once in a step
        // Tracking which octopi flashed gives us the number at the end
        let mut flashed: HashSet<usize> = HashSet::new();
        while self
            .octopi
            .iter()
            .enumerate()
            .any(|(idx, el)| *el > 9 && !flashed.contains(&idx))
        {
            let flash_pending = self
                .octopi
                .iter()
                .enumerate()
                .filter(|(idx, el)| **el > 9 && !flashed.contains(idx))
                .map(|(idx, _)| idx)
                .collect::<HashSet<_>>();

            for index in flash_pending
                .iter()
                .flat_map(|idx| neighbours(*idx, self.dim))
                .filter(|idx| !flashed.contains(idx))
            {
                if let Some(energy_level) = self.octopi.get_mut(index) {
                    *energy_level += 1;
                }
            }

            flashed.extend(flash_pending.into_iter());
        }

        for index in flashed.iter() {
            if let Some(energy_level) = self.octopi.get_mut(*index) {
                *energy_level = 0;
            }
        }

        Some(flashed.len() as u32)
    }
}

#[derive(Debug, PartialEq)]
struct OctoGrid {
    octopi: Vec<u8>,
    dim: usize,
}

impl OctoGrid {
    fn flash_counter(&self) -> FlashCounter {
        FlashCounter {
            octopi: self.octopi.clone(),
            dim: self.dim as usize,
        }
    }
}

impl FromStr for OctoGrid {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let side_length = lines.len();
        let octopi = lines
            .into_iter()
            .flat_map(|line| line.chars())
            .map(|c| {
                c.to_digit(10)
                    .ok_or(eyre!("Could not convert {} to a number", c))
            })
            .collect::<eyre::Result<Vec<_>>>()?
            .into_iter()
            .map(|n| n as u8)
            .collect();

        Ok(Self {
            octopi,
            dim: side_length,
        })
    }
}

#[cfg(test)]
mod tests;
