#![feature(array_windows)]

use adventofcode2021::delimiters::SECTION;
use adventofcode2021::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Input::from_file("src/bin/day14/input.txt");
    let (template, rules) = parse(input);
    let a = part1::solve(&template, &rules);
    eprintln!("Part 1: {}", a);
    assert_eq!(2345, a);
    let a = part2::solve(&template, &rules);
    eprintln!("Part 2: {}", a);
    assert_eq!(2432786807053, a);
    Ok(())
}

#[derive(Copy, Clone)]
struct Pair(u8);
impl Pair {
    #[inline]
    fn encode(a: u8, b: u8) -> Self {
        Self(a << 4 | b)
    }
    #[inline]
    fn second_index(self) -> usize {
        (self.0 & 0xf) as usize
    }
    #[inline]
    fn index(self) -> usize {
        self.0 as usize
    }
}

#[allow(clippy::type_complexity)]
fn parse<R: std::io::BufRead>(input: Input<R>) -> (Vec<u8>, Vec<([u8; 2], u8)>) {
    let (template, rules) = input.delimited_once(SECTION);
    let template = template.into_bytes();
    let rules = rules
        .lines()
        .map(|line| {
            let (from, to) = line.delimited_once(" -> ");
            (from.into_byte_array::<2>(), to.into_byte_array::<1>()[0])
        })
        .collect();
    (template, rules)
}

struct ElementLookup([u8; 32]);
impl ElementLookup {
    fn new(template: &[u8], rules: &[([u8; 2], u8)]) -> ElementLookup {
        let mut elements = rules
            .iter()
            .flat_map(|(k, v)| k.iter().chain(std::iter::once(v)))
            .chain(template.iter())
            .copied()
            .collect::<Vec<_>>();
        elements.sort_unstable();
        elements.dedup();
        let mut table = [0xffu8; 32];
        for (i, c) in elements.iter().enumerate() {
            table[(*c - b'A') as usize] = i as u8;
        }
        ElementLookup(table)
    }

    fn encode_pair(&self, a: u8, b: u8) -> Pair {
        let a = self.0[(a - b'A') as usize];
        let b = self.0[(b - b'A') as usize];
        Pair::encode(a, b)
    }
    fn encode(&self, a: u8) -> u8 {
        self.0[(a - b'A') as usize]
    }
}

fn compile_rules(
    element_ids: &ElementLookup,
    rules: &[([u8; 2], u8)],
) -> [Option<(Pair, Pair)>; 256] {
    let mut rules_table = [None; 256];
    for ([a, b], insert) in rules {
        let when = element_ids.encode_pair(*a, *b);
        let replace = (
            element_ids.encode_pair(*a, *insert),
            element_ids.encode_pair(*insert, *b),
        );
        rules_table[when.index()] = Some(replace);
    }
    rules_table
}

pub fn solve(iterations: usize, template: &[u8], rules: &[([u8; 2], u8)]) -> usize {
    let element_lookup = ElementLookup::new(template, rules);
    let rules = compile_rules(&element_lookup, rules);

    let mut elements = [0i64; 16];
    for element in template {
        let element = element_lookup.encode(*element);
        elements[element as usize] += 1;
    }

    let mut prev = [0i64; 256];
    let mut next = [0i64; 256];

    for [a, b] in template.array_windows::<2>() {
        let pair = element_lookup.encode_pair(*a, *b);
        prev[pair.index()] += 1;
    }

    for _ in 0..iterations {
        for pair_index in 0..256 {
            let count = prev[pair_index];
            if count > 0 {
                if let Some((pair1, pair2)) = &rules[pair_index] {
                    elements[pair1.second_index()] += count;
                    next[pair1.index()] += count;
                    next[pair2.index()] += count;
                } else {
                    next[pair_index] += count;
                }
            }
        }
        prev.swap_with_slice(&mut next);
        next.fill(0);
    }

    let min = elements.iter().filter(|n| **n > 0).min().unwrap();
    let max = elements.iter().filter(|n| **n > 0).max().unwrap();

    (max - min) as usize
}

mod part1 {
    #[cfg(test)]
    use crate::*;

    pub fn solve(template: &[u8], rules: &[([u8; 2], u8)]) -> usize {
        super::solve(10, template, rules)
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        let (template, rules) = parse(Input::from_readable(INPUT));
        assert_eq!(1588, solve(&template, &rules));
    }
}

mod part2 {
    #[cfg(test)]
    use crate::*;

    pub fn solve(template: &[u8], rules: &[([u8; 2], u8)]) -> usize {
        super::solve(40, template, rules)
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        let (template, rules) = parse(Input::from_readable(INPUT));
        assert_eq!(2188189693529, solve(&template, &rules));
    }
}
