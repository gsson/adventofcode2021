use adventofcode2021::vector::*;
use adventofcode2021::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Input::from_file("src/bin/day22/input.txt");
    let boxes = parse(input);

    let a = part1::solve(&boxes);
    eprintln!("Part 1: {:?}", a);
    assert_eq!(653798, a);

    let a = part2::solve(&boxes);
    eprintln!("Part 2: {:?}", a);
    assert_eq!(1257350313518866, a);
    Ok(())
}

#[inline]
fn axis_intersection(a_min: i32, a_max: i32, b_min: i32, b_max: i32) -> Option<(i32, i32)> {
    let min = a_min.max(b_min);
    let max = a_max.min(b_max);
    if min <= max {
        Some((min, max))
    } else {
        None
    }
}

#[test]
fn test_axis_intersection() {
    assert_eq!(Some((10, 20)), axis_intersection(10, 20, 10, 20));
    assert_eq!(Some((15, 15)), axis_intersection(15, 15, 10, 20));
    assert_eq!(Some((20, 20)), axis_intersection(10, 20, 20, 30));
    assert_eq!(None, axis_intersection(10, 20, 21, 30));
}

#[derive(Copy, Clone)]
pub struct AxisAlignedBox {
    value: isize,
    min: Vec3i,
    max: Vec3i,
}

impl AxisAlignedBox {
    pub const fn on(min: Vec3i, max: Vec3i) -> Self {
        AxisAlignedBox { value: 1, min, max }
    }
    pub const fn off(min: Vec3i, max: Vec3i) -> Self {
        AxisAlignedBox { value: 0, min, max }
    }

    pub fn intersect(&self, other: &AxisAlignedBox) -> Option<AxisAlignedBox> {
        axis_intersection(self.min.0, self.max.0, other.min.0, other.max.0)
            .zip(axis_intersection(
                self.min.1,
                self.max.1,
                other.min.1,
                other.max.1,
            ))
            .zip(axis_intersection(
                self.min.2,
                self.max.2,
                other.min.2,
                other.max.2,
            ))
            .map(|(((x1, x2), (y1, y2)), (z1, z2))| AxisAlignedBox {
                value: 0,
                min: Vec3i(x1, y1, z1),
                max: Vec3i(x2, y2, z2),
            })
    }

    #[inline]
    pub fn volume(&self) -> isize {
        let lengths = (self.max - self.min) + 1;
        (lengths.0 as isize) * (lengths.1 as isize) * (lengths.2 as isize)
    }
}

#[test]
fn test_volume() {
    let c = AxisAlignedBox {
        value: 0,
        min: Vec3i(0, 0, 0),
        max: Vec3i(0, 0, 0),
    };
    assert_eq!(1, c.volume());
    let c = AxisAlignedBox {
        value: 0,
        min: Vec3i(-1, -1, -1),
        max: Vec3i(0, 0, 0),
    };
    assert_eq!(8, c.volume());
    let c = AxisAlignedBox {
        value: 0,
        min: Vec3i(-1, -1, -1),
        max: Vec3i(1, 1, 1),
    };
    assert_eq!(27, c.volume())
}

fn parse<R: std::io::BufRead>(input: Input<R>) -> Vec<AxisAlignedBox> {
    fn parse_extent<R: std::io::BufRead>(extent: Input<R>) -> (i32, i32) {
        let (_axis, extent) = extent.delimited_once(b"=");
        let (low, high) = extent.delimited_once("..");
        (low.parse(), high.parse())
    }
    fn parse_box<R: std::io::BufRead>(input: Input<R>) -> AxisAlignedBox {
        let (state, extents) = input.delimited_once(" ");
        let value = match state.into_string().as_str() {
            "on" => 1,
            "off" => 0,
            _ => unreachable!(),
        };
        let (x_ext, yz_ext) = extents.delimited_once(",");
        let x_ext = parse_extent(x_ext);
        let (y_ext, z_ext) = yz_ext.delimited_once(",");
        let y_ext = parse_extent(y_ext);
        let z_ext = parse_extent(z_ext);
        let a = Vec3i(x_ext.0, y_ext.0, z_ext.0);
        let b = Vec3i(x_ext.1, y_ext.1, z_ext.1);
        AxisAlignedBox {
            value,
            min: a,
            max: b,
        }
    }
    input.lines().map(parse_box).collect()
}

fn intersections(first: AxisAlignedBox, boxes: &[AxisAlignedBox]) -> Vec<AxisAlignedBox> {
    let mut out = vec![first];
    match first.value {
        0 => {}
        -1 | 1 => {
            for i in 0..boxes.len() {
                let other = &boxes[i];
                if let Some(mut overlapping) = first.intersect(other) {
                    overlapping.value = -first.value;
                    out.extend(intersections(overlapping, &boxes[i + 1..]))
                }
            }
        }
        _ => unreachable!(),
    }

    out
}

// Worst case time and space O(n!)
// Could be space O(n) by summing while iterating instead of collecting all boxes.
pub fn reboot_reactor(boxes: &[AxisAlignedBox]) -> isize {
    let mut all = Vec::new();
    for i in 0..boxes.len() {
        all.extend(intersections(boxes[i], &boxes[i + 1..]))
    }

    all.iter().map(|c| c.value * c.volume()).sum::<isize>()
}

mod part1 {
    use crate::AxisAlignedBox;
    use adventofcode2021::vector::Vec3i;

    pub fn solve(boxes: &[AxisAlignedBox]) -> isize {
        let bounds = AxisAlignedBox {
            value: 0,
            min: Vec3i(-50, -50, -50),
            max: Vec3i(50, 50, 50),
        };
        let boxes = boxes
            .iter()
            .copied()
            .filter(|c| bounds.intersect(c).is_some())
            .collect::<Vec<_>>();
        super::reboot_reactor(&boxes)
    }

    #[test]
    fn test_intersect() {
        /*  +---------+
         *  |11x7     |
         *  | +-------+
         *  | |9x6    |
         *  | | +-----+
         *  | | |7x5  |
         *  +-| | +---+
         *    +-| |5x3|
         *      +-+---+
         *  77 + 9 + 7 = 93
         */
        const BOXES: &[AxisAlignedBox] = &[
            AxisAlignedBox::on(Vec3i(0, 0, 0), Vec3i(10, 6, 0)),
            AxisAlignedBox::on(Vec3i(2, 2, 0), Vec3i(10, 7, 0)),
            AxisAlignedBox::on(Vec3i(4, 4, 0), Vec3i(10, 8, 0)),
            AxisAlignedBox::on(Vec3i(6, 6, 0), Vec3i(10, 8, 0)),
        ];
        assert_eq!(93, solve(BOXES));
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test1.txt");
        let boxes = crate::parse(crate::Input::from_readable(INPUT));
        assert_eq!(590784, solve(&boxes));
    }
}

mod part2 {
    use crate::AxisAlignedBox;

    pub fn solve(boxes: &[AxisAlignedBox]) -> isize {
        super::reboot_reactor(boxes)
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test2.txt");
        let boxes = crate::parse(crate::Input::from_readable(INPUT));
        assert_eq!(2758514936282235, solve(&boxes));
    }
}
