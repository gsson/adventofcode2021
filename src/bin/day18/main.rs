use adventofcode2021::*;
use std::fmt::Debug;
use std::rc::Rc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Input::from_file("src/bin/day18/input.txt");
    let numbers = parse(input);
    let numbers = numbers.into_iter().map(|n| flatten(&n)).collect::<Vec<_>>();
    let a = part1::solve(numbers.clone());
    eprintln!("Part 1: {:?}", a);
    assert_eq!(3654, a);
    let a = part2::solve(numbers);
    eprintln!("Part 2: {:?}", a);
    assert_eq!(4578, a);
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Number {
    Regular(i32),
    Pair(Rc<(Number, Number)>),
}

fn parse_regular(v: &str) -> (Number, &str) {
    let end = v.find(|c: char| !c.is_ascii_digit()).unwrap_or(v.len());
    let (n, v) = v.split_at(end);
    (Number::Regular(n.parse().unwrap()), v)
}

fn parse_pair(v: &str) -> (Number, &str) {
    assert!(v.starts_with('['));
    let (a, v) = parse_number(&v[1..]);
    assert!(v.starts_with(','));
    let (b, v) = parse_number(&v[1..]);
    assert!(v.starts_with(']'));
    (Number::Pair(Rc::new((a, b))), &v[1..])
}

fn parse_number(v: &str) -> (Number, &str) {
    if v.starts_with('[') {
        parse_pair(v)
    } else {
        parse_regular(v)
    }
}

fn parse<R: std::io::BufRead>(input: Input<R>) -> Vec<Number> {
    fn parse_line<R: std::io::BufRead>(input: Input<R>) -> Number {
        parse_number(&input.into_string()).0
    }
    input.lines().map(parse_line).collect()
}

fn magnitude(number: impl IntoIterator<Item = (i32, i32)>) -> i32 {
    fn aggregate(stack: &mut Vec<(i32, i32)>, b: i32, db: i32) {
        match stack.last().copied() {
            Some((a, da)) if da == db => {
                let _ = stack.pop();
                aggregate(stack, 3 * a + 2 * b, da - 1);
            }
            _ => stack.push((b, db)),
        }
    }
    let mut stack = Vec::new();
    for (b, db) in number {
        aggregate(&mut stack, b, db);
    }
    stack.pop().unwrap().0
}

#[test]
fn test_magnitude() {
    fn test(expected: i32, input: &str) {
        let input = flatten(&parse_number(input).0);
        assert_eq!(expected, magnitude(input))
    }
    test(129, "[[9,1],[1,9]]");
    test(143, "[[1,2],[[3,4],5]]");
    test(1384, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    test(445, "[[[[1,1],[2,2]],[3,3]],[4,4]]");
    test(791, "[[[[3,0],[5,3]],[4,4]],[5,5]]");
    test(1137, "[[[[5,0],[7,4]],[5,5]],[6,6]]");
    test(
        3488,
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
    );
}

fn flatten(number: &Number) -> Vec<(i32, i32)> {
    fn flatten_into(depth: i32, number: &Number, stack: &mut Vec<(i32, i32)>) {
        match number {
            Number::Regular(i) => stack.push((*i, depth)),
            Number::Pair(pair) => {
                flatten_into(depth + 1, &pair.0, stack);
                flatten_into(depth + 1, &pair.1, stack);
            }
        }
    }

    let mut stack = Vec::new();
    flatten_into(0, number, &mut stack);
    stack
}

fn explode(number: &mut Vec<(i32, i32)>) -> bool {
    for i in 0.. {
        if i > number.len() - 1 {
            return false;
        }
        // Adjacent values with the same depth means it's a pair of regular numbers
        // It was promised anything needing to be exploded would be regular numbers.
        if number[i].1 == 5 && number[i + 1].1 == 5 {
            let a = number[i].0;
            let b = number[i + 1].0;
            // .., (a, 5), (b, 5), .. => .., (0, 4), ..
            number[i] = (0, 4);
            number.remove(i + 1);
            // Update previous regular number
            // .., (n, _), .. => .., (n + a, _), ..
            if i > 0 {
                number[i - 1].0 += a;
            }
            // Update next regular number
            // .., (n, _), .. => .., (n + b, _), ..
            if i < number.len() - 1 {
                number[i + 1].0 += b;
            }
            return true;
        }
    }
    unreachable!()
}

fn split(number: &mut Vec<(i32, i32)>) -> bool {
    for i in 0.. {
        if i > number.len() - 1 {
            return false;
        }

        if number[i].0 >= 10 {
            let a = number[i].0 / 2;
            let b = (number[i].0 + 1) / 2;

            number[i] = (a, number[i].1 + 1);
            number.insert(i + 1, (b, number[i].1));
            return true;
        }
    }
    unreachable!()
}

#[test]
fn test_split() {
    fn test(expected: &str, input: &str) {
        let expected = flatten(&parse_number(expected).0);
        let mut input = flatten(&parse_number(input).0);
        split(&mut input);
        assert_eq!(expected, input);
    }

    test("[[5,5],11]", "[10,11]");
    test("[[5,5],[5,6]]", "[[5,5],11]")
}

fn reduce(number: &mut Vec<(i32, i32)>) {
    loop {
        if explode(number) {
            continue;
        }
        if !split(number) {
            return;
        }
    }
}

#[test]
fn test_reduce() {
    fn test(expected: &str, input: &str) {
        let expected = flatten(&parse_number(expected).0);
        let mut input = flatten(&parse_number(input).0);
        reduce(&mut input);
        assert_eq!(expected, input);
    }

    test(
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
    );
}

fn add(
    a: impl IntoIterator<Item = (i32, i32)>,
    b: impl IntoIterator<Item = (i32, i32)>,
) -> Vec<(i32, i32)> {
    let mut number = a
        .into_iter()
        .chain(b.into_iter())
        .map(|(n, d)| (n, d + 1))
        .collect();
    reduce(&mut number);
    number
}

#[test]
fn test_add() {
    fn test(expected: &str, a: &str, b: &str) {
        let expected = flatten(&parse_number(expected).0);
        let a = flatten(&parse_number(a).0);
        let b = flatten(&parse_number(b).0);
        let out = add(a, b);
        assert_eq!(expected, out);
    }

    test(
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        "[[[[4,3],4],4],[7,[[8,4],9]]]",
        "[1,1]",
    );
}

#[test]
fn test_add_sum() {
    fn sum(numbers: Vec<Vec<(i32, i32)>>) -> Vec<(i32, i32)> {
        numbers.into_iter().reduce(add).unwrap()
    }

    fn test(expected: &str, a: &[&str]) {
        let expected = flatten(&parse_number(expected).0);
        let input = a
            .iter()
            .map(|s| flatten(&parse_number(s).0))
            .collect::<Vec<_>>();
        let out = sum(input);
        assert_eq!(expected, out);
    }

    test(
        "[[[[1,1],[2,2]],[3,3]],[4,4]]",
        &["[1,1]", "[2,2]", "[3,3]", "[4,4]"],
    );
    test(
        "[[[[3,0],[5,3]],[4,4]],[5,5]]",
        &["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"],
    );
    test(
        "[[[[5,0],[7,4]],[5,5]],[6,6]]",
        &["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"],
    );
    test(
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        &[
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ],
    );
}

#[test]
fn test_explode() {
    fn test(expected: &str, input: &str) {
        let expected = flatten(&parse_number(expected).0);
        let mut input = flatten(&parse_number(input).0);
        explode(&mut input);
        assert_eq!(expected, input);
    }

    test("[[[[0,9],2],3],4]", "[[[[[9,8],1],2],3],4]");
    test("[7,[6,[5,[7,0]]]]", "[7,[6,[5,[4,[3,2]]]]]");
    test("[[6,[5,[7,0]]],3]", "[[6,[5,[4,[3,2]]]],1]");
    test(
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
    );
    test(
        "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
    );
}

mod part1 {
    use crate::*;

    pub fn solve(numbers: Vec<Vec<(i32, i32)>>) -> i32 {
        numbers.into_iter().reduce(add).map(magnitude).unwrap()
    }
}

mod part2 {
    use crate::*;
    pub fn solve(numbers: Vec<Vec<(i32, i32)>>) -> i32 {
        let mut m = 0;
        for j in 0..numbers.len() {
            for i in 0..numbers.len() {
                if i != j {
                    let a = magnitude(add(numbers[i].iter().copied(), numbers[j].iter().copied()));
                    let b = magnitude(add(numbers[j].iter().copied(), numbers[i].iter().copied()));
                    m = m.max(a);
                    m = m.max(b);
                }
            }
        }
        m
    }
}
