use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

pub struct Input<I> {
    input: BufReader<I>,
}

impl <S: std::io::Read> Input<S> {
    pub fn from_readable(input: S) -> Self {
        Self {
            input: BufReader::new(input)
        }
    }
}
impl Input<File> {
    pub fn from_file(input: impl AsRef<Path>) -> Self {
        Self::from_readable(File::open(input.as_ref()).unwrap())
    }
}

impl <I: std::io::Read> Input<I> {
    pub fn lines(self) -> InputLines<BufReader<I>> {
        InputLines {
            input: self.input.lines()
        }
    }
}

pub struct InputLines<R> {
    input: Lines<R>,
}

pub trait TokenParse: Sized {
    fn numbers(self) -> NumberIter<Self>;
}

impl <R: BufRead> TokenParse for InputLines<R> {
    fn numbers(self) -> NumberIter<Self> {
        NumberIter {
            lines: self
        }
    }
}

impl <R: BufRead> Iterator for InputLines<R> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.input.next().map(|r| r.unwrap())
    }
}



pub fn open(p: impl Into<std::path::PathBuf>) -> std::io::Result<Lines<std::io::BufReader<File>>> {
    let f = File::open(p.into())?;
    let r = std::io::BufReader::new(f);
    Ok(r.lines())
}

pub struct NumberIter<I> {
    lines: I
}

impl <I: Iterator<Item=String>> Iterator for NumberIter<I> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.lines.next()?.parse().unwrap())
    }
}