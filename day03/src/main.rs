use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", solve_1(content.as_str()).unwrap_or_default());
    println!("Part 2: {}", solve_2(content.as_str()).unwrap_or_default());
}

fn solve_1(s: &str) -> Option<u128> {
    let lines: Vec<&str> = s.lines().collect();
    let height = lines.len();
    let min_count_for_majority = (height / 2 + height % 2) as u32;
    let width = lines.get(0)?.len();
    let (gamma, epsilon) = lines
        .into_iter()
        .fold(vec![0u32; width], |mut acc, line| {
            for (idx, ch) in line.char_indices() {
                if let Some(v) = acc.get_mut(idx) {
                    *v += (ch == '1') as u32;
                }
            }

            acc
        })
        .into_iter()
        .map(|one_counts| one_counts > min_count_for_majority)
        .fold((0, 0), |(gamma, epsilon), bit| {
            ((gamma * 2) + (bit as u128), (epsilon * 2) + (!bit as u128))
        });

    Some(gamma * epsilon)
}

fn solve_2(s: &str) -> Option<u32> {
    let lines: Vec<&str> = s.lines().collect();
    let width = lines.get(0)?.len();
    // Construct
    let mut oxygen = lines.clone();
    let mut co2 = lines;
    let mut n = 0;
    loop {
        if n >= width || (co2.len() <= 1 && oxygen.len() <= 1) {
            break;
        }

        if oxygen.len() > 1 {
            // Get most common.
            let most_common = most_common_at(&oxygen, n).unwrap_or('1');
            // Filter lines that don't fit.
            oxygen = oxygen
                .into_iter()
                .filter(equal_to_at_fn(most_common, n))
                .collect();
        }

        if co2.len() > 1 {
            // Get least common.
            let least_common = least_common_at(&co2, n).unwrap_or('0');
            // Filter lines that don't fit.
            co2 = co2
                .into_iter()
                .filter(equal_to_at_fn(least_common, n))
                .collect();
        }

        n += 1;
    }

    oxygen
        .get(0)
        .and_then(|line| u32::from_str_radix(line, 2).ok())
        .zip(
            co2.get(0)
                .and_then(|line| u32::from_str_radix(line, 2).ok()),
        )
        .map(|(oxygen_rating, co2_rating)| oxygen_rating * co2_rating)
}

fn equal_to_at_fn(ch: char, n: usize) -> impl FnMut(&&str) -> bool {
    move |line: &&str| line.chars().nth(n).map(|c| c == ch).unwrap_or(false)
}

fn most_common_at(data: &Vec<&str>, column: usize) -> Option<char> {
    let height = data.len();
    let ones = data
        .into_iter()
        .filter_map(|line| line.chars().nth(column))
        .map(|ch| ch == '1')
        .filter(|b| *b)
        .count();

    let majority = height / 2 + height % 2;

    if ones == majority {
        None
    } else {
        Some(if ones > majority { '1' } else { '0' })
    }
}

fn least_common_at(data: &Vec<&str>, column: usize) -> Option<char> {
    let most_common = most_common_at(data, column)?;
    Some(match most_common {
        '1' => '0',
        '0' => '1',
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use bitvec::{bitvec, order::Lsb0};

    use super::{least_common_at, most_common_at, solve_1, solve_2};
    use std::fs;

    #[test]
    fn bitvec_works() {
        let actual = bitvec![
            0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1,
            1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0,
            1, 0
        ];
        let expected = "[001001111010110101111010101111001111110010000110010001001010]";
        assert_eq!(format!("{:b}", actual), expected);
    }

    #[test]
    fn solve_1_works() {
        let content = fs::read_to_string("small_input.txt").unwrap();
        let actual = solve_1(content.as_str()).unwrap();
        let expected = 198;
        assert_eq!(actual, expected);
    }

    #[test]
    fn most_common_at_works() {
        let input = vec!["010", "010", "101", "011"];
        let actual = most_common_at(&input, 0);
        assert_eq!(actual, Some('0'));
        let actual = most_common_at(&input, 1);
        assert_eq!(actual, Some('1'));
        let actual = most_common_at(&input, 2);
        assert_eq!(actual, None);
    }

    #[test]
    fn least_common_at_works() {
        let input = vec!["010", "010", "101", "011"];
        let actual = least_common_at(&input, 0);
        assert_eq!(actual, Some('1'));
        let actual = least_common_at(&input, 1);
        assert_eq!(actual, Some('0'));
        let actual = least_common_at(&input, 2);
        assert_eq!(actual, None);
    }

    #[test]
    fn solve_2_works() {
        let content = fs::read_to_string("small_input.txt").unwrap();
        let actual = solve_2(content.as_str()).unwrap();
        let expected = 230;
        assert_eq!(actual, expected);
    }
}
