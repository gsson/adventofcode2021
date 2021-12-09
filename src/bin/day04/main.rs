use adventofcode2021::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = part1::solve(Input::from_file("src/bin/day04/input.txt"));
    eprintln!("Part 1: {}", a);
    assert_eq!(38913, a);
    let a = part2::solve(Input::from_file("src/bin/day04/input.txt"));
    eprintln!("Part 2: {}", a);
    assert_eq!(16836, a);
    Ok(())
}

struct Board {
    numbers: [u128; 10],
}

impl Board {
    const ROWS: usize = 5;
    const COLUMNS: usize = 5;
    fn new() -> Self {
        Self {
            numbers: [0; Self::ROWS + Self::COLUMNS],
        }
    }
    #[inline]
    fn bit(n: u32) -> u128 {
        1u128 << n
    }
    #[inline]
    fn mask(n: u32) -> u128 {
        !Self::bit(n)
    }
    fn set(&mut self, r: usize, c: usize, n: u32) {
        let bit = Self::bit(n);
        self.numbers[r] |= bit;
        self.numbers[c + Self::ROWS] |= bit;
    }

    fn mark(&mut self, n: u32) {
        let mask = Self::mask(n);
        for n in &mut self.numbers {
            *n &= mask;
        }
    }

    fn winner(&self) -> bool {
        self.numbers.iter().any(|n| *n == 0)
    }

    fn unmarked_sum(&self) -> u32 {
        self.numbers[0..Self::ROWS]
            .iter()
            .flat_map(|n| n.bit_indices())
            .sum()
    }
}

fn parse<R: std::io::BufRead>(input: Input<R>) -> (Vec<u32>, Vec<Board>) {
    fn board_row(board: &mut Board, r: usize, input_line: BufInput) {
        input_line
            .words()
            .parse::<u32>()
            .enumerate()
            .for_each(|(c, n)| board.set(r, c, n));
    }
    fn board(input_section: BufInput) -> Board {
        let mut board = Board::new();
        input_section
            .lines()
            .enumerate()
            .for_each(|(r, line)| board_row(&mut board, r, line));
        board
    }
    let (numbers, boards) = input.delimited_once("\n\n");
    let numbers = numbers
        .comma_separated()
        .parse::<u32>()
        .collect::<Vec<_>>();

    let boards = boards.sections().map(board).collect::<Vec<_>>();

    (numbers, boards)
}

mod part1 {
    use crate::parse;
    use adventofcode2021::*;

    pub fn solve<R: std::io::BufRead>(input: Input<R>) -> u32 {
        let (numbers, mut boards) = parse(input);

        for number in numbers {
            for board in boards.iter_mut() {
                board.mark(number);
                if board.winner() {
                    return number * board.unmarked_sum();
                }
            }
        }
        unreachable!()
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(4512, solve(Input::from_readable(INPUT)));
    }
}

mod part2 {
    use crate::parse;
    use adventofcode2021::*;

    pub fn solve<R: std::io::BufRead>(input: Input<R>) -> u32 {
        let (numbers, mut boards) = parse(input);

        for number in numbers {
            if boards.len() > 1 {
                boards = boards
                    .into_iter()
                    .map(|mut board| {
                        board.mark(number);
                        board
                    })
                    .filter(|board| !board.winner())
                    .collect();
            } else {
                let loser = boards.get_mut(0).unwrap();
                loser.mark(number);
                if loser.winner() {
                    return number * loser.unmarked_sum();
                }
            }
        }
        unreachable!()
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(1924, solve(Input::from_readable(INPUT)));
    }
}
