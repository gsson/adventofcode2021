#![feature(control_flow_enum)]

use adventofcode2021::delimiters::LINE;
use adventofcode2021::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Input::from_file("src/bin/day21/input.txt");
    let (p1, p2) = parse(input);

    let a = part1::solve(p1, p2);
    eprintln!("Part 1: {:?}", a);
    assert_eq!(913560, a);

    let a = part2::solve(p1, p2);
    eprintln!("Part 2: {:?}", a);
    assert_eq!(110271560863819, a);
    Ok(())
}

fn parse<R: std::io::BufRead>(input: Input<R>) -> (usize, usize) {
    fn parse_player<R: std::io::BufRead>(input: Input<R>) -> usize {
        input.words().nth(4).unwrap().parse::<usize>()
    }

    let (p1, p2) = input.delimited_once(LINE);
    (parse_player(p1), parse_player(p2))
}

mod part1 {
    #[inline]
    const fn tri(a: usize) -> usize {
        (a * (a + 1)) >> 1
    }

    #[inline]
    const fn roll(a: usize) -> usize {
        let a = a * 3;
        tri(a + 3) - tri(a)
    }

    fn turn(r: usize, (pos, score): (usize, usize)) -> (usize, usize) {
        let roll = roll(r);
        let pos = (pos + roll - 1) % 10 + 1;
        (pos, score + pos)
    }

    fn play(p1: usize, p2: usize, limit: usize) -> (usize, usize, usize) {
        let mut a = (p1, 0);
        let mut b = (p2, 0);
        for i in 0.. {
            a = turn(i, a);
            if a.1 >= limit {
                return ((i + 1) * 3, a.1, b.1);
            }
            std::mem::swap(&mut a, &mut b);
        }
        unreachable!()
    }

    pub fn solve(p1: usize, p2: usize) -> usize {
        let (i, _winner, loser) = play(p1, p2, 1000);
        i * loser
    }

    #[cfg(test)]
    use crate::{parse, Input};

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        let (p1, p2) = parse(Input::from_readable(INPUT));
        assert_eq!(739785, solve(p1, p2));
    }
}

mod part2 {
    use std::collections::HashMap;
    use std::ops::ControlFlow;

    const ROLLS: [(usize, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    fn turn((pos, score): (usize, usize), roll: usize) -> (usize, usize) {
        let pos = (pos + roll) % 10;
        let score = score + pos + 1;
        (pos, score)
    }

    #[derive(Hash, Ord, PartialOrd, Eq, PartialEq)]
    struct GameState([(usize, usize); 2]);

    impl GameState {
        fn new(p1_position: usize, p2_position: usize) -> Self {
            Self([(p1_position - 1, 0), (p2_position - 1, 0)])
        }
        fn turn(&self, player: usize, roll: usize) -> GameState {
            match player {
                0 => GameState([turn(self.0[0], roll), self.0[1]]),
                1 => GameState([self.0[0], turn(self.0[1], roll)]),
                _ => unreachable!(),
            }
        }
        fn is_finished(&self) -> bool {
            self.0[0].1 >= 21 || self.0[1].1 >= 21
        }
        fn winner(&self) -> (usize, usize) {
            match (self.0[0].1, self.0[1].1) {
                (s1, _) if s1 >= 21 => (1, 0),
                (_, s2) if s2 >= 21 => (0, 1),
                _ => unreachable!(),
            }
        }
        fn round(self, player: usize, u: usize) -> GameStateIter {
            if self.is_finished() {
                GameStateIter::Finished(Some((self, u)))
            } else {
                GameStateIter::Ongoing(player, 0, self, u)
            }
        }
    }

    enum GameStateIter {
        Ongoing(usize, usize, GameState, usize),
        Finished(Option<(GameState, usize)>),
    }

    impl Iterator for GameStateIter {
        type Item = (GameState, usize);

        fn next(&mut self) -> Option<Self::Item> {
            match self {
                GameStateIter::Ongoing(player, i, s, u) => {
                    if *i < ROLLS.len() {
                        let (roll, count) = ROLLS[*i];
                        *i += 1;
                        Some((s.turn(*player, roll), *u * count))
                    } else {
                        None
                    }
                }
                GameStateIter::Finished(gs) => gs.take(),
            }
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            match self {
                GameStateIter::Ongoing(_, i, _, _) => (ROLLS.len() - i, Some(ROLLS.len() - i)),
                GameStateIter::Finished(_) => (1, Some(1)),
            }
        }
    }

    fn next(
        universes: HashMap<GameState, usize>,
        player: usize,
    ) -> ControlFlow<usize, HashMap<GameState, usize>> {
        let len = universes.len();
        let universes = universes
            .into_iter()
            .flat_map(|(game, u)| game.round(player, u))
            .fold(
                HashMap::<GameState, usize>::with_capacity(len + ROLLS.len()),
                |mut m, (g, n)| {
                    *m.entry(g).or_default() += n;
                    m
                },
            );
        if universes.keys().all(|g| g.is_finished()) {
            let (p1_wins, p2_wins) = universes
                .into_iter()
                .map(|(g, u)| {
                    let (a, b) = g.winner();
                    (a * u, b * u)
                })
                .fold(
                    (0, 0),
                    |(p1_wins, p2_wins), (p1_local_wins, p2_local_wins)| {
                        (p1_wins + p1_local_wins, p2_wins + p2_local_wins)
                    },
                );
            ControlFlow::Break(p1_wins.max(p2_wins))
        } else {
            ControlFlow::Continue(universes)
        }
    }

    pub fn solve(p1: usize, p2: usize) -> usize {
        (0..2)
            .cycle()
            .try_fold(HashMap::from([(GameState::new(p1, p2), 1)]), next)
            .break_value()
            .unwrap()
    }

    #[cfg(test)]
    use crate::{parse, Input};

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        let (p1, p2) = parse(Input::from_readable(INPUT));
        assert_eq!(444356092776315, solve(p1, p2));
    }
}
