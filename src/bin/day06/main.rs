use adventofcode2021::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = part1::solve(Input::from_file("src/bin/day06/input.txt"));
    eprintln!("Part 1: {}", a);
    assert_eq!(395627, a);
    let a = part2::solve(Input::from_file("src/bin/day06/input.txt"));
    eprintln!("Part 2: {}", a);
    assert_eq!(1767323539209, a);
    Ok(())
}

const MAX_CYCLE: usize = 8;
const RESTART_CYCLE: usize = 6;

fn parse<R: std::io::BufRead>(input: Input<R>) -> [u64; MAX_CYCLE + 1] {
    let mut cycles = [0u64; MAX_CYCLE + 1];
    for cycle in input.comma_separated().parse::<usize>() {
        cycles[cycle] += 1;
    }
    cycles
}

mod part1 {
    use crate::{parse, MAX_CYCLE, RESTART_CYCLE};
    use adventofcode2021::*;

    pub fn solve<R: std::io::BufRead>(input: Input<R>) -> u64 {
        let mut cycles = parse(input);
        for _ in 0..80 {
            cycles.rotate_left(1);
            cycles[RESTART_CYCLE] += cycles[MAX_CYCLE];
        }
        cycles.iter().sum::<u64>()
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(5934, solve(Input::from_readable(INPUT)));
    }
}

mod part2 {
    use crate::{parse, MAX_CYCLE, RESTART_CYCLE};
    use adventofcode2021::*;

    pub fn solve<R: std::io::BufRead>(input: Input<R>) -> u64 {
        let mut cycles = parse(input);
        for _ in 0..256 {
            cycles.rotate_left(1);
            cycles[RESTART_CYCLE] += cycles[MAX_CYCLE];
        }
        cycles.iter().sum::<u64>()
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(26984457539, solve(Input::from_readable(INPUT)));
    }
}
