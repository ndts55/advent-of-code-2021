use std::{cmp::min, collections::HashSet, fs};

use itertools::Itertools;

fn main() {
    let matrix = Matrix::from(fs::read_to_string("input.txt").expect("No input file."));
    let part_one = solve_1(&matrix);
    println!("Part One: {part_one}");
    let part_two = solve_2(&matrix);
    println!("Part Two: {part_two}");
}

fn solve_1(matrix: &Matrix) -> u32 {
    matrix
        .low_points()
        .into_iter()
        .filter_map(|(x, y)| matrix.get(x, y).map(|n| n as u32 + 1))
        .sum()
}

fn solve_2(matrix: &Matrix) -> u32 {
    matrix
        .basins()
        .into_iter()
        .map(|b| b.len())
        .sorted_unstable_by(|a, b| Ord::cmp(a, b).reverse())
        .take(3)
        .product::<usize>() as u32
}

#[derive(Debug, PartialEq)]
struct Matrix {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Matrix {
    fn basins(&self) -> Vec<HashSet<(usize, usize)>> {
        // Calculate the basins for each low point.
        self.low_points()
            .into_iter()
            .map(|lp| self.basin_at(lp))
            .collect()
    }

    fn basin_at(&self, start: (usize, usize)) -> HashSet<(usize, usize)> {
        let mut visited = HashSet::new();
        let mut of_interest = vec![start];
        while let Some(oip @ (x, y)) = of_interest.pop() {
            of_interest.append(
                &mut surrounding_coordinates(self.width, self.height, x, y)
                    .into_iter()
                    .filter(|sp @ &(sx, sy)| {
                        !visited.contains(sp)
                            && !of_interest.contains(sp)
                            && self.get(sx, sy).filter(|v| *v != 9).is_some()
                    })
                    .collect(),
            );
            visited.insert(oip);
        }

        visited
    }

    fn low_points(&self) -> Vec<(usize, usize)> {
        (0..self.width)
            .cartesian_product(0..self.height)
            .filter(|(x, y)| self.is_low_point(*x, *y))
            .collect()
    }

    fn is_low_point(&self, x: usize, y: usize) -> bool {
        self.get(x, y)
            .filter(|n| *n != 9 && self.get_surrounding(x, y).iter().all(|s| *n < *s))
            .is_some()
    }

    fn get_surrounding(&self, x: usize, y: usize) -> Vec<u8> {
        surrounding_coordinates(self.width, self.height, x, y)
            .into_iter()
            .filter_map(|(dx, dy)| self.get(dx, dy))
            .collect()
    }

    fn get(&self, x: usize, y: usize) -> Option<u8> {
        let idx = x + self.width * y;
        self.data.get(idx).copied()
    }
}

fn surrounding_coordinates(
    width: usize,
    height: usize,
    x: usize,
    y: usize,
) -> HashSet<(usize, usize)> {
    let left_x = x.checked_sub(1).unwrap_or_default();
    let right_x = min(x + 1, width - 1);
    let up_y = y.checked_sub(1).unwrap_or_default();
    let down_y = min(y + 1, height - 1);
    let mut hs = HashSet::from([(left_x, y), (right_x, y), (x, up_y), (x, down_y)]);
    hs.remove(&(x, y));
    hs
}

impl From<Vec<Vec<u8>>> for Matrix {
    fn from(vv: Vec<Vec<u8>>) -> Self {
        let height = vv.len();
        let width = vv.get(0).map(|v| v.len()).unwrap_or_default();
        let data = vv.into_iter().flatten().collect();

        Self {
            data,
            width,
            height,
        }
    }
}

impl From<&str> for Matrix {
    fn from(s: &str) -> Self {
        let vv: Vec<Vec<u8>> = s
            .lines()
            .map(|line| {
                line.chars()
                    .filter_map(|ch| ch.to_digit(10))
                    .map(|n| n as u8)
                    .collect::<Vec<u8>>()
            })
            .collect();

        Self::from(vv)
    }
}

impl From<String> for Matrix {
    fn from(s: String) -> Self {
        Self::from(s.as_str())
    }
}

#[cfg(test)]
mod tests;
