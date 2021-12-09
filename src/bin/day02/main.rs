use adventofcode2021::*;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = part1::solve(Input::from_file("src/bin/day02/input.txt"));
    eprintln!("Part 1: {}", a);
    assert_eq!(1488669, a);
    let a = part2::solve(Input::from_file("src/bin/day02/input.txt"));
    eprintln!("Part 2: {}", a);
    assert_eq!(1176514794, a);
    Ok(())
}

enum Instruction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromInput for Instruction {
    fn from_input<R: BufRead>(input: Input<R>) -> Self {
        let (instruction, magnitude) = input.delimited_once(" ");
        let magnitude = magnitude.parse::<i32>();
        match instruction.into_string().as_str() {
            "forward" => Self::Forward(magnitude),
            "up" => Self::Up(magnitude),
            "down" => Self::Down(magnitude),
            _ => unreachable!(),
        }
    }
}

mod part1 {
    use crate::Instruction;
    use adventofcode2021::{Input, TokenParse};

    pub fn solve<R: std::io::BufRead>(input: Input<R>) -> i32 {
        let (h, d) = input.lines().parse::<Instruction>().fold((0, 0), fold);
        h * d
    }

    fn fold((horizontal, depth): (i32, i32), instruction: Instruction) -> (i32, i32) {
        match instruction {
            Instruction::Forward(magnitude) => (horizontal + magnitude, depth),
            Instruction::Up(magnitude) => (horizontal, depth - magnitude),
            Instruction::Down(magnitude) => (horizontal, depth + magnitude),
        }
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(150, solve(Input::from_readable(INPUT)));
    }
}

mod part2 {
    use crate::Instruction;
    use adventofcode2021::{Input, TokenParse};

    pub fn solve<R: std::io::BufRead>(input: Input<R>) -> i32 {
        let (_, h, d) = input.lines().parse::<Instruction>().fold((0, 0, 0), fold);
        h * d
    }

    fn fold(
        (aim, horizontal, depth): (i32, i32, i32),
        instruction: Instruction,
    ) -> (i32, i32, i32) {
        match instruction {
            Instruction::Forward(magnitude) => {
                (aim, horizontal + magnitude, depth + aim * magnitude)
            }
            Instruction::Up(magnitude) => (aim - magnitude, horizontal, depth),
            Instruction::Down(magnitude) => (aim + magnitude, horizontal, depth),
        }
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(900, solve(Input::from_readable(INPUT)));
    }
}
