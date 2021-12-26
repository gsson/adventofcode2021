use adventofcode2021::vector::Vec2i;
use adventofcode2021::*;
use std::cmp::Ordering;
use std::ops::{Bound, RangeBounds};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Input::from_file("src/bin/day17/input.txt");
    let (min_x, max_x, min_y, max_y) = parse(input);
    let a = part1::solve(min_x, max_x, min_y, max_y);
    eprintln!("Part 1: {:?}", a);
    assert_eq!(7750, a);
    let a = part2::solve(min_x, max_x, min_y, max_y);
    eprintln!("Part 2: {:?}", a);
    assert_eq!(4120, a);
    Ok(())
}

fn parse<R: std::io::BufRead>(input: Input<R>) -> (i32, i32, i32, i32) {
    fn parse_range<R: std::io::BufRead>(input: Input<R>) -> (i32, i32) {
        let (min, max) = input.delimited_once("..");
        (min.parse(), max.parse())
    }
    let (x, y) = input.delimited_once(", ");
    let x = x.delimited_once("=").1;
    let (min_x, max_x) = parse_range(x);
    let y = y.delimited_once("=").1;
    let (min_y, max_y) = parse_range(y);
    (min_x, max_x, min_y, max_y)
}

struct YIter {
    y: i32,
    v: i32,
}

impl YIter {
    pub fn new(v: i32) -> Self {
        Self { y: 0, v }
    }
}

impl Iterator for YIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.y += self.v;
        self.v -= 1;
        Some(self.y)
    }
}

fn max_height(min_y: i32) -> i32 {
    min_y * (min_y + 1) / 2
}

fn pos_x(vx: i32, t: i32) -> i32 {
    let t = t + 1;
    if t < vx {
        (2 * vx - t + 1) * t / 2
    } else {
        pos_x_end(vx)
    }
}

#[inline]
fn pos_x_end(vx: i32) -> i32 {
    vx * (vx + 1) / 2
}

fn find_yts<R>(init_vel: i32, range: R) -> Vec<i32>
where
    R: RangeBounds<i32>,
{
    let lower = match range.start_bound() {
        Bound::Included(i) => i - 1,
        Bound::Excluded(i) => *i,
        Bound::Unbounded => unreachable!(),
    };
    YIter::new(init_vel)
        .take_while(|y| *y > lower)
        .enumerate()
        .filter_map(|(t, y)| range.contains(&y).then(|| t as i32))
        .collect::<Vec<i32>>()
}

fn update_velocity(velocity: Vec2i) -> Vec2i {
    let Vec2i(dx, dy) = velocity;
    let dx = match dx.cmp(&0) {
        Ordering::Less => dx + 1,
        Ordering::Equal => dx,
        Ordering::Greater => dx - 1,
    };
    let dy = dy - 1;
    Vec2i(dx, dy)
}

pub fn simulate(
    mut velocity: Vec2i,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
) -> Option<i32> {
    let mut position = Vec2i(0, 0);
    let mut height = position.1;
    loop {
        eprintln!("{} {}", position.0, position.1);

        if position.0 >= min_x && position.1 <= max_y {
            if position.0 > max_x || position.1 < min_y {
                return None;
            } else {
                return Some(height);
            }
        }

        position += velocity;
        height = height.max(position.1);
        velocity = update_velocity(velocity);
    }
}

mod part1 {
    use crate::*;

    pub fn solve(_min_x: i32, _max_x: i32, min_y: i32, _max_y: i32) -> i32 {
        max_height(min_y)
    }

    #[test]
    fn test() {
        assert_eq!(45, solve(20, 30, -10, -5));
    }
}

mod part2 {
    use crate::*;

    fn find_vx(x: f64) -> f64 {
        // x = (vx * (vx - 1)) / 2
        // x = (vx ^ 2 - vx) / 2
        // x = 0.5 * vx ^ 2 - 0.5 * vx
        // 0 = 0.5 * vx ^ 2 - 0.5 * vx - x
        // vx = -0.5 + sqrt(0.5 ^ 2 - 4 * 0.5 * (-x))
        // vx = sqrt(0.25 + 2 * x) - 0.5
        f64::sqrt(0.25 + 2.0 * x) - 0.5
    }

    fn min_vx(x: i32) -> i32 {
        find_vx(x as f64).floor() as i32
    }

    fn test_x<XR>(min_vx: i32, max_vx: i32, xs: XR, ts: &[i32]) -> usize
    where
        XR: RangeBounds<i32>,
    {
        (min_vx..=max_vx)
            .filter(|vx| ts.iter().any(|t| xs.contains(&pos_x(*vx, *t))))
            .count()
    }

    pub fn solve(min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> usize {
        let min_vy = min_y;
        let max_vy = max_height(min_y);
        let min_vx = min_vx(min_x);
        let max_vx = max_x;
        (min_vy..=max_vy)
            .map(|vy| (vy, find_yts(vy, min_y..=max_y)))
            .filter(|(_, yts)| !yts.is_empty())
            .map(|(_, yts)| test_x(min_vx, max_vx, min_x..=max_x, &yts))
            .sum()
    }

    #[test]
    fn test() {
        assert_eq!(112, solve(20, 30, -10, -5));
    }
}
