use adventofcode2021::delimiters::{COMMA, SECTION};
use adventofcode2021::*;
use std::collections::{HashMap, HashSet};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Input::from_file("src/bin/day13/input.txt");
    let (points, instructions) = parse(input);
    let a = part1::solve(&points, &instructions);
    eprintln!("Part 1: {}", a);
    assert_eq!(827, a);
    let a = part2::solve(&points, &instructions);
    eprintln!("Part 2:\n{}", a);
    let expected = Input::from_file("src/bin/day13/part2-output.txt").into_string();
    assert_eq!(expected, a);
    Ok(())
}

#[allow(clippy::type_complexity)]
fn parse<R: std::io::BufRead>(input: Input<R>) -> (Vec<(i32, i32)>, Vec<(String, i32)>) {
    let (points, instructions) = input.delimited_once(SECTION);
    let points = points
        .lines()
        .map(|p| {
            let (x, y) = p.delimited_once(COMMA);
            (x.parse::<i32>(), y.parse::<i32>())
        })
        .collect::<Vec<_>>();
    let instructions = instructions
        .lines()
        .map(|instruction| {
            let (direction, position) = instruction.words().last().unwrap().delimited_once("=");
            (direction.into_string(), position.parse::<i32>())
        })
        .collect::<Vec<_>>();
    (points, instructions)
}

fn render(points: &[(i32, i32)]) -> String {
    let max_x = *points.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *points.iter().map(|(_, y)| y).max().unwrap();
    let mut lines = HashMap::<i32, HashSet<i32>>::new();
    for point in points {
        lines.entry(point.1).or_default().insert(point.0);
    }
    let empty = HashSet::new();

    let mut out = String::new();
    for y in 0..=max_y {
        let xs = lines.get(&y).unwrap_or(&empty);
        let line = (0..=max_x)
            .map(|x| if xs.contains(&x) { "#" } else { "." })
            .collect::<String>();
        out.push_str(&line);
        out.push('\n');
    }
    out
}

fn fold_position(p: i32, (_, fold): &&(String, i32)) -> i32 {
    if p > *fold {
        2 * fold - p
    } else {
        p
    }
}

mod part1 {
    use crate::*;

    pub fn solve(points: &[(i32, i32)], instructions: &[(String, i32)]) -> usize {
        let (x_folds, y_folds) = instructions
            .iter()
            .take(1)
            .partition::<Vec<_>, _>(|(d, _)| d == "x");
        let points = points
            .iter()
            .map(|(x, y)| {
                let x = x_folds.iter().fold(*x, fold_position);
                let y = y_folds.iter().fold(*y, fold_position);
                (x, y)
            })
            .collect::<HashSet<_>>();

        points.len()
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        let (points, instructions) = parse(Input::from_readable(INPUT));
        assert_eq!(17, solve(&points, &instructions));
    }
}

mod part2 {
    use crate::*;

    pub fn solve(points: &[(i32, i32)], instructions: &[(String, i32)]) -> String {
        let (x_folds, y_folds) = instructions
            .iter()
            .partition::<Vec<_>, _>(|(d, _)| d == "x");
        let points = points
            .iter()
            .map(|(x, y)| {
                let x = x_folds.iter().fold(*x, fold_position);
                let y = y_folds.iter().fold(*y, fold_position);
                (x, y)
            })
            .collect::<Vec<_>>();
        render(&points)
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        const OUTPUT: &str = include_str!("test-part2-output.txt");
        let (points, instructions) = parse(Input::from_readable(INPUT));
        assert_eq!(OUTPUT, solve(&points, &instructions));
    }
}
