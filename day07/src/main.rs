use std::fs;

fn main() {
    let s = fs::read_to_string("input.txt").unwrap();
    let crabs = Crabs::from(s);
    let part_1 = solve_part_1(&crabs);
    println!("Part One: {part_1}");
    let part_2 = solve_part_2(&crabs);
    println!("Part Two: {part_2}");
}

fn solve_part_1(crabs: &Crabs) -> u32 {
    if let Some(m) = crabs.median() {
        crabs.simple_fuel_cost_for_alignment(m)
    } else {
        0
    }
}

fn solve_part_2(crabs: &Crabs) -> u32 {
    if let Some(mean) = crabs.mean() {
        // It's not stupid if it works.
        (mean - 1..=mean + 1)
            .into_iter()
            .map(|m| crabs.additive_fuel_cost_for_alignment(m))
            .min()
            .unwrap_or_default()
    } else {
        0
    }
}

#[derive(Debug, PartialEq)]
struct Crabs {
    positions: Vec<u32>,
}

impl Crabs {
    fn median(&self) -> Option<u32> {
        let k = (self.positions.len() as f32 / 2.0).ceil() as usize;
        self.positions.get(k).copied()
    }

    fn mean(&self) -> Option<u32> {
        let l = self.positions.len();
        if l == 0 {
            None
        } else {
            let s = self.positions.iter().copied().sum::<u32>() as f64;
            Some((s / l as f64).ceil() as u32)
        }
    }

    fn simple_fuel_cost_for_alignment(&self, alignment: u32) -> u32 {
        self.positions
            .iter()
            .map(|position| match position.cmp(&alignment) {
                std::cmp::Ordering::Less => alignment - position,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => position - alignment,
            })
            .sum()
    }

    fn additive_fuel_cost_for_alignment(&self, alignment: u32) -> u32 {
        self.positions
            .iter()
            .map(|position| {
                let steps = match position.cmp(&alignment) {
                    std::cmp::Ordering::Less => alignment - position,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => position - alignment,
                };
                (steps * (steps + 1)) / 2
            })
            .sum()
    }
}

impl From<String> for Crabs {
    fn from(s: String) -> Self {
        let mut positions: Vec<u32> = s
            .split(',')
            .filter_map(|nstr| nstr.parse::<u32>().ok())
            .collect();

        positions.sort_unstable();

        Self { positions }
    }
}

#[cfg(test)]
mod tests;
