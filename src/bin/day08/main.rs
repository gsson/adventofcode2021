use adventofcode2021::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = part1::solve(Input::from_file("src/bin/day08/input.txt"));
    eprintln!("Part 1: {}", a);
    assert_eq!(321, a);
    let a = part2::solve(Input::from_file("src/bin/day08/input.txt"));
    eprintln!("Part 2: {}", a);
    assert_eq!(1028926, a);
    Ok(())
}

fn parse<R: std::io::BufRead>(input: Input<R>) -> impl Iterator<Item = (Vec<String>, Vec<String>)> {
    fn words<R: std::io::BufRead>(input: Input<R>) -> Vec<String> {
        input.words().map(|w| w.into_string()).collect()
    }
    fn line<R: std::io::BufRead>(input: Input<R>) -> (Vec<String>, Vec<String>) {
        let (left, right) = input.delimited_once(" | ");
        (words(left), words(right))
    }

    input.lines().map(line)
}

mod part1 {
    use crate::parse;
    use adventofcode2021::*;

    fn count_known_lit(lit: &[String]) -> usize {
        lit.iter()
            .filter(|lit| matches!(lit.len(), 2 | 3 | 4 | 7))
            .count()
    }

    pub fn solve<R: std::io::BufRead>(input: Input<R>) -> usize {
        parse(input).map(|(_, lit)| count_known_lit(&lit)).sum()
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(26, solve(Input::from_readable(INPUT)));
    }
}

mod part2 {
    use crate::parse;
    use adventofcode2021::*;
    use std::ops::Index;

    fn normalise(s: &str) -> u8 {
        s.bytes().fold(0u8, |s, b| s | 1 << (b - b'a'))
    }

    pub fn digit(table: impl Index<usize, Output = u8>, digit: u8) -> i32 {
        *table.index(digit as usize) as i32
    }

    fn generate_table(combinations: &[String]) -> Option<[u8; 128]> {
        let by_number_of_segments = combinations
            .iter()
            .map(|s| normalise(s.as_str()))
            .group_by(|n| n.count_ones());

        let one = *by_number_of_segments.get(&2)?.first()?;
        let seven = *by_number_of_segments.get(&3)?.first()?;
        let four = *by_number_of_segments.get(&4)?.first()?;
        let eight = *by_number_of_segments.get(&7)?.first()?;

        let six = *by_number_of_segments
            .get(&6)?
            .iter() // Contains 0, 6 and 9
            .find(|n| **n | one != **n)?; // Segments of 6 is the only number that is not a superset of the segments of 1

        let five = *by_number_of_segments
            .get(&5)?
            .iter() // Contains 2, 3 and 5
            .find(|n| **n | six == six)?; // Segments of 5 is the only number that is a subset of the segments of 6

        let nine = five | one; // 9 is just the union of the segments of 1 and 5

        let zero = *by_number_of_segments
            .get(&6)?
            .iter() // Contains 0, 6 and 9
            .find(|n| **n != six && **n != nine)?; // 0

        let three = *by_number_of_segments
            .get(&5)?
            .iter() // Contains 2, 3 and 5
            .find(|n| **n != five && **n | nine == nine)?; // Excluding 5, the segments of 3 is a subset of the segments of 9

        let two = *by_number_of_segments
            .get(&5)?
            .iter() // Contains 2, 3 and 5
            .find(|n| **n != three && **n != five)?; // 2

        let mut table = [0xffu8; 128];
        table[zero as usize] = 0;
        table[one as usize] = 1;
        table[two as usize] = 2;
        table[three as usize] = 3;
        table[four as usize] = 4;
        table[five as usize] = 5;
        table[six as usize] = 6;
        table[seven as usize] = 7;
        table[eight as usize] = 8;
        table[nine as usize] = 9;

        Some(table)
    }

    pub fn solve<R: std::io::BufRead>(input: Input<R>) -> i32 {
        parse(input)
            .map(|(config, lit)| {
                let table = generate_table(&config).unwrap();
                lit.iter()
                    .map(|s| normalise(s))
                    .fold(0i32, |s, n| s * 10 + digit(table, n))
            })
            .sum::<i32>()
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(61229, solve(Input::from_readable(INPUT)));
    }
}
