use adventofcode2021::delimiters::SECTION;
use adventofcode2021::*;
use std::iter::repeat;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = Input::from_file("src/bin/day20/input.txt");
    let (enhancer, image) = parse(input);

    let a = part1::solve(&enhancer, &image);
    eprintln!("Part 1: {:?}", a);
    assert_eq!(5583, a);

    let a = part2::solve(&enhancer, &image);
    eprintln!("Part 2: {:?}", a);
    assert_eq!(19592, a);
    Ok(())
}

#[derive(Clone)]
pub struct Image {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Image {
    fn row_segment(&self, row: usize, col1: usize, col2: usize) -> &[u8] {
        &self.row(row)[col1..col2]
    }

    fn row(&self, row: usize) -> &[u8] {
        let start = row * self.width;
        let end = (row + 1) * self.width;
        &self.data[start..end]
    }

    fn enhance(&self, enhancer: &ImageEnhancer) -> Self {
        let mut data = Vec::with_capacity(self.data.len());
        let mut a = self.row(0);
        let mut b = self.row(1);
        data.extend(a);
        for y in 2..self.height {
            let c = self.row(y);
            data.push(b[0]);
            for x in 0..self.width - 2 {
                let t = (a[x] << 2) | (a[x + 1] << 1) | a[x + 2];
                let u = (b[x] << 2) | (b[x + 1] << 1) | b[x + 2];
                let v = (c[x] << 2) | (c[x + 1] << 1) | c[x + 2];
                let p = enhancer.pixel((t as i32) << 6 | (u as i32) << 3 | (v as i32));
                data.push(p);
            }
            data.push(b[self.width - 1]);

            a = b;
            b = c;
        }
        data.extend(b);

        Self {
            width: self.width,
            height: self.height,
            data,
        }
    }

    fn expand(&self, by: usize, pixel: u8) -> Self {
        let width = self.width + by * 2;
        let height = self.height + by * 2;
        let mut data = Vec::with_capacity(width * height);
        data.extend(repeat(pixel).take(width * by));

        for row in 0..self.height {
            data.extend(repeat(pixel).take(by));
            data.extend(self.row(row));
            data.extend(repeat(pixel).take(by));
        }
        data.extend(repeat(pixel).take(width * by));
        assert_eq!(data.len(), width * height);

        Self {
            width,
            height,
            data,
        }
    }
    fn shrink(&self, by: usize) -> Self {
        let width = self.width - by * 2;
        let height = self.height - by * 2;
        let mut data = Vec::with_capacity(width * height);
        for row in by..self.height - by {
            data.extend(self.row_segment(row, by, by + width));
        }
        assert_eq!(data.len(), width * height);

        Self {
            width,
            height,
            data,
        }
    }
}

pub struct ImageEnhancer([u64; 8]);
impl ImageEnhancer {
    pub fn pixel(&self, v: i32) -> u8 {
        let offset = v >> 6;
        let bit = 1u64 << (v & 0x3f);
        (self.0[offset as usize] & bit != 0) as u8
    }
}

fn parse<R: std::io::BufRead>(input: Input<R>) -> (ImageEnhancer, Image) {
    fn pixel(b: u8) -> u8 {
        match b {
            b'#' => 1,
            b'.' => 0,
            _ => unreachable!(),
        }
    }

    fn parse_algorithm<R: std::io::BufRead>(input: Input<R>) -> ImageEnhancer {
        let mut algorithm = [0u64; 8];
        for (i, b) in input.bytes().enumerate() {
            let b = pixel(b) as u64;
            let o = i >> 6;
            algorithm[o] |= b << (i & 0x3f);
        }
        ImageEnhancer(algorithm)
    }

    fn parse_image<R: std::io::BufRead>(input: Input<R>) -> Image {
        let (first, remaining) = input.delimited_once(delimiters::LINE);

        let mut data = first.bytes().map(pixel).collect::<Vec<u8>>();
        let width = data.len();
        for line in remaining.lines() {
            data.extend(line.bytes().map(pixel))
        }

        let height = data.len() / width;
        Image {
            width,
            height,
            data,
        }
    }

    let (algorithm, image) = input.delimited_once(SECTION);

    (parse_algorithm(algorithm), parse_image(image))
}

mod part1 {
    use crate::*;

    pub fn solve(enhancer: &ImageEnhancer, image: &Image) -> usize {
        let mut pixel = 0;
        let mut image = image.clone();
        for _ in 0..2 {
            image = image.expand(2, pixel).enhance(enhancer).shrink(1);
            pixel = enhancer.pixel(pixel as i32 * 511);
        }
        image.data.iter().map(|b| *b as usize).sum()
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        let (enhancer, image) = parse(Input::from_readable(INPUT));
        assert_eq!(35, solve(&enhancer, &image));
    }
}

mod part2 {
    use crate::*;

    pub fn solve(enhancer: &ImageEnhancer, image: &Image) -> usize {
        let mut pixel = 0;
        let mut image = image.clone();
        for _ in 0..50 {
            image = image.expand(2, pixel).enhance(enhancer).shrink(1);
            pixel = enhancer.pixel(pixel as i32 * 511);
        }
        image.data.iter().map(|b| *b as usize).sum()
    }

    #[test]
    fn test() {
        const INPUT: &[u8] = include_bytes!("test.txt");
        let (enhancer, image) = parse(Input::from_readable(INPUT));
        assert_eq!(3351, solve(&enhancer, &image));
    }
}
