use std::fmt;
use std::fmt::Debug;
use crate::axis::Axis;
use crate::Size;

/// A type that stores generic data along two axes: horizontal and vertical.
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct BiAxial<T> {
    pub horizontal: T,
    pub vertical: T,
}

impl BiAxial<f64> {
    #[inline]
    pub const fn from_kurbo_size(size: Size) -> Self {
        BiAxial { horizontal: size.width, vertical: size.height }
    }
}

impl<T: Debug> fmt::Display for BiAxial<T> {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "BiAxial<{:?}, {:?}>", self.horizontal, self.vertical)
    }
}


impl<T> BiAxial<T> {
    /// Constructs a size (planar with f64 type)
    #[inline]
    pub const fn new(horizontal: T, vertical: T) -> Self {
        BiAxial { horizontal, vertical }
    }

    /// Extract the value for the given axis.
    pub fn value_for_axis(self, axis: Axis) -> T {
        match axis {
            Axis::Horizontal => self.horizontal,
            Axis::Vertical => self.vertical,
        }
    }

    /// Extract the value for the given axis.
    pub fn set_for_axis(self, axis: Axis, value: T) -> Self {
        let mut new_self = self;
        match axis {
            Axis::Horizontal => new_self.horizontal = value,
            Axis::Vertical => new_self.vertical = value,
        };
        new_self
    }

    /// Construct a new BiAxial given a major axis and values for the major and minor axes.
    pub fn new_by_axis(major: T, minor: T, axis: Axis) -> BiAxial<T> {
        match axis {
            Axis::Horizontal => BiAxial::new(major, minor),
            Axis::Vertical => BiAxial::new(minor, major),
        }
    }

    pub fn raw(self) -> (T, T) {
        return (self.horizontal, self.vertical)
    }
}