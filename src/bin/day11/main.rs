#![feature(array_zip)]

use adventofcode2021::delimiters::LINE;
use adventofcode2021::*;
use std::iter::{once, repeat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = part1::solve(Input::from_file("src/bin/day11/input.txt"));
    eprintln!("Part 1: {}", a);
    assert_eq!(1679, a);
    let a = part2::solve(Input::from_file("src/bin/day11/input.txt"));
    eprintln!("Part 2: {}", a);
    assert_eq!(519, a);
    Ok(())
}

#[derive(Copy, Clone)]
struct Grid {
    grid: [u8; 12 * 12],
}

impl Grid {
    const SURROUNDING: [usize; 8] = [
        !(12 + 1) + 1,
        !12 + 1,
        !(12 - 1) + 1,
        !1 + 1,
        1,
        12 - 1,
        12,
        12 + 1,
    ];
    const MASK: [u8; 12 * 12] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00,
        0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00,
        0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    fn new_from_slice(from: &[u8]) -> Self {
        let mut grid = [0u8; 12 * 12];
        grid.copy_from_slice(from);
        Self { grid }
    }

    fn increase(grid: [u8; 12 * 12]) -> [u8; 12 * 12] {
        grid.zip(Self::MASK).map(|(value, mask)| (value + 1) & mask)
    }

    fn reset(grid: [u8; 12 * 12]) -> [u8; 12 * 12] {
        grid.map(|value| if value > 9 { 0 } else { value })
    }

    fn spread(mut grid: [u8; 12 * 12]) -> (usize, [u8; 12 * 12]) {
        let mut remaining = Vec::from_iter(
            grid.iter()
                .enumerate()
                .filter_map(|(i, v)| (*v > 9).then(|| i)),
        );
        let mut flashed = remaining.len();
        while let Some(index) = remaining.pop() {
            for offset in Self::SURROUNDING {
                let i = index.wrapping_add(offset);
                grid[i] = match grid[i] {
                    9 => {
                        flashed += 1;
                        remaining.push(i);
                        10
                    }
                    v => v + 1,
                }
            }
        }
        (flashed, grid)
    }
    fn step(self) -> (usize, Self) {
        let new_grid = Self::increase(self.grid);
        let (flashed, new_grid) = Self::spread(new_grid);
        (
            flashed,
            Self {
                grid: Self::reset(new_grid),
            },
        )
    }
}

fn parse<R: std::io::BufRead>(input: Input<R>) -> (usize, usize, Vec<u8>) {
    fn parse_row<R: std::io::BufRead>(input: Input<R>) -> impl Iterator<Item = u8> {
        once(0)
            .chain(input.bytes().map(|b| (b - b'0')))
            .chain(once(0))
    }
    let (first, remaining) = input.delimited_once(LINE);
    let mut first = parse_row(first).collect::<Vec<_>>();
    let width = first.len();
    let mut grid = Vec::from_iter(repeat(0).take(width));
    grid.append(&mut first);
    grid.extend(remaining.lines().flat_map(parse_row));
    grid.extend(repeat(0).take(width));
    let height = grid.len() / width;
    (width, height, grid)
}

mod part1 {
    use crate::*;

    pub fn solve<R: std::io::BufRead>(input: Input<R>) -> usize {
        let (_width, _height, grid) = parse(input);
        let grid = Grid::new_from_slice(&grid);
        let (flashes, _) = (0..100).fold((0, grid), |(flashes, grid), _| {
            let (f, grid) = grid.step();
            (flashes + f, grid)
        });
        flashes
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(1656, solve(Input::from_readable(INPUT)));
    }
}

mod part2 {
    use crate::*;
    use std::ops::ControlFlow::{Break, Continue};

    pub fn solve<R: std::io::BufRead>(input: Input<R>) -> usize {
        let (_width, _height, grid) = parse(input);
        let grid = Grid::new_from_slice(&grid);
        let result = (1..).try_fold(grid, |grid, i| {
            let (flashed, grid) = grid.step();
            if flashed == 100 {
                Break(i)
            } else {
                Continue(grid)
            }
        });
        match result {
            Break(i) => i,
            _ => unreachable!(),
        }
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(195, solve(Input::from_readable(INPUT)));
    }
}
