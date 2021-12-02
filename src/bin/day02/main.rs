use adventofcode2021::*;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let a = part1::solve(Input::from_file("src/bin/day02/input.txt"));
    eprintln!("Part 1: {}", a);
    assert_eq!(1488669, a);
    let a = part2::solve(Input::from_file("src/bin/day02/input.txt"));
    eprintln!("Part 2: {}", a);
    assert_eq!(1176514794, a);
    Ok(())
}

mod part1 {
    use adventofcode2021::Input;

    pub fn solve<I: std::io::Read>(input: Input<I>) -> i32 {
        let (h, d) = input
            .lines()
            .fold((0, 0), fold);
        h * d
    }

    fn fold((horizontal, depth): (i32, i32), instruction: String) -> (i32, i32) {
        let (command, magnitude) = instruction.split_once(' ').unwrap();
        let magnitude = magnitude.parse::<i32>().unwrap();
        match command {
            "forward" => (horizontal + magnitude, depth),
            "up" => (horizontal, depth - magnitude),
            "down" => (horizontal, depth + magnitude),
            _ => unreachable!()
        }
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(150, solve(Input::from_readable(INPUT)));
    }
}

mod part2 {
    use adventofcode2021::Input;

    pub fn solve<I: std::io::Read>(input: Input<I>) -> i32 {
        let (_, h, d) = input
            .lines()
            .fold((0, 0, 0), fold);
        h * d
    }

    fn fold((aim, horizontal, depth): (i32, i32, i32), instruction: String) -> (i32, i32, i32) {
        let (command, magnitude) = instruction.split_once(' ').unwrap();
        let magnitude = magnitude.parse::<i32>().unwrap();
        match command {
            "forward" => (aim, horizontal + magnitude, depth + aim * magnitude),
            "up" => (aim - magnitude, horizontal, depth),
            "down" => (aim + magnitude, horizontal, depth),
            _ => unreachable!()
        }
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(900, solve(Input::from_readable(INPUT)));
    }
}