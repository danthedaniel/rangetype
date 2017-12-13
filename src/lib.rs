#[macro_use]
extern crate static_assertions;

#[macro_use]
mod macros;
#[cfg(test)]
mod tests;

use std::fmt::{Debug, Display, Formatter, Error};
use std::ops::{Range, Add, Mul, Neg, Sub, Div};
use std::cmp::Ordering;

/// A range-checked number type.
#[derive(Clone, PartialEq, Eq)]
pub struct RangeType<T> {
    val: T,
    range: Range<T>
}

impl<T: PartialOrd + Display> RangeType<T> {
    /// Create a new RangeType.
    ///
    /// # Arguments:
    /// * `val` - The value for the type.
    /// * `range` - The range between which the value must stay.
    pub fn new(val: T, range: Range<T>) -> RangeType<T> {
        if val < range.start || val > range.end {
            panic!("{} is not in the range {}..{}", val, range.start, range.end);
        }
        RangeType { val: val, range: range }
    }

    /// Yield the RangeType's internal value.
    pub fn as_raw(self) -> T {
        self.val
    }

    /// Convert to a RangeType with a different range.
    ///
    /// # Arguments:
    /// * `range` - The new range.
    pub fn with_range(self, range: Range<T>) -> RangeType<T> {
        RangeType::new(self.val, range)
    }
}

impl<T: Ord> Ord for RangeType<T> {
    fn cmp(&self, other: &RangeType<T>) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl<T: Ord> PartialOrd for RangeType<T> {
    fn partial_cmp(&self, other: &RangeType<T>) -> Option<Ordering> {
        Some(self.val.cmp(&other.val))
    }
}

impl<T: Debug> Debug for RangeType<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?} ", self.val)
    }
}

impl<T: Display> Display for RangeType<T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.val)
    }
}

impl<T: Neg<Output=T> + PartialOrd + Display> Neg for RangeType<T> {
    type Output = RangeType<T>;

    fn neg(self) -> RangeType<T> {
        RangeType::new(-self.val, self.range)
    }
}

/// Implement a binary operator for RangeType.
macro_rules! impl_range_op {
    ($trait:ident, $method:ident, $op:tt) => {
        impl<T: $trait<Output=T> + PartialOrd + Display> $trait for RangeType<T> {
            type Output = RangeType<T>;

            fn $method(self, other: RangeType<T>) -> RangeType<T> {
                if self.range != other.range {
                    panic!("Ranges are unequal");
                }
                RangeType::new(self.val $op other.val, self.range)
            }
        }
    }
}

impl_range_op!(Add, add, +);
impl_range_op!(Mul, mul, *);
impl_range_op!(Sub, sub, -);
impl_range_op!(Div, div, /);
