use std::{
    collections::{BTreeMap, HashMap},
    fs,
};

static SPLIT_PATTERN: &str = " -> ";

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    println!(
        "Part 1: {}",
        counting_solution(content.as_str(), 10).unwrap_or_default()
    );
    println!(
        "Part 2: {}",
        counting_solution(content.as_str(), 40).unwrap_or_default()
    );
    println!(
        "Crazy:  {}",
        counting_solution(content.as_str(), 125).unwrap_or_default()
    );
}

fn counting_solution(content: &str, iterations: usize) -> Option<u128> {
    let (current, rules) = parse_content(&content)?;
    let letter_counts = apply_rules(bigram_counts(&current), rules, iterations);
    Some(min_max_delta(&letter_counts))
}

fn apply_rules(
    bigram_counts: BTreeMap<(char, char), u128>,
    rules: HashMap<(char, char), char>,
    n: usize,
) -> BTreeMap<char, u128> {
    let mut bigram_counts = bigram_counts;
    for _ in 0..n {
        let new_bigram_counts = bigram_counts
            .iter()
            .fold(BTreeMap::new(), |mut acc, (k, &v)| {
                if let Some(&c) = rules.get(k) {
                    *(acc.entry((k.0, c)).or_default()) += v;
                    *(acc.entry((c, k.1)).or_default()) += v;
                } else {
                    *(acc.entry(*k).or_default()) += v;
                }
                acc
            });
        bigram_counts = new_bigram_counts;
    }

    let mut letter_counts = bigram_counts.iter().fold(
        BTreeMap::new(),
        |mut acc, ((f, s), &v)| {
            *(acc.entry(*f).or_default()) += v;
            *(acc.entry(*s).or_default()) += v;
            acc
        },
    );
    for (_, v) in letter_counts.iter_mut() {
        *v = (*v / 2) + (*v % 2);
    }

    letter_counts
}

fn bigram_counts(s: &str) -> BTreeMap<(char, char), u128> {
    let mut btree = BTreeMap::new();
    let s = s.chars().collect::<Vec<char>>();
    let s = s.as_slice();
    for n in 0..s.len() - 1 {
        let k = (s[n], s[n + 1]);
        let old_value = btree.entry(k).or_default();
        *old_value += 1;
    }

    btree
}

fn parse_content(content: &str) -> Option<(&str, HashMap<(char, char), char>)> {
    let mut lines = content.lines();
    let s = lines.next()?;
    let rules = lines.filter_map(parse_line).collect();
    Some((s, rules))
}

fn parse_line(line: &str) -> Option<((char, char), char)> {
    let split: Vec<&str> = line.split(SPLIT_PATTERN).collect();
    if split.len() != 2 {
        None
    } else {
        let cs = split[0].chars().collect::<Vec<_>>();
        let k = cs.get(0).zip(cs.get(1)).map(|(&f, &s)| (f, s))?;
        Some((k, split[1].chars().collect::<Vec<_>>()[0]))
    }
}

fn min_max_delta(btree: &BTreeMap<char, u128>) -> u128 {
    btree
        .values()
        .min()
        .zip(btree.values().max())
        .map(|(min, max)| max - min)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use std::{
        collections::{BTreeMap, HashMap},
        fs,
    };

    use super::{
        apply_rules, bigram_counts, counting_solution, min_max_delta, parse_content, parse_line,
    };
    #[test]
    fn parse_line_returns_none() {
        let actual = parse_line("AB => C");
        assert_eq!(actual, None);
        let actual = parse_line("AB: C");
        assert_eq!(actual, None);
        let actual = parse_line("AB C");
        assert_eq!(actual, None);
        let actual = parse_line("\n");
        assert_eq!(actual, None);
    }

    #[test]
    fn parse_line_returns_some() {
        let lines = vec![
            "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B", "HN -> C", "NN -> C",
            "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B", "CC -> N", "CN -> C",
        ];
        let actual: Vec<_> = lines.into_iter().map(parse_line).collect();
        let expected = vec![
            Some((('C', 'H'), 'B')),
            Some((('H', 'H'), 'N')),
            Some((('C', 'B'), 'H')),
            Some((('N', 'H'), 'C')),
            Some((('H', 'B'), 'C')),
            Some((('H', 'C'), 'B')),
            Some((('H', 'N'), 'C')),
            Some((('N', 'N'), 'C')),
            Some((('B', 'H'), 'H')),
            Some((('N', 'C'), 'B')),
            Some((('N', 'B'), 'B')),
            Some((('B', 'N'), 'B')),
            Some((('B', 'B'), 'N')),
            Some((('B', 'C'), 'B')),
            Some((('C', 'C'), 'N')),
            Some((('C', 'N'), 'C')),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_content_small_input() {
        let content = fs::read_to_string("small_input.txt").unwrap();
        let actual = parse_content(content.as_str());
        let expected = Some((
            "NNCB",
            HashMap::from([
                (('C', 'H'), 'B'),
                (('H', 'H'), 'N'),
                (('C', 'B'), 'H'),
                (('N', 'H'), 'C'),
                (('H', 'B'), 'C'),
                (('H', 'C'), 'B'),
                (('H', 'N'), 'C'),
                (('N', 'N'), 'C'),
                (('B', 'H'), 'H'),
                (('N', 'C'), 'B'),
                (('N', 'B'), 'B'),
                (('B', 'N'), 'B'),
                (('B', 'B'), 'N'),
                (('B', 'C'), 'B'),
                (('C', 'C'), 'N'),
                (('C', 'N'), 'C'),
            ]),
        ));
        assert_eq!(actual, expected);
    }

    #[test]
    fn bigram_counts_works() {
        let input = "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB";
        let actual = bigram_counts(input);
        let expected = BTreeMap::from([
            (('N', 'B'), 9),
            (('B', 'B'), 9),
            (('B', 'N'), 6),
            (('B', 'C'), 4),
            (('C', 'C'), 2),
            (('C', 'N'), 3),
            (('N', 'C'), 1),
            (('C', 'B'), 5),
            (('B', 'H'), 3),
            (('H', 'C'), 3),
            (('H', 'H'), 1),
            (('H', 'N'), 1),
            (('N', 'H'), 1),
        ]);

        assert_eq!(actual, expected);
    }

    #[test]
    fn apply_rules_works() {
        let initial = BTreeMap::from([
            // NNCB
            (('N', 'N'), 1),
            (('N', 'C'), 1),
            (('C', 'B'), 1),
        ]);
        let rules = HashMap::from([
            (('C', 'H'), 'B'),
            (('H', 'H'), 'N'),
            (('C', 'B'), 'H'),
            (('N', 'H'), 'C'),
            (('H', 'B'), 'C'),
            (('H', 'C'), 'B'),
            (('H', 'N'), 'C'),
            (('N', 'N'), 'C'),
            (('B', 'H'), 'H'),
            (('N', 'C'), 'B'),
            (('N', 'B'), 'B'),
            (('B', 'N'), 'B'),
            (('B', 'B'), 'N'),
            (('B', 'C'), 'B'),
            (('C', 'C'), 'N'),
            (('C', 'N'), 'C'),
        ]);
        let actual = apply_rules(initial, rules, 4);
        // let expected_bigram_counts = BTreeMap::from([
        //     (('N', 'B'), 9),
        //     (('B', 'B'), 9),
        //     (('B', 'N'), 6),
        //     (('B', 'C'), 4),
        //     (('C', 'C'), 2),
        //     (('C', 'N'), 3),
        //     (('N', 'C'), 1),
        //     (('C', 'B'), 5),
        //     (('B', 'H'), 3),
        //     (('H', 'C'), 3),
        //     (('H', 'H'), 1),
        //     (('H', 'N'), 1),
        //     (('N', 'H'), 1),
        // ]);
        let expected_letter_counts = BTreeMap::from([('N', 11), ('B', 23), ('C', 10), ('H', 5)]);
        // let expected = (expected_bigram_counts, expected_letter_counts);
        assert_eq!(actual, expected_letter_counts);
    }

    #[test]
    fn min_max_delta_works() {
        let input = BTreeMap::from([('N', 11), ('B', 23), ('C', 10), ('H', 5)]);
        let actual = min_max_delta(&input);
        let expected = 23 - 5;
        assert_eq!(actual, expected);
    }

    #[test]
    fn small_input() {
        let content = fs::read_to_string("small_input.txt").unwrap();
        let actual = counting_solution(content.as_str(), 4);
        let expected = Some(23 - 5);
        assert_eq!(expected, actual);
    }
}
