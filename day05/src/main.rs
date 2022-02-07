use std::{collections::HashMap, fs, str::FromStr};

fn main() {
    let s = fs::read_to_string("input.txt").unwrap();
    let vents = parse_input(s.as_str()).unwrap();
    let part_one = solve_part_one(&vents);
    println!("Part One: {part_one}");
    let part_two = solve_part_two(&vents);
    println!("Part Two: {part_two}");
}

fn solve_part_one(vents: &Vec<Vents>) -> usize {
    vents
        .iter()
        .filter(|&vl| vl.start.x == vl.end.x || vl.start.y == vl.end.y)
        .fold(HashMap::new(), |mut hm: HashMap<Point, u32>, vl| {
            for point in vl.points() {
                *(hm.entry(point)).or_default() += 1;
            }

            hm
        })
        .into_iter()
        .filter(|(_, c)| *c > 1)
        .count()
}

fn solve_part_two(vents: &Vec<Vents>) -> usize {
    vents
        .iter()
        .fold(HashMap::new(), |mut hm: HashMap<Point, u32>, vl| {
            for point in vl.points() {
                *(hm.entry(point)).or_default() += 1;
            }

            hm
        })
        .into_iter()
        .filter(|(_, c)| *c > 1)
        .count()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ns: Vec<&str> = s.trim().split(',').collect();
        let x = ns
            .get(0)
            .ok_or("Missing first number.")?
            .parse::<i32>()
            .map_err(|_| "Unable to parse first number.")?;
        let y = ns
            .get(1)
            .ok_or("Missing second number")?
            .parse::<i32>()
            .map_err(|_| "Unable to parse second number.")?;

        Ok(Self { x, y })
    }
}

struct VentLine {
    start: Point,
    end: Point,
    current: Option<Point>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Vents {
    start: Point,
    end: Point,
}

impl Vents {
    fn new(start: Point, end: Point) -> Self {
        Vents { start, end }
    }

    fn points(&self) -> VentLine {
        VentLine {
            start: self.start,
            end: self.end,
            current: Some(self.start),
        }
    }
}

impl Iterator for VentLine {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        // If self.current is `None` we stop iterating.
        let current = self.current?;

        let next = Point {
            x: current.x
                + match self.start.x.cmp(&self.end.x) {
                    std::cmp::Ordering::Less => 1,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => -1,
                },
            y: current.y
                + match self.start.y.cmp(&self.end.y) {
                    std::cmp::Ordering::Less => 1,
                    std::cmp::Ordering::Equal => 0,
                    std::cmp::Ordering::Greater => -1,
                },
        };

        if current == self.end {
            self.current = None;
        } else {
            self.current = Some(next);
        }

        Some(current)
    }
}

impl FromStr for Vents {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ps: Vec<&str> = s.trim().split("->").collect();
        let start = Point::from_str(ps.get(0).ok_or("Missing start point.")?)?;
        let end = Point::from_str(ps.get(1).ok_or("Missing end point")?)?;
        Ok(Self::new(start, end))
    }
}

fn parse_input(s: &str) -> Result<Vec<Vents>, <Vents as FromStr>::Err> {
    s.lines()
        .map(Vents::from_str)
        .collect::<Result<Vec<Vents>, <Vents as FromStr>::Err>>()
}

#[cfg(test)]
mod tests;
