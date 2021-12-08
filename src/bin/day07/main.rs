use adventofcode2021::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = part1::solve(Input::from_file("src/bin/day07/input.txt"));
    eprintln!("Part 1: {}", a);
    assert_eq!(326132, a);
    let a = part2::solve(Input::from_file("src/bin/day07/input.txt"));
    eprintln!("Part 2: {}", a);
    assert_eq!(88612508, a);
    Ok(())
}

fn parse<R: std::io::BufRead>(input: Input<R>) -> Vec<i32> {
    input.comma_separated().parse::<i32>().collect()
}

mod part1 {
    use crate::parse;
    use adventofcode2021::*;

    fn fuel_use(crab_positions: &[i32], target_position: i32) -> i32 {
        crab_positions
            .iter()
            .map(|p| (p - target_position).abs())
            .sum()
    }

    pub fn solve<R: std::io::BufRead>(input: Input<R>) -> i32 {
        let mut crab_positions = parse(input);
        crab_positions.sort_unstable();
        let n = crab_positions.len() as i32;
        let median = crab_positions[(n / 2) as usize];
        fuel_use(&crab_positions, median)
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(37, solve(Input::from_readable(INPUT)));
    }
}

mod part2 {
    use crate::parse;
    use adventofcode2021::*;
    use std::cmp::min;

    fn fuel_use(crab_positions: &[i32], target_position: i32) -> i32 {
        crab_positions
            .iter()
            .map(|p| {
                let distance = (p - target_position).abs();
                distance * (distance + 1) / 2
            })
            .sum()
    }

    pub fn solve<R: std::io::BufRead>(input: Input<R>) -> i32 {
        let crab_positions = parse(input);
        let n = crab_positions.len() as f64;
        let mean = crab_positions.iter().sum::<i32>() as f64 / n;
        let floor = mean.floor() as i32;
        let ceil = mean.ceil() as i32;
        min(
            fuel_use(&crab_positions, floor),
            fuel_use(&crab_positions, ceil),
        )
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(168, solve(Input::from_readable(INPUT)));
    }
}
