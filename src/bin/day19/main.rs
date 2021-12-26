use adventofcode2021::vector::Vec3i;
use adventofcode2021::*;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct BeaconSignature(i32, i32);

impl BeaconSignature {
    fn from_pair(a: &Vec3i, b: &Vec3i) -> Self {
        let d = b - a;
        let m = d.manhattan();
        let mm = d.0.abs().max(d.1.abs()).max(d.2.abs());

        Self(m, mm)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Input::from_file("src/bin/day19/input.txt");
    let scanners = parse(input);

    let a = part1::solve(&scanners);
    eprintln!("Part 1: {:?}", a);
    assert_eq!(306, a);

    let a = part2::solve(&scanners);
    eprintln!("Part 2: {:?}", a);
    assert_eq!(9764, a);
    Ok(())
}

fn parse<R: std::io::BufRead>(input: Input<R>) -> Vec<Vec<Vec3i>> {
    fn parse_beacon<R: std::io::BufRead>(input: Input<R>) -> Vec3i {
        let mut i = input.comma_separated().parse::<i32>();
        let x = i.next().unwrap();
        let y = i.next().unwrap();
        let z = i.next().unwrap();
        Vec3i(x, y, z)
    }
    fn parse_scanner<R: std::io::BufRead>(input: Input<R>) -> Vec<Vec3i> {
        input.lines().skip(1).map(parse_beacon).collect()
    }
    input.sections().map(parse_scanner).collect()
}

fn beacon_signatures(beacons: &[Vec3i]) -> BTreeMap<BeaconSignature, (usize, usize)> {
    let mut signatures = BTreeMap::new();
    for j in 0..beacons.len() - 1 {
        let b = beacons[j];
        for i in j + 1..beacons.len() {
            assert!(signatures
                .insert(BeaconSignature::from_pair(&beacons[i], &b), (i, j))
                .is_none());
        }
    }
    signatures
}

// For overlapping scanners, the number of shared beacon
// signatures must be a least 12 choose 2 (number of shared beacons, number of beacons in a pair)
// = 12! / (2! * (12 - 2)!)
// = 12! / (2 * 10!)
// = 11 * 12 / 2
// = 66

const MIN_SHARED_SIGNATURES: usize = 66;

fn overlaps(
    a: &BTreeMap<BeaconSignature, (usize, usize)>,
    b: &BTreeMap<BeaconSignature, (usize, usize)>,
) -> Option<Vec<((usize, usize), (usize, usize))>> {
    let mut overlaps = Vec::new();
    for (kb, vb) in b {
        if let Some(va) = a.get(kb) {
            overlaps.push((*va, *vb))
        }
    }
    (overlaps.len() >= MIN_SHARED_SIGNATURES).then(|| overlaps)
}

fn align(
    unaligned: &[Vec3i],
    align_to: &[Vec3i],
    overlapping: &[((usize, usize), (usize, usize))],
) -> Option<(Vec3i, Vec<Vec3i>)> {
    for rotation in &Vec3i::ROTATIONS {
        for ((ai, aj), (uii, uji)) in overlapping {
            let ai = &align_to[*ai];
            let aj = &align_to[*aj];
            let ui = unaligned[*uii] * rotation;
            let uj = unaligned[*uji] * rotation;
            if ai - ui == aj - uj {
                let translation = ai - ui;

                let aligned = unaligned
                    .iter()
                    .map(|v| v * rotation + translation)
                    .collect::<Vec<_>>();
                if verify_alignment(&aligned, align_to, overlapping) {
                    return Some((translation, aligned));
                }
            }
        }
    }
    None
}

pub fn find_overlapping(
    scanner_signatures: &[BTreeMap<BeaconSignature, (usize, usize)>],
    to_test: usize,
    skip: &BTreeSet<usize>,
) -> Vec<(usize, Vec<((usize, usize), (usize, usize))>)> {
    let to_test = &scanner_signatures[to_test];
    scanner_signatures
        .iter()
        .enumerate()
        .filter(|(i, _)| !skip.contains(i))
        .filter_map(|(i, signature)| {
            overlaps(to_test, signature).map(|overlapping_pairs| (i, overlapping_pairs))
        })
        .collect()
}

fn verify_alignment(
    scanner: &[Vec3i],
    align_to: &[Vec3i],
    overlapping: &[((usize, usize), (usize, usize))],
) -> bool {
    let mut i = 0;
    for ((ai, aj), (bi, bj)) in overlapping {
        let ai = &align_to[*ai];
        let aj = &align_to[*aj];
        let bi = &scanner[*bi];
        let bj = &scanner[*bj];
        if (ai == bi && aj == bj) || (ai == bj && aj == bi) {
            i += 1;
        }
    }
    i >= MIN_SHARED_SIGNATURES
}

fn align_scanners(scanners: &[Vec<Vec3i>]) -> Vec<(Vec3i, Vec<Vec3i>)> {
    let (first, remaining) = scanners.split_first().unwrap();
    let mut unaligned_scanners = remaining
        .iter()
        .map(|s| (s.to_vec(), beacon_signatures(s)))
        .collect::<Vec<_>>();

    let first = first.to_vec();
    let first_signature = beacon_signatures(&first);
    let mut result = vec![(Vec3i::ZERO, first.clone())];
    let mut aligned_scanners = vec![(first, first_signature)];
    while !unaligned_scanners.is_empty() {
        while let Some((aligned_scanner, aligned_signature)) = aligned_scanners.pop() {
            let mut u = 0;
            while u < unaligned_scanners.len() {
                let (unaligned_scanner, unaligned_signature) = &unaligned_scanners[u];
                if let Some(pairs) = overlaps(&aligned_signature, unaligned_signature) {
                    let (scanner_position, aligned) =
                        align(unaligned_scanner, &aligned_scanner, &pairs)
                            .expect("Failed to align beacons");
                    let (_, unaligned_signature) = unaligned_scanners.swap_remove(u);
                    result.push((scanner_position, aligned.clone()));
                    aligned_scanners.push((aligned, unaligned_signature));
                } else {
                    u += 1;
                }
            }
        }
    }
    result
}

mod part1 {
    use crate::*;
    use std::collections::HashSet;

    pub fn solve(scanners: &[Vec<Vec3i>]) -> usize {
        let all_beacons = align_scanners(scanners)
            .into_iter()
            .flat_map(|(_, v)| v.into_iter())
            .collect::<HashSet<_>>();
        all_beacons.len()
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        let scanners = parse(Input::from_readable(INPUT));
        assert_eq!(79, solve(&scanners));
    }
}

mod part2 {
    use crate::*;

    pub fn solve(scanners: &[Vec<Vec3i>]) -> i32 {
        let aligned_scanners = align_scanners(scanners);

        let mut m = 0;
        for i in 0..aligned_scanners.len() - 1 {
            let (a, _) = aligned_scanners[i];
            for (b, _) in aligned_scanners.iter().skip(i) {
                m = m.max((b - a).manhattan());
            }
        }
        m
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        let scanners = parse(Input::from_readable(INPUT));
        assert_eq!(3621, solve(&scanners));
    }
}
