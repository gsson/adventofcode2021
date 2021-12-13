use adventofcode2021::*;
use std::collections::{BTreeSet, HashMap};
use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Input::from_file("src/bin/day12/input.txt");
    let graph = graph_from_input(input);
    let a = part1::solve(&graph);
    eprintln!("Part 1: {}", a);
    assert_eq!(3738, a);
    let a = part2::solve(&graph);
    eprintln!("Part 2: {}", a);
    assert_eq!(120506, a);
    Ok(())
}

fn parse<R: std::io::BufRead>(input: Input<R>) -> Vec<(String, String)> {
    fn connection<R: std::io::BufRead>(input: Input<R>) -> (String, String) {
        let (a, b) = input.delimited_once("-");
        (a.into_string(), b.into_string())
    }
    input.lines().map(connection).collect()
}

pub struct Graph {
    connections: Vec<Vec<u8>>,
    small_caverns: NodeSet,
    start: u8,
    end: u8,
}

impl Graph {
    #[inline]
    fn is_end(&self, index: u8) -> bool {
        index == self.end
    }
    #[inline]
    fn is_start(&self, index: u8) -> bool {
        index == self.start
    }
}

fn graph_from_input<B: BufRead>(input: Input<B>) -> Graph {
    create_graph(parse(input))
}

fn create_graph(connections: Vec<(String, String)>) -> Graph {
    fn is_small_cavern(name: &str) -> bool {
        name.chars().all(|c| c.is_ascii_lowercase())
    }
    let caverns = connections
        .iter()
        .flat_map(|(a, b)| [a.as_str(), b.as_str()].into_iter())
        .collect::<BTreeSet<_>>();
    let caverns = Vec::from_iter(caverns.into_iter());
    let name_to_index = caverns
        .iter()
        .enumerate()
        .map(|(i, name)| (*name, i as u8))
        .collect::<HashMap<_, _>>();
    let mut connections2 = Vec::with_capacity(caverns.len());
    connections2.extend(std::iter::repeat(Vec::new()).take(caverns.len()));
    for (a, b) in &connections {
        let a = name_to_index[a.as_str()];
        let b = name_to_index[b.as_str()];
        connections2[a as usize].push(b);
        connections2[b as usize].push(a);
    }
    let start = name_to_index["start"];
    let end = name_to_index["end"];
    let small_caverns = caverns
        .into_iter()
        .enumerate()
        .filter_map(|(i, name)| is_small_cavern(name).then(|| i))
        .fold(NodeSet::empty(), |s, i| s.add(i as u8));

    Graph {
        connections: connections2,
        small_caverns,
        start,
        end,
    }
}

#[derive(Copy, Clone)]
struct NodeSet(u64);
impl NodeSet {
    fn empty() -> Self {
        Self(0)
    }
    fn new(i: u8) -> Self {
        Self(Self::bit(i))
    }
    #[inline]
    fn bit(i: u8) -> u64 {
        1 << (i as u32)
    }
    #[inline]
    fn add(self, i: u8) -> Self {
        Self(self.0 | Self::bit(i))
    }
    #[inline]
    fn contains(self, i: u8) -> bool {
        self.0 & Self::bit(i) != 0
    }
    #[inline]
    fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}

mod part1 {
    use crate::*;

    fn traverse(graph: &Graph) -> usize {
        let mut complete = 0;
        let mut stack = vec![(graph.start, NodeSet::new(graph.start))];
        while let Some((cavern, visited_small)) = stack.pop() {
            for adjacent in &graph.connections[cavern as usize] {
                let adjacent = *adjacent;
                if visited_small.contains(adjacent) {
                    continue;
                }
                if graph.is_end(adjacent) {
                    complete += 1;
                    continue;
                }
                let visited_small = visited_small
                    .add(adjacent)
                    .intersection(graph.small_caverns);
                stack.push((adjacent, visited_small));
            }
        }

        complete
    }

    pub fn solve(graph: &Graph) -> usize {
        traverse(graph)
    }

    #[test]
    fn test1() {
        const INPUT: &[u8] = include_bytes!("test1.txt");
        assert_eq!(10, solve(&graph_from_input(Input::from_readable(INPUT))));
    }
    #[test]
    fn test2() {
        const INPUT: &[u8] = include_bytes!("test2.txt");
        assert_eq!(19, solve(&graph_from_input(Input::from_readable(INPUT))));
    }
    #[test]
    fn test3() {
        const INPUT: &[u8] = include_bytes!("test3.txt");
        assert_eq!(226, solve(&graph_from_input(Input::from_readable(INPUT))));
    }
}

mod part2 {
    use crate::*;

    fn traverse(graph: &Graph) -> usize {
        let mut complete = 0;
        let mut stack = vec![(graph.start, NodeSet::new(graph.start), false)];
        while let Some((cavern, visited_small, any_visited_twice)) = stack.pop() {
            for adjacent in &graph.connections[cavern as usize] {
                let adjacent = *adjacent;
                let this_visited_once = visited_small.contains(adjacent);
                if (this_visited_once && any_visited_twice) || graph.is_start(adjacent) {
                    continue;
                }
                if graph.is_end(adjacent) {
                    complete += 1;
                    continue;
                }
                let visited_small = visited_small
                    .add(adjacent)
                    .intersection(graph.small_caverns);
                stack.push((
                    adjacent,
                    visited_small,
                    any_visited_twice || this_visited_once,
                ));
            }
        }

        complete
    }

    pub fn solve(graph: &Graph) -> usize {
        traverse(graph)
    }

    #[test]
    fn test1() {
        const INPUT: &[u8] = include_bytes!("test1.txt");
        assert_eq!(36, solve(&graph_from_input(Input::from_readable(INPUT))));
    }
    #[test]
    fn test2() {
        const INPUT: &[u8] = include_bytes!("test2.txt");
        assert_eq!(103, solve(&graph_from_input(Input::from_readable(INPUT))));
    }
    #[test]
    fn test3() {
        const INPUT: &[u8] = include_bytes!("test3.txt");
        assert_eq!(3509, solve(&graph_from_input(Input::from_readable(INPUT))));
    }
}
