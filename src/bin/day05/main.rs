use adventofcode2021::*;
use std::cmp::max;
use std::io::BufRead;
use std::ops::{Add, Mul, Sub};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = part1::solve(Input::from_file("src/bin/day05/input.txt"));
    eprintln!("Part 1: {}", a);
    assert_eq!(7085, a);
    let a = part2::solve(Input::from_file("src/bin/day05/input.txt"));
    eprintln!("Part 2: {}", a);
    assert_eq!(20271, a);
    Ok(())
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl FromInput for Point {
    fn from_input<R: BufRead>(input: Input<R>) -> Self {
        let (x, y) = input.delimited_once(",");
        Self {
            x: x.parse(),
            y: y.parse()
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn is_orthogonal(&self) -> bool {
        let Point { x: dx, y: dy } = self.end - self.start;
        dx == 0 || dy == 0
    }

    pub fn points(&self) -> LinePointsIter {
        let Point { x: dx, y: dy } = self.end - self.start;
        let length = max(dx.abs(), dy.abs());
        let slope = Point {
            x: dx.signum(),
            y: dy.signum(),
        };
        LinePointsIter {
            start: self.start,
            slope,
            length,
            i: 0,
        }
    }
}

impl FromInput for Line {
    fn from_input<R: BufRead>(input: Input<R>) -> Self {
        let (start, end) = input.delimited_once(" -> ");
        Self {
            start: start.parse(),
            end: end.parse(),
        }
    }
}

struct LinePointsIter {
    start: Point,
    slope: Point,
    length: i32,
    i: i32,
}

impl Iterator for LinePointsIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i > self.length {
            None
        } else {
            let point = self.start + self.slope * self.i;
            self.i += 1;
            Some(point)
        }
    }
}

fn parse<R: std::io::BufRead>(input: Input<R>) -> impl Iterator<Item = Line> {
    input.lines().parse()
}

mod part1 {
    use crate::parse;
    use adventofcode2021::*;
    use std::collections::HashSet;

    pub fn solve<R: std::io::BufRead>(input: Input<R>) -> usize {
        let (_, overlaps) = parse(input)
            .filter(|l| l.is_orthogonal())
            .flat_map(move |l| l.points())
            .fold(
                (HashSet::new(), HashSet::new()),
                |(mut all, mut overlaps), p| {
                    if !all.insert(p) {
                        overlaps.insert(p);
                    }
                    (all, overlaps)
                },
            );
        overlaps.len()
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(5, solve(Input::from_readable(INPUT)));
    }
}

mod part2 {
    use crate::parse;
    use adventofcode2021::*;
    use std::collections::HashMap;

    pub fn solve<R: std::io::BufRead>(input: Input<R>) -> usize {
        let points = parse(input).flat_map(|l| l.points()).fold(
            HashMap::<_, i32>::new(),
            |mut points, p| {
                *points.entry(p).or_default() += 1;
                points
            },
        );
        points.values().filter(|v| **v > 1).count()
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(12, solve(Input::from_readable(INPUT)));
    }
}
