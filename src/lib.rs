use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::marker::PhantomData;
use std::path::Path;
use std::str::FromStr;

pub struct Input<R> {
    input: R,
}

impl <S: std::io::Read> Input<BufReader<S>> {
    pub fn from_readable(input: S) -> Self {
        Self {
            input: BufReader::new(input)
        }
    }
}

impl Input<BufReader<File>> {
    pub fn from_file(input: impl AsRef<Path>) -> Self {
        Self::from_readable(File::open(input.as_ref()).unwrap())
    }
}

impl <R: std::io::BufRead> Input<R> {
    pub fn lines(self) -> InputLines<R> {
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
