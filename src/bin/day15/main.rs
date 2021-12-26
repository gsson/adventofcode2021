#![feature(array_windows)]

use adventofcode2021::delimiters::LINE;
use adventofcode2021::vector::{Dot, Vec2i};
use adventofcode2021::*;
use std::collections::BinaryHeap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Input::from_file("src/bin/day15/input.txt");
    let (width, height, risk) = parse(input);
    let a = part1::solve(width, height, &risk);
    eprintln!("Part 1: {}", a);
    assert_eq!(388, a);
    let a = part2::solve(width, height, &risk);
    eprintln!("Part 2: {}", a);
    assert_eq!(2819, a);
    Ok(())
}

#[allow(clippy::type_complexity)]
fn parse<R: std::io::BufRead>(input: Input<R>) -> (usize, usize, Vec<u8>) {
    let (first, rest) = input.delimited_once(LINE);
    let mut risk = first.bytes().map(|b| b - b'0').collect::<Vec<_>>();
    let width = risk.len();
    risk.extend(rest.lines().flat_map(|line| line.bytes().map(|b| b - b'0')));
    let height = risk.len() / width;
    (width, height, risk)
}

pub fn solve(width: usize, height: usize, risk: &[u8]) -> i32 {
    let mut cumulative_risk = vec![i32::MAX; risk.len()];
    let mut asdsf = BinaryHeap::new();
    let to_index = Vec2i(1, width as i32);
    let last = Vec2i((width - 1) as i32, (height - 1) as i32).dot(to_index) as usize;

    let mut threshold = 0;
    for x in 1..width {
        threshold += risk[x] as i32
    }
    for y in 1..height {
        threshold += risk[y * width + height - 1] as i32
    }
    cumulative_risk[last] = threshold;
    cumulative_risk[0] = 0;

    let width = width as i32;
    let height = height as i32;

    asdsf.push((-(risk[1] as i32), Vec2i::RIGHT));
    asdsf.push((-(risk[width as usize] as i32), Vec2i::DOWN));
    while let Some((r, vec)) = asdsf.pop() {
        let i = vec.dot(to_index) as usize;
        if -r >= cumulative_risk[i] || -r >= cumulative_risk[last] {
            continue;
        }
        cumulative_risk[i] = -r;

        if vec.0 > 0 {
            let next = vec + Vec2i::LEFT;
            let i = next.dot(to_index) as usize;
            let r = r - risk[i] as i32;
            if -r < cumulative_risk[i] && -r < cumulative_risk[last] {
                asdsf.push((r, next));
            }
        }
        if vec.1 > 0 {
            let next = vec + Vec2i::UP;
            let i = next.dot(to_index) as usize;
            let r = r - risk[i] as i32;
            if -r < cumulative_risk[i] && -r < cumulative_risk[last] {
                asdsf.push((r, next));
            }
        }
        if vec.0 < width - 1 {
            let next = vec + Vec2i::RIGHT;
            let i = next.dot(to_index) as usize;
            let r = r - risk[i] as i32;
            if -r < cumulative_risk[i] && -r < cumulative_risk[last] {
                asdsf.push((r, next));
            }
        }
        if vec.1 < height - 1 {
            let next = vec + Vec2i::DOWN;
            let i = next.dot(to_index) as usize;
            let r = r - risk[i] as i32;
            if -r < cumulative_risk[i] && -r < cumulative_risk[last] {
                asdsf.push((r, next));
            }
        }
    }
    cumulative_risk[last]
}

mod part1 {
    #[cfg(test)]
    use crate::*;

    pub fn solve(width: usize, height: usize, risk: &[u8]) -> i32 {
        super::solve(width, height, risk)
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        let (width, height, risk) = parse(Input::from_readable(INPUT));
        assert_eq!(40, solve(width, height, &risk));
    }
}

mod part2 {
    #[cfg(test)]
    use crate::*;

    pub fn solve(width: usize, height: usize, risk: &[u8]) -> i32 {
        let new_width = width * 5;
        let new_height = height * 5;
        let mut new_risk = Vec::with_capacity(new_width * new_height);
        for v in 0..5 {
            for y in 0..height {
                let row = &risk[y * width..(y + 1) * width];
                for u in 0..5 {
                    new_risk.extend(row.iter().map(|r| (r + u + v - 1) % 9 + 1));
                }
            }
        }
        super::solve(new_width, new_height, &new_risk)
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        let (width, height, risk) = parse(Input::from_readable(INPUT));
        assert_eq!(315, solve(width, height, &risk));
    }
}
