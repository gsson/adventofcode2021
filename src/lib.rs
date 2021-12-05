mod input;

pub use input::*;

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
    v: T,
}

pub struct BitIterator<T> {
    v: T,
}

macro_rules! iteratorext_impl {
    ($SelfT: ty) => {
        impl BitOrAggregate for $SelfT {
            fn bitor<I>(iter: I) -> Self
            where
                I: Iterator<Item = Self>,
            {
                iter.fold(0, std::ops::BitOr::bitor)
            }
        }

        impl<'a> BitOrAggregate<&'a $SelfT> for $SelfT {
            fn bitor<I>(iter: I) -> Self
            where
                I: Iterator<Item = &'a Self>,
            {
                iter.copied().bitor()
            }
        }
    };
}

macro_rules! bititerate_impl {
    ($SelfT:ty) => {
        impl Bits for $SelfT {
            fn bit_indices(self) -> BitIndexIterator<$SelfT> {
                BitIndexIterator { v: self }
            }

            fn bits(self) -> BitIterator<$SelfT> {
                BitIterator { v: self }
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
    };
}

bititerate_impl!(u32);
bititerate_impl!(u128);
iteratorext_impl!(u32);
