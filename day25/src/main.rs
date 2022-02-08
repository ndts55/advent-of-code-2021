use std::fs;

fn main() {
    let s = fs::read_to_string("input.txt").unwrap();
    let mut herds = Herds::from(s.as_str());
    let part_one = herd_deadlock(&mut herds);
    println!("Part One: {part_one}");
}

fn herd_deadlock(herds: &mut Herds) -> u32 {
    let mut previous_positions = herds.next();
    let mut current_positions = herds.next();
    let mut counter = 1;
    // Iterate while counting until previous and current are the same.
    while previous_positions != current_positions {
        counter += 1;
        previous_positions = current_positions;
        current_positions = herds.next();
    }

    counter
}

#[derive(Debug, PartialEq)]
struct CucumberPositions {
    east: Vec<(u8, u8)>,
    south: Vec<(u8, u8)>,
}

impl From<&str> for CucumberPositions {
    fn from(s: &str) -> Self {
        let (east, south) = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.char_indices()
                    .map(|(x, ch)| (ch, (x as u8, y as u8)))
                    .collect::<Vec<(char, (u8, u8))>>()
            })
            .fold(
                (Vec::new(), Vec::new()),
                |(mut eacc, mut sacc), (ch, pos)| {
                    match ch {
                        '>' => {
                            eacc.push(pos);
                        }
                        'v' => {
                            sacc.push(pos);
                        }
                        _ => {}
                    }

                    (eacc, sacc)
                },
            );

        Self { east, south }
    }
}

#[derive(Debug, PartialEq)]
struct Herds {
    cucumbers: CucumberPositions,
    max_x: u8,
    max_y: u8,
}

impl Iterator for Herds {
    type Item = CucumberPositions;

    fn next(&mut self) -> Option<Self::Item> {
        let mut east = self
            .cucumbers
            .east
            .iter()
            .map(|p @ (x, y)| {
                // Calculate desired destination.
                let dst = (if *x == self.max_x { 0 } else { x + 1 }, *y);
                // Check if cucumber can move there.
                if self.cucumbers.east.contains(&dst) || self.cucumbers.south.contains(&dst) {
                    *p
                } else {
                    // Move cucumber, if possible.
                    dst
                }
            })
            .collect();

        unsafe {
            std::ptr::swap(&mut self.cucumbers.east, &mut east);
        }

        let mut south = self
            .cucumbers
            .south
            .iter()
            .map(|p @ (x, y)| {
                // Calculate desired destinatioin.
                let dst = (*x, if *y == self.max_y { 0 } else { y + 1 });
                // Check if cucumber can move there.
                if self.cucumbers.east.contains(&dst) || self.cucumbers.south.contains(&dst) {
                    *p
                } else {
                    // Move cucumber, if possible.
                    dst
                }
            })
            .collect();

        unsafe {
            std::ptr::swap(&mut self.cucumbers.south, &mut south);
        }

        Some(CucumberPositions { east, south })
    }
}

impl From<&str> for Herds {
    fn from(s: &str) -> Self {
        let max_y = (s.lines().count() - 1) as u8;
        let max_x = (s
            .lines()
            .nth(0)
            .map(|line| line.chars().count())
            .unwrap_or_default()
            - 1) as u8;
        let cucumbers = CucumberPositions::from(s);

        Self {
            cucumbers,
            max_x,
            max_y,
        }
    }
}

#[cfg(test)]
mod tests;
