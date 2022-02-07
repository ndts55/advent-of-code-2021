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
mod tests {
    use std::{fs, str::FromStr};

    use super::{parse_input, solve_part_one, solve_part_two, Point, Vents};

    #[test]
    fn solve_part_two_small_input() {
        let expected = 12;
        let s = fs::read_to_string("small_input.txt").unwrap();
        let inp = parse_input(s.as_str()).unwrap();
        let actual = solve_part_two(&inp);
        assert_eq!(actual, expected);
    }

    #[test]
    fn solve_part_two_input() {
        let expected = 21140;
        let s = fs::read_to_string("input.txt").unwrap();
        let vents = parse_input(s.as_str()).unwrap();
        let actual = solve_part_two(&vents);
        assert_eq!(actual, expected);
    }

    #[test]
    fn solve_part_one_small_input() {
        let expected = 5;
        let s = fs::read_to_string("small_input.txt").unwrap();
        let inp = parse_input(s.as_str()).unwrap();
        let actual = solve_part_one(&inp);
        assert_eq!(actual, expected);
    }

    #[test]
    fn solve_part_one_input() {
        let expected = 7269;
        let s = fs::read_to_string("input.txt").unwrap();
        let vents = parse_input(s.as_str()).unwrap();
        let actual = solve_part_one(&vents);
        assert_eq!(actual, expected);
    }

    #[test]
    fn ventline_impl_iterator_works_diagonal() {
        let expected = vec![
            Point { x: 8, y: 0 },
            Point { x: 7, y: 1 },
            Point { x: 6, y: 2 },
            Point { x: 5, y: 3 },
            Point { x: 4, y: 4 },
            Point { x: 3, y: 5 },
            Point { x: 2, y: 6 },
            Point { x: 1, y: 7 },
            Point { x: 0, y: 8 },
        ];
        let actual: Vec<Point> = Vents::new(Point { x: 8, y: 0 }, Point { x: 0, y: 8 })
            .points()
            .into_iter()
            .collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn ventline_impl_iterator_works_horizontal() {
        let expected = vec![
            Point { x: 0, y: 9 },
            Point { x: 1, y: 9 },
            Point { x: 2, y: 9 },
            Point { x: 3, y: 9 },
            Point { x: 4, y: 9 },
            Point { x: 5, y: 9 },
        ];
        let actual: Vec<Point> = Vents::new(Point { x: 0, y: 9 }, Point { x: 5, y: 9 })
            .points()
            .into_iter()
            .collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn ventline_impl_iterator_works_vertical() {
        let expected = vec![
            Point { x: 4, y: 4 },
            Point { x: 4, y: 3 },
            Point { x: 4, y: 2 },
            Point { x: 4, y: 1 },
            Point { x: 4, y: 0 },
        ];
        let actual: Vec<Point> = Vents::new(Point { x: 4, y: 4 }, Point { x: 4, y: 0 })
            .points()
            .into_iter()
            .collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_point_success() {
        let expected = Point { x: 2, y: 4 };
        let actual_res = Point::from_str("2,4 ");
        assert!(actual_res.is_ok());
        let actual = actual_res.unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_point_failure() {
        for inp in vec!["asdf", "2,", ",4 ", "abc,2", "2,abc "] {
            assert!(Point::from_str(inp).is_err());
        }
    }

    #[test]
    fn parse_ventline_success() {
        let expected = Vents::new(Point { x: 0, y: 9 }, Point { x: 5, y: 9 });
        let actual_res = Vents::from_str("0,9 -> 5,9");
        assert!(actual_res.is_ok());
        let actual = actual_res.unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_ventline_failure() {
        for inp in vec!["0,9 to 5,9", "0,9 -> five,9", "9 -> 5,9", "blub"] {
            assert!(Vents::from_str(inp).is_err());
        }
    }

    #[test]
    fn parse_small_input_works() {
        let expected = vec![
            Vents::new(Point { x: 0, y: 9 }, Point { x: 5, y: 9 }),
            Vents::new(Point { x: 8, y: 0 }, Point { x: 0, y: 8 }),
            Vents::new(Point { x: 9, y: 4 }, Point { x: 3, y: 4 }),
            Vents::new(Point { x: 2, y: 2 }, Point { x: 2, y: 1 }),
            Vents::new(Point { x: 7, y: 0 }, Point { x: 7, y: 4 }),
            Vents::new(Point { x: 6, y: 4 }, Point { x: 2, y: 0 }),
            Vents::new(Point { x: 0, y: 9 }, Point { x: 2, y: 9 }),
            Vents::new(Point { x: 3, y: 4 }, Point { x: 1, y: 4 }),
            Vents::new(Point { x: 0, y: 0 }, Point { x: 8, y: 8 }),
            Vents::new(Point { x: 5, y: 5 }, Point { x: 8, y: 2 }),
        ];
        let s = fs::read_to_string("small_input.txt").unwrap();
        let actual_res = parse_input(s.as_str());
        assert!(actual_res.is_ok());
        let actual = actual_res.unwrap();
        assert_eq!(actual, expected);
    }
}
