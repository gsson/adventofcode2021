use adventofcode2021::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = part1::solve(Input::from_file("src/bin/day10/input.txt"));
    eprintln!("Part 1: {}", a);
    assert_eq!(341823, a);
    let a = part2::solve(Input::from_file("src/bin/day10/input.txt"));
    eprintln!("Part 2: {}", a);
    assert_eq!(2801302861, a);
    Ok(())
}

fn parse<R: std::io::BufRead>(input: Input<R>) -> impl Iterator<Item = Vec<u8>> {
    input.lines().map(|l| l.into_bytes())
}

mod part1 {
    use crate::*;

    fn error_score(line: Vec<u8>) -> Option<usize> {
        let mut open_chunks = Vec::new();
        line.into_iter()
            .find_map(|b| match (b, &open_chunks.as_slice()) {
                (b'(', _) | (b'[', _) | (b'{', _) | (b'<', _) => {
                    open_chunks.push(b);
                    None
                }
                (b')', [.., b'('])
                | (b']', [.., b'['])
                | (b'}', [.., b'{'])
                | (b'>', [.., b'<']) => {
                    let _ = open_chunks.pop().unwrap();
                    None
                }
                (b')', _) => Some(3),
                (b']', _) => Some(57),
                (b'}', _) => Some(1197),
                (b'>', _) => Some(25137),
                _ => unreachable!(),
            })
    }

    pub fn solve<R: std::io::BufRead>(input: Input<R>) -> usize {
        let lines = parse(input);
        lines.filter_map(error_score).sum()
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(26397, solve(Input::from_readable(INPUT)));
    }
}

mod part2 {
    use crate::*;
    fn chunk_matches(open: u8, close: u8) -> bool {
        matches!(
            (open, close),
            (b'(', b')') | (b'[', b']') | (b'{', b'}') | (b'<', b'>')
        )
    }

    fn is_incomplete(line: Vec<u8>) -> Option<Vec<u8>> {
        let mut open_chunks = Vec::new();
        for b in line.into_iter() {
            match b {
                b'(' | b'[' | b'{' | b'<' => open_chunks.push(b),
                b')' | b']' | b'}' | b'>' => {
                    if let Some(last_open) = open_chunks.last() {
                        if chunk_matches(*last_open, b) {
                            open_chunks.pop().unwrap();
                        } else {
                            return None;
                        }
                    } else {
                        return None;
                    }
                }
                _ => unreachable!(),
            }
        }
        (!open_chunks.is_empty()).then(|| open_chunks)
    }
    fn brace_score(brace: u8) -> usize {
        match brace {
            b'(' => 1,
            b'[' => 2,
            b'{' => 3,
            b'<' => 4,
            _ => unreachable!(),
        }
    }

    pub fn solve<R: std::io::BufRead>(input: Input<R>) -> usize {
        let lines = parse(input);
        let mut scores = Vec::new();
        for line in lines {
            if let Some(mut open) = is_incomplete(line) {
                let mut score = 0;
                while let Some(next) = open.pop() {
                    score = score * 5 + brace_score(next);
                }
                scores.push(score);
            }
        }
        scores.sort_unstable();
        scores[scores.len() / 2]
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        assert_eq!(288957, solve(Input::from_readable(INPUT)));
    }
}
