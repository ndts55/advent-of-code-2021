use std::collections::VecDeque;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let input = parse("input.txt")?;
    println!("Answer Part 1: {}", solve_part_1(&input));
    println!("Answer Part 2: {}", solve_part_2(&input));

    Ok(())
}

fn solve_part_1(depths: &Vec<i32>) -> i32 {
    depths
        .into_iter()
        .fold((None, 0), |acc, depth| {
            let (previous, count) = acc;
            if let Some(p) = previous {
                (Some(depth), count + (if p < depth { 1 } else { 0 }))
            } else {
                (Some(depth), count)
            }
        })
        .1
}

fn solve_part_2(depths: &Vec<i32>) -> i32 {
    solve_part_1(&trigram_sum(depths))
}

fn trigram_sum(data: &Vec<i32>) -> Vec<i32> {
    data.windows(3)
        .map(|window| window.into_iter().sum())
        .collect()
}

fn parse(path: &str) -> io::Result<Vec<i32>> {
    Ok(fs::read_to_string(path)?
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect::<Vec<i32>>())
}

#[cfg(test)]
mod tests {

    use super::{parse, solve_part_1, solve_part_2, trigram_sum};
    #[test]
    fn parse_test() {
        let expected = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let actual = parse("small_input.txt").unwrap();
        assert_eq!(expected, actual)
    }

    #[test]
    fn small_input_part_1() {
        let expected = 7;
        let actual = solve_part_1(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn small_input_part_2() {
        let expected = 5;
        let actual = solve_part_2(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn small_input_trigramsum() {
        let expected = vec![607, 618, 618, 617, 647, 716, 769, 792];

        let v: Vec<i32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        let actual = trigram_sum(&v);
        assert_eq!(expected, actual);
    }
}
