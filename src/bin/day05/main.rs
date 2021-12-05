use std::cmp::max;
use std::convert::Infallible;
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;
use adventofcode2021::*;

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

impl FromStr for Point {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").unwrap();
        Ok(Self { x: x.parse().unwrap(), y: y.parse().unwrap() } )
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
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
        let slope = Point { x: dx.signum(), y: dy.signum() };
        LinePointsIter {
            start: self.start,
            slope,
            length,
            i: 0
        }
    }
}

impl FromStr for Line {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").unwrap();
        Ok(Self { start: start.parse().unwrap(), end: end.parse().unwrap()})
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

fn parse<R: std::io::BufRead>(input: Input<R>) -> impl Iterator<Item=Line> {
    input.lines()
        .parse()
}

mod part1 {
    use std::collections::HashSet;
    use crate::parse;
    use adventofcode2021::*;

    pub fn solve<R: std::io::BufRead>(input: Input<R>) -> usize {
        let (_, overlaps) = parse(input)
            .filter(|l| l.is_orthogonal())
            .flat_map(move |l| l.points())
            .fold((HashSet::new(), HashSet::new()), |(mut all, mut overlaps), p| {
                if !all.insert(p) {
                    overlaps.insert(p);
                }
                (all, overlaps)
            });
        overlaps.len()
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(5, solve(Input::from_readable(INPUT)));
    }
}

mod part2 {
    use std::collections::HashSet;
    use crate::parse;
    use adventofcode2021::*;

    pub fn solve<R: std::io::BufRead>(input: Input<R>) -> usize {
        let (_, overlaps) = parse(input)
            .flat_map(|l| l.points())
            .fold((HashSet::new(), HashSet::new()), |(mut all, mut overlaps), p| {
                if !all.insert(p) {
                    overlaps.insert(p);
                }
                (all, overlaps)
            });
        overlaps.len()
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(12, solve(Input::from_readable(INPUT)));
    }
}
