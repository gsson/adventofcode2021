use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, ErrorKind};
use std::marker::PhantomData;
use std::path::Path;
use std::str::FromStr;

pub struct Input<R> {
    input: R,
}

pub type BufInput = Input<Cursor<Vec<u8>>>;

impl<S: std::io::Read> Input<BufReader<S>> {
    pub fn from_readable(input: S) -> Self {
        Self::new(BufReader::new(input))
    }
}

impl<R: std::io::BufRead> Input<R> {
    pub fn new(input: R) -> Self {
        Self { input }
    }
    pub fn into_string(mut self) -> String {
        let mut s = String::new();
        self.input.read_to_string(&mut s).unwrap();
        s
    }
}

impl Input<BufReader<File>> {
    pub fn from_file(input: impl AsRef<Path>) -> Self {
        Self::from_readable(File::open(input.as_ref()).unwrap())
    }
}

impl<R: std::io::BufRead> Input<R> {
    pub fn delimited<D: AsRef<[u8]> + Sized>(self, delimiter: D) -> Delimited<R, D> {
        Delimited {
            delimiter,
            input: self.input,
        }
    }

    pub fn delimited_once<D: AsRef<[u8]> + Sized>(mut self, delimiter: D) -> (Input<Cursor<Vec<u8>>>, Self) {
        let first = read_delimited(&mut self.input, delimiter.as_ref()).unwrap();
        (Input::new(Cursor::new(first)), self)
    }

    #[inline]
    pub fn lines(self) -> Delimited<R, [u8; 1]> {
        self.delimited([b'\n'])
    }

    #[inline]
    pub fn sections(self) -> Delimited<R, [u8; 2]> {
        self.delimited([b'\n', b'\n'])
    }

    #[inline]
    pub fn comma_separated(self) -> Delimited<R, [u8; 1]> {
        self.delimited([b','])
    }

    pub fn words(self) -> Words<R> {
        Words { input: self.input }
    }

    pub fn parse<T>(self) -> T
    where
        T: FromStr,
        T::Err: Debug,
    {
        self.into_string().parse().unwrap()
    }
}

#[inline]
fn read_until(input: &mut impl std::io::BufRead, delimiter: u8, buf: &mut Vec<u8>) -> bool {
    match input.read_until(delimiter, buf) {
        Ok(n) => n > 0,
        Err(error) => panic!("Read failed: {:?}", error),
    }
}

fn read_delimited(input: &mut impl std::io::BufRead, delimiter: &[u8]) -> Option<Vec<u8>> {
    let last = *delimiter.last().unwrap();
    let mut buf = Vec::new();
    if !read_until(input, last, &mut buf) {
        return None;
    }

    loop {
        if buf.ends_with(delimiter) {
            buf.truncate(buf.len() - delimiter.len());
            return Some(buf);
        }
        if !read_until(input, last, &mut buf) {
            return Some(buf);
        }
    }
}

fn read_word(input: &mut impl std::io::BufRead) -> Option<Vec<u8>> {
    let mut buf = Vec::new();
    loop {
        let (done, used) = {
            let available = match input.fill_buf() {
                Ok(n) => n,
                Err(ref error) if error.kind() == ErrorKind::Interrupted => continue,
                Err(error) => panic!("Read failed: {:?}", error),
            };
            if let Some(start_of_word) = available.iter().position(|b| !b.is_ascii_whitespace()) {
                let from_start = &available[start_of_word..];
                if let Some(word_length) = from_start.iter().position(|b| b.is_ascii_whitespace()) {
                    buf.extend_from_slice(&from_start[..word_length]);
                    (true, start_of_word + word_length)
                } else {
                    buf.extend_from_slice(from_start);
                    (false, available.len())
                }
            } else {
                (false, available.len())
            }
        };
        input.consume(used);
        if done {
            return Some(buf);
        } else if used == 0 {
            return if buf.is_empty() { None } else { Some(buf) };
        }
    }
}

pub struct Delimited<R, D> {
    delimiter: D,
    input: R,
}

impl<R: std::io::BufRead, D: std::convert::AsRef<[u8]>> Iterator for Delimited<R, D> {
    type Item = BufInput;

    fn next(&mut self) -> Option<Self::Item> {
        read_delimited(&mut self.input, self.delimiter.as_ref())
            .map(|vec| Input::new(Cursor::new(vec)))
    }
}

pub struct Words<R> {
    input: R,
}

impl<R: std::io::BufRead> Iterator for Words<R> {
    type Item = BufInput;

    fn next(&mut self) -> Option<Self::Item> {
        read_word(&mut self.input).map(|vec| Input::new(Cursor::new(vec)))
    }
}

pub trait TokenParse: Sized {
    fn parse<T>(self) -> ParseIter<Self, T>;
}

impl<R: BufRead, D> TokenParse for Delimited<R, D> {
    fn parse<T>(self) -> ParseIter<Self, T> {
        ParseIter {
            tokens: self,
            _t: Default::default(),
        }
    }
}

impl<R: BufRead> TokenParse for Words<R> {
    fn parse<T>(self) -> ParseIter<Self, T> {
        ParseIter {
            tokens: self,
            _t: Default::default(),
        }
    }
}

pub struct ParseIter<I, T> {
    tokens: I,
    _t: PhantomData<T>,
}

impl<I, E, T> Iterator for ParseIter<I, T>
where
    I: Iterator<Item = BufInput>,
    E: Debug,
    T: FromStr<Err = E>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.tokens.next()?.parse::<T>())
    }
}
