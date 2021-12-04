use std::fmt::{Debug};
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, ErrorKind, Lines};
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
        Self {
            input
        }
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
    pub fn lines(self) -> Delimited<R, [u8; 1]> {
        Delimited {
            delimiter: [b'\n'],
            input: self.input,
        }
    }

    pub fn sections(self) -> Delimited<R, [u8; 2]> {
        Delimited {
            delimiter: [b'\n', b'\n'],
            input: self.input,
        }
    }

    pub fn comma_separated(self) -> Delimited<R, [u8; 1]> {
        Delimited {
            delimiter: [b','],
            input: self.input,
        }
    }

    pub fn words(self) -> Words<R> {
        Words {
            input: self.input
        }
    }

    pub fn parse<T>(self) -> T
        where
            T: FromStr,
            T::Err: Debug {
        self.into_string().parse().unwrap()
    }
}

#[inline]
fn read_until(input: &mut impl std::io::BufRead, delimiter: u8, buf: &mut Vec<u8>) -> bool {
    match input.read_until(delimiter, buf) {
        Ok(n) => n > 0,
        Err(error) => panic!("Read failed: {:?}", error)
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
            if let Some(start_of_word) = available.iter()
                .position(|b| !b.is_ascii_whitespace()) {
                let from_start = &available[start_of_word..];
                if let Some(word_length) = from_start.iter()
                    .position(|b| b.is_ascii_whitespace()) {
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
            return if buf.is_empty() {
                None
            } else {
                Some(buf)
            };
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
        read_word(&mut self.input)
            .map(|vec| Input::new(Cursor::new(vec)))
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

pub fn open(p: impl Into<std::path::PathBuf>) -> std::io::Result<Lines<std::io::BufReader<File>>> {
    let f = File::open(p.into())?;
    let r = std::io::BufReader::new(f);
    Ok(r.lines())
}

pub struct ParseIter<I, T> {
    tokens: I,
    _t: PhantomData<T>,
}

impl<I, E, T> Iterator for ParseIter<I, T> where
    I: Iterator<Item=BufInput>,
    E: Debug,
    T: FromStr<Err=E> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.tokens.next()?.parse::<T>())
    }
}

pub trait BitOrAggregate<B = Self> {
    fn bitor<I>(iter: I) -> Self
    where
    I: Iterator<Item = B>;
}

pub trait IteratorExt: Iterator {
    fn bitor<T>(self) -> T
        where
            T: BitOrAggregate<Self::Item>,
            Self: Sized,
    {
        T::bitor(self)
    }
}

impl<I: Iterator> IteratorExt for I {}

pub trait Bits: Sized {
    fn bit_indices(self) -> BitIndexIterator<Self>;
    fn bits(self) -> BitIterator<Self>;
    fn push_lsb(self, one: bool) -> Self;
    fn highest_one_bit(self) -> Self;
}

pub struct BitIndexIterator<T> {
    v: T
}

pub struct BitIterator<T> {
    v: T
}

macro_rules! iteratorext_impl {
    ($SelfT: ty) => {
        impl BitOrAggregate for $SelfT {
            fn bitor<I>(iter: I) -> Self where I: Iterator<Item=Self> {
                iter.fold(0, std::ops::BitOr::bitor)
            }
        }

        impl <'a> BitOrAggregate<&'a $SelfT> for $SelfT {
            fn bitor<I>(iter: I) -> Self where I: Iterator<Item=&'a Self> {
                iter.copied().bitor()
            }
        }
    }

}

macro_rules! bititerate_impl {
    ($SelfT:ty) => {
        impl Bits for $SelfT {
            fn bit_indices(self) -> BitIndexIterator<$SelfT> {
                BitIndexIterator {
                    v: self
                }
            }

            fn bits(self) -> BitIterator<$SelfT> {
                BitIterator {
                    v: self
                }
            }
            fn push_lsb(self, one: bool) -> $SelfT {
                (self << 1) | (one as $SelfT)
            }

            fn highest_one_bit(self) -> $SelfT {
                const MAX_BIT: $SelfT = 1 << (<$SelfT>::BITS - 1);
                self & (MAX_BIT >> self.leading_zeros())
            }
        }

        impl Iterator for BitIndexIterator<$SelfT> {
            type Item = u32;

            fn next(&mut self) -> Option<Self::Item> {
                if self.v == 0 {
                    None
                } else {
                    let bit_index = self.v.trailing_zeros();
                    let bit = (1 as $SelfT) << bit_index;
                    self.v ^= bit;
                    Some(bit_index)
                }
            }
        }
        impl Iterator for BitIterator<$SelfT> {
            type Item = $SelfT;

            fn next(&mut self) -> Option<Self::Item> {
                if self.v == 0 {
                    None
                } else {
                    let bit_index = self.v.trailing_zeros();
                    let bit = (1 as $SelfT) << bit_index;
                    self.v ^= bit;
                    Some(bit)
                }
            }
        }
    }
}

bititerate_impl!(u32);
iteratorext_impl!(u32);
