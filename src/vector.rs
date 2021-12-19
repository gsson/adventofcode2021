use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

pub trait Dot<Rhs = Self> {
    type Output;
    fn dot(self, rhs: Rhs) -> Self::Output;
}

macro_rules! binary_ref_ops {
    ($SelfT:ty, $OtherT:ty, $imp:ident, $method:ident) => {
        impl<'a> $imp<$OtherT> for &'a $SelfT {
            type Output = <$SelfT as $imp<$OtherT>>::Output;

            #[inline]
            fn $method(self, other: $OtherT) -> <$SelfT as $imp<$OtherT>>::Output {
                $imp::$method(*self, other)
            }
        }
        impl<'a> $imp<&'a $OtherT> for $SelfT {
            type Output = <$SelfT as $imp<$OtherT>>::Output;

            #[inline]
            fn $method(self, other: &'a $OtherT) -> <$SelfT as $imp<$OtherT>>::Output {
                $imp::$method(self, *other)
            }
        }
        impl<'a> $imp<&'a $OtherT> for &'a $SelfT {
            type Output = <$SelfT as $imp<$OtherT>>::Output;

            #[inline]
            fn $method(self, other: &'a $OtherT) -> <$SelfT as $imp<$OtherT>>::Output {
                $imp::$method(*self, *other)
            }
        }
    };
}

macro_rules! assign_ref_ops {
    ($SelfT:ty, $OtherT:ty, $imp:ident, $method:ident) => {
        impl $imp<&$OtherT> for $SelfT {
            #[inline]
            fn $method(&mut self, other: &$OtherT) {
                $imp::$method(self, *other)
            }
        }
    };
}

macro_rules! vec2_impl {
    ($SelfT:ident, $MatrixT:ident, $comp:ty, $neg_one:literal, $zero:literal, $one:literal, $($derive:ident),*) => {

        // NOTE: Stored in column major order
        #[derive(Debug, Copy, Clone, $($derive),*)]
        pub struct $MatrixT(pub $comp, pub $comp, pub $comp, pub $comp);
        impl $MatrixT {
            #[inline]
            pub const fn row_major(r1c1: $comp, r1c2: $comp, r2c1: $comp, r2c2: $comp) -> Self {
                $MatrixT(r1c1, r2c1, r1c2, r2c2)
            }

            #[inline]
            pub const fn col_major(r1c1: $comp, r2c1: $comp, r1c2: $comp, r2c2: $comp) -> Self {
                $MatrixT(r1c1, r2c1, r1c2, r2c2)
            }
        }
        impl From<[[$comp; 2]; 2]> for $MatrixT {
            #[inline]
            fn from(v: [[$comp; 2]; 2]) -> Self {
                Self(
                    v[0][0], v[1][0],
                    v[0][1], v[1][1],
                )
            }
        }

        #[derive(Debug, Copy, Clone, $($derive),*)]
        pub struct $SelfT(pub $comp, pub $comp);
        impl $SelfT {
            pub const MAX: $SelfT = $SelfT(<$comp>::MAX, <$comp>::MAX);
            pub const MIN: $SelfT = $SelfT(<$comp>::MIN, <$comp>::MIN);
            pub const ZERO: $SelfT = $SelfT($zero, $zero);
            pub const IDENTITY: $MatrixT = $MatrixT::row_major($one, $zero, $zero, $one);
            pub const ROTATIONS: [$MatrixT; 4] = [
                $MatrixT::row_major($one, $zero, $zero, $one),
                $MatrixT::row_major($zero, $neg_one, $one, $zero),
                $MatrixT::row_major($neg_one, $zero, $zero, $neg_one),
                $MatrixT::row_major($zero, $one, $neg_one, $zero),
            ];
        }

        impl From<($comp, $comp)> for $SelfT {
            #[inline]
            fn from((c0, c1): ($comp, $comp)) -> $SelfT {
                Self(c0, c1)
            }
        }

        impl Add<$comp> for $SelfT {
            type Output = $SelfT;

            #[inline]
            fn add(self, rhs: $comp) -> $SelfT {
                $SelfT(self.0 + rhs, self.1 + rhs)
            }
        }

        impl Add<$SelfT> for $SelfT {
            type Output = $SelfT;

            #[inline]
            fn add(self, rhs: $SelfT) -> $SelfT {
                $SelfT(self.0 + rhs.0, self.1 + rhs.1)
            }
        }

        binary_ref_ops!($SelfT, $comp, Add, add);
        binary_ref_ops!($SelfT, $SelfT, Add, add);

        impl AddAssign<$comp> for $SelfT {
            #[inline]
            fn add_assign(&mut self, rhs: $comp) {
                self.0 += rhs;
                self.1 += rhs;
            }
        }

        impl AddAssign<$SelfT> for $SelfT {
            #[inline]
            fn add_assign(&mut self, rhs: $SelfT) {
                self.0 += rhs.0;
                self.1 += rhs.1;
            }
        }

        assign_ref_ops!($SelfT, $comp, AddAssign, add_assign);
        assign_ref_ops!($SelfT, $SelfT, AddAssign, add_assign);

        impl Sub<$comp> for $SelfT {
            type Output = $SelfT;

            #[inline]
            fn sub(self, rhs: $comp) -> $SelfT {
                $SelfT(self.0 - rhs, self.1 - rhs)
            }
        }

        impl Sub<$SelfT> for $SelfT {
            type Output = $SelfT;

            #[inline]
            fn sub(self, rhs: $SelfT) -> $SelfT {
                $SelfT(self.0 - rhs.0, self.1 - rhs.1)
            }
        }

        binary_ref_ops!($SelfT, $comp, Sub, sub);
        binary_ref_ops!($SelfT, $SelfT, Sub, sub);

        impl SubAssign<$comp> for $SelfT {
            #[inline]
            fn sub_assign(&mut self, rhs: $comp) {
                self.0 -= rhs;
                self.1 -= rhs;
            }
        }

        impl SubAssign<$SelfT> for $SelfT {
            #[inline]
            fn sub_assign(&mut self, rhs: $SelfT) {
                self.0 -= rhs.0;
                self.1 -= rhs.1;
            }
        }

        assign_ref_ops!($SelfT, $comp, SubAssign, sub_assign);
        assign_ref_ops!($SelfT, $SelfT, SubAssign, sub_assign);

        impl Dot<$SelfT> for $SelfT {
            type Output = $comp;

            #[inline]
            fn dot(self, rhs: $SelfT) -> $comp {
                self.0 * rhs.0 + self.1 * rhs.1
            }
        }
        binary_ref_ops!($SelfT, $SelfT, Dot, dot);

        impl Mul<$comp> for $SelfT {
            type Output = $SelfT;

            #[inline]
            fn mul(self, rhs: $comp) -> $SelfT {
                $SelfT(self.0 * rhs, self.1 * rhs)
            }
        }

        binary_ref_ops!($SelfT, $comp, Mul, mul);

        impl Mul<$MatrixT> for $SelfT {
            type Output = $SelfT;

            #[inline]
            fn mul(self, rhs: $MatrixT) -> $SelfT {
                $SelfT(self.0 * rhs.0 + self.1 * rhs.1, self.0 * rhs.2 + self.1 * rhs.3)
            }
        }

        binary_ref_ops!($SelfT, $MatrixT, Mul, mul);

        impl Neg for $SelfT {
            type Output = $SelfT;
            #[inline]
            fn neg(self) -> Self::Output {
                $SelfT(-self.0, -self.1)
            }
        }

        impl Neg for &$SelfT {
            type Output = $SelfT;
            #[inline]
            fn neg(self) -> Self::Output {
                $SelfT(-self.0, -self.1)
            }
        }
    };
}

vec2_impl!(Vec2f, Matrix2f, f32, -1.0f32, 0.0f32, 1.0f32, PartialOrd, PartialEq);
vec2_impl!(Vec2i, Matrix2i, i32, -1i32, 0i32, 1i32, Hash, Ord, PartialOrd, Eq, PartialEq);

impl Vec2i {
    pub const LEFT: Vec2i = Vec2i(-1, 0);
    pub const RIGHT: Vec2i = Vec2i(1, 0);
    pub const UP: Vec2i = Vec2i(0, -1);
    pub const DOWN: Vec2i = Vec2i(0, 1);

    pub fn checked_translate(self, rhs: &Vec2i) -> Option<Self> {
        self.0
            .checked_add(rhs.0)
            .zip(self.1.checked_add(rhs.1))
            .map(|(c0, c1)| Vec2i(c0, c1))
    }
    pub fn to_index(self, width: usize) -> usize {
        self.0 as usize + width * self.1 as usize
    }
}
