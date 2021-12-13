use adventofcode2021::*;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = part1::solve(Input::from_file("src/bin/day09/input.txt"));
    eprintln!("Part 1: {}", a);
    assert_eq!(633, a);
    let a = part2::solve(Input::from_file("src/bin/day09/input.txt"));
    eprintln!("Part 2: {}", a);
    assert_eq!(1050192, a);
    Ok(())
}

fn parse<R: std::io::BufRead>(input: Input<R>) -> (usize, usize, Vec<u8>) {
    let mut map = Vec::new();
    let (first, remaining) = input.delimited_once(delimiters::LINE);
    let lines = remaining.lines();

    let first = first.bytes().collect::<Vec<_>>();
    let row_width = first.len() + 2;
    map.extend(std::iter::repeat(9).take(row_width));

    map.push(9);
    map.extend(first.iter().map(|b| b - b'0'));
    map.push(9);

    for line in lines {
        map.push(9);
        map.extend(line.bytes().map(|b| b - b'0'));
        map.push(9);
    }
    map.extend(std::iter::repeat(9).take(row_width));
    (row_width, map.len() / row_width, map)
}

fn is_basin_low(offset: usize, width: usize, map: &[u8]) -> bool {
    let offsets: [usize; 4] = [width.twos_complement(), 1usize.twos_complement(), 1, width];
    let center = map[offset];
    offsets
        .into_iter()
        .map(|o| offset.wrapping_add(o))
        .all(|position| map[position] > center)
}

fn basin_size(basin_low: usize, width: usize, map: &[u8]) -> usize {
    let offsets: [usize; 4] = [width.twos_complement(), 1usize.twos_complement(), 1, width];
    let mut tested = HashSet::new();
    let mut to_test = vec![basin_low];
    let mut size = 0;
    while let Some(offset) = to_test.pop() {
        for o in offsets {
            let position = offset.wrapping_add(o);
            if tested.insert(position) && map[position] < 9 {
                // This position belongs to the basin
                size += 1;
                to_test.push(position)
            }
        }
    }
    size
}

mod part1 {
    use crate::*;

    pub fn solve<R: std::io::BufRead>(input: Input<R>) -> usize {
        let (width, height, map) = parse(input);

        (width + 1..((height - 1) * width - 1))
            .filter_map(|offset| {
                is_basin_low(offset, width, &map).then(|| map[offset] as usize + 1)
            })
            .sum::<usize>()
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(15, solve(Input::from_readable(INPUT)));
    }
}

mod part2 {
    use crate::*;

    pub fn solve<R: std::io::BufRead>(input: Input<R>) -> usize {
        let (width, height, map) = parse(input);
        let mut image = Vec::with_capacity((width - 2) * (height - 2) * 3);
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let depth = map[y * width + x];
                if depth < 9 {
                    image.push(0x10);
                    image.push(0x10);
                    image.push(0xff - depth * 10);
                } else {
                    image.push(0x80);
                    image.push(0x80);
                    image.push(0x80);
                }
            }
        }

        let mut basin_sizes = (width + 1..((height - 1) * width - 1))
            .filter_map(|offset| is_basin_low(offset, width, &map).then(|| offset))
            .map(|basin_low| basin_size(basin_low, width, &map))
            .collect::<Vec<_>>();
        basin_sizes.sort_unstable();
        basin_sizes.into_iter().rev().take(3).product::<usize>()
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(1134, solve(Input::from_readable(INPUT)));
    }
}
