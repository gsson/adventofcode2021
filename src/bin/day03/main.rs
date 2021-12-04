use adventofcode2021::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = part1::solve(Input::from_file("src/bin/day03/input.txt"));
    eprintln!("Part 1: {}", a);
    assert_eq!(1025636, a);
    let a = part2::solve(Input::from_file("src/bin/day03/input.txt"));
    eprintln!("Part 2: {}", a);
    assert_eq!(793873, a);
    Ok(())
}

mod part1 {
    use adventofcode2021::*;

    pub fn solve<R: std::io::BufRead>(input: Input<R>) -> u32 {
        let values = input
            .lines()
            .map(|s| u32::from_str_radix(&s.into_string(), 2).unwrap())
            .collect::<Vec<_>>();
        let threshold = (values.len() / 2) as u32;
        let highest_one_bit = values.iter().bitor::<u32>().highest_one_bit();
        let mask = highest_one_bit | (highest_one_bit - 1);
        let number_of_bits = (highest_one_bit.trailing_zeros() + 1) as usize;

        let bit_counts = values
            .into_iter()
            .fold(vec![0u32; number_of_bits], |mut state, value| {
                value.bit_indices().for_each(|i| state[i as usize] += 1);
                state
            });
        let gamma = bit_counts
            .into_iter()
            .rev()
            .fold(0u32, |n, i| n.push_lsb(i >= threshold));
        let epsilon = (!gamma) & mask;
        gamma * epsilon
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(198, solve(Input::from_readable(INPUT)));
    }
}

mod part2 {
    use adventofcode2021::*;

    fn oxygen(highest_bit: u32, values: &[u32]) -> u32 {
        let mut test_bit = highest_bit;
        let mut remaining = values.to_vec();
        loop {
            let (ones, zeroes) = remaining
                .into_iter()
                .partition::<Vec<_>, _>(|n| *n & test_bit != 0);
            remaining = if ones.len() >= zeroes.len() {
                ones
            } else {
                zeroes
            };
            if remaining.len() == 1 {
                return remaining.pop().unwrap();
            }
            test_bit >>= 1;
        }
    }

    fn co2_scrubber(highest_bit: u32, values: &[u32]) -> u32 {
        let mut test_bit = highest_bit;
        let mut remaining = values.to_vec();
        loop {
            let (ones, zeroes) = remaining
                .into_iter()
                .partition::<Vec<_>, _>(|n| *n & test_bit != 0);
            remaining = if ones.len() < zeroes.len() {
                ones
            } else {
                zeroes
            };
            if remaining.len() == 1 {
                return remaining.pop().unwrap();
            }
            test_bit >>= 1;
        }
    }

    pub fn solve<R: std::io::BufRead>(input: Input<R>) -> i32 {
        let values = input
            .lines()
            .map(|s| u32::from_str_radix(&s.into_string(), 2).unwrap())
            .collect::<Vec<_>>();

        let highest_one_bit = values.iter().bitor::<u32>().highest_one_bit();
        let oxygen = oxygen(highest_one_bit, &values);
        let co2_scrubber = co2_scrubber(highest_one_bit, &values);
        (oxygen * co2_scrubber) as i32
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(230, solve(Input::from_readable(INPUT)));
    }
}
