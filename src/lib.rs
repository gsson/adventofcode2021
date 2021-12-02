use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::marker::PhantomData;
use std::path::Path;
use std::str::FromStr;

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
    fn parse<T>(self) -> ParseIter<Self, T>;
}

impl <R: BufRead> TokenParse for InputLines<R> {
    fn parse<T>(self) -> ParseIter<Self, T> {
        ParseIter {
            tokens: self,
            _t: Default::default()
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

pub struct ParseIter<I, T> {
    tokens: I,
    _t: PhantomData<T>
}

impl <I: Iterator<Item=String>, E: Debug, T: FromStr<Err=E>> Iterator for ParseIter<I, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.tokens.next()?.parse::<T>().unwrap())
    }
}