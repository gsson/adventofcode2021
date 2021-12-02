use adventofcode2021::*;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let a = part1::solve(Input::from_file("src/bin/day01/input.txt"));
    eprintln!("Part 1: {}", a);
    assert_eq!(1791, a);
    let a = part2::solve(Input::from_file("src/bin/day01/input.txt"));
    eprintln!("Part 2: {}", a);
    assert_eq!(1822, a);
    Ok(())
}

mod part1 {
    use adventofcode2021::{Input, TokenParse};

    pub fn solve<I: std::io::Read>(input: Input<I>) -> usize {
        let (r, _) = input
            .lines()
            .numbers()
            .fold((0, None), fold);
        r
    }

    fn fold(state: (usize, Option<usize>), next: usize) -> (usize, Option<usize>) {
        match state {
            (n, Some(prev)) if prev < next => (n + 1, Some(next)),
            (n, _) => (n, Some(next)),
        }
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(7, solve(Input::from_readable(INPUT)));
    }
}

mod part2 {
    use adventofcode2021::{Input, TokenParse};

    pub fn solve<I: std::io::Read>(input: Input<I>) -> usize {
        let (r, _, _) = input
            .lines()
            .numbers()
            .fold((0, None, (0, 0)), fold);
        r
    }

    fn fold(state: (usize, Option<usize>, (usize, usize)), next: usize) -> (usize, Option<usize>, (usize, usize)) {
        match state {
            (n, Some(prev_sum), (prev1, prev2)) if prev_sum < prev1 + prev2 + next => (n + 1, Some(prev1 + prev2 + next), (prev2, next)),
            (n, Some(_), (prev1, prev2)) => (n, Some(prev1 + prev2 + next), (prev2, next)),
            (n, None, (0, 0)) => (n, None, (0, next)),
            (n, None, (0, prev2)) => (n, None, (prev2, next)),
            (n, None, (prev1, prev2)) => (n, Some(prev1 + prev2 + next), (prev2, next)),
        }
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(5, solve(Input::from_readable(INPUT)));
    }
}