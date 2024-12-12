// Copyright 2019 the Xilem Authors and the Druid Authors
// SPDX-License-Identifier: Apache-2.0

use vello::kurbo::Size;

/// Constraints for layout.
///
/// The layout strategy for Masonry is strongly inspired by Flutter,
/// and this struct is similar to the [Flutter BoxConstraints] class.
///
/// At the moment, it represents simply a minimum and maximum size.
/// A widget's [`layout`] method should choose an appropriate size that
/// meets these constraints.
///
/// Further, a container widget should compute appropriate constraints
/// for each of its child widgets, and pass those down when recursing.
///
/// The constraints are always [rounded away from zero] to integers
/// to enable pixel perfect layout.
///
/// [`layout`]: crate::widget::Widget::layout
/// [Flutter BoxConstraints]: https://api.flutter.dev/flutter/rendering/BoxConstraints-class.html
/// [rounded away from zero]: Size::expand
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoxConstraints {
    exact: Size,
}

impl BoxConstraints {

    /// Create a new box constraints object.
    ///
    /// Create constraints based on minimum and maximum size.
    ///
    /// The given sizes are also [rounded away from zero],
    /// so that the layout is aligned to integers.
    ///
    /// [rounded away from zero]: Size::expand
    pub fn new(exact: Size) -> BoxConstraints {
        BoxConstraints {
            exact: exact.expand(),
        }
    }

    /// Clamp a given size so that it fits within the constraints.
    ///
    /// The given size is also [rounded away from zero],
    /// so that the layout is aligned to integers.
    ///
    /// [rounded away from zero]: Size::expand
    pub fn constrain(&self, size: impl Into<Size>) -> Size {
        // TODO: Determine desired logic for this.
        // Size::new(0.0, 0.0),
        size.into().expand().clamp(self.exact, self.exact)
    }

    /// Returns the max size of these constraints.
    pub fn size(&self) -> Size {
        self.exact
    }

    /// Check to see if these constraints are legit.
    ///
    /// In Debug mode, logs a warning if `BoxConstraints` are invalid.
    pub fn debug_check(&self, name: &str) {
        if cfg!(not(debug_assertions)) {
            return;
        }

        if self.exact.width.is_nan() {
            debug_panic!("Width constraint passed to {name} is NaN");
        }
        if self.exact.height.is_nan() {
            debug_panic!("Height constraint passed to {name} is NaN");
        }
        if self.exact.width.is_infinite() {
            debug_panic!("Infinite width constraint passed to {name}");
        }
        if self.exact.height.is_infinite() {
            debug_panic!("Infinite height constraint passed to {name}");
        }
        if self.exact.width < 0.0 {
            debug_panic!("Negative width constraint passed to {name}");
        }
        if self.exact.height < 0.0 {
            debug_panic!("Negative height constraint passed to {name}");
        }

        if !(self.exact.expand() == self.exact)
        {
            debug_panic!("Unexpanded BoxConstraints passed to {name}: {self:?}",);
        }
    }

    /// Shrink constraints by size
    ///
    /// The given size is also [rounded away from zero],
    /// so that the layout is aligned to integers.
    ///
    /// [rounded away from zero]: Size::expand
    pub fn shrink(&self, diff: impl Into<Size>) -> BoxConstraints {
        let diff = diff.into().expand();
        let new_size = Size::new(
            (self.size().width - diff.width).max(0.),
            (self.size().height - diff.height).max(0.),
        );

        BoxConstraints::new(new_size)
    }

    /// Test whether these constraints contain the given `Size`.
    pub fn contains(&self, size: impl Into<Size>) -> bool {
        let size = size.into();
        (size.width <= self.exact.width)
            && (size.height <= self.exact.height)
    }

    /// Find the `Size` within these `BoxConstraint`s that minimises the difference between the
    /// returned `Size`'s aspect ratio and `aspect_ratio`, where *aspect ratio* is defined as
    /// `height / width`.
    ///
    /// If multiple `Size`s give the optimal `aspect_ratio`, then the one with the `width` nearest
    /// the supplied width will be used. Specifically, if `width == 0.0` then the smallest possible
    /// `Size` will be chosen, and likewise if `width == f64::INFINITY`, then the largest `Size`
    /// will be chosen.
    ///
    /// Use this function when maintaining an aspect ratio is more important than minimizing the
    /// distance between input and output size width and height.
    pub fn constrain_aspect_ratio(&self, aspect_ratio: f64, width: f64) -> Size {
        // Minimizing/maximizing based on aspect ratio seems complicated, but in reality everything
        // is linear, so the amount of work to do is low.
        let ideal_size = Size::new(width, width * aspect_ratio);

        // It may be possible to remove these in the future if the invariant is checked elsewhere.
        let aspect_ratio = aspect_ratio.abs();
        let width = width.abs();

        // Firstly check if we can simply return the exact requested
        if self.contains(ideal_size) {
            return ideal_size;
        }

        // Then we check if any `Size`s with our desired aspect ratio are inside the constraints.
        // TODO this currently outputs garbage when things are < 0 - See https://github.com/linebender/xilem/issues/377
        let max_w_min_h = 0.0;
        let max_w_max_h = self.exact.height / self.exact.width;

        // When the aspect ratio line crosses the constraints, the closest point must be one of the
        // two points where the aspect ratio enters/exits.

        // When the aspect ratio line doesn't intersect the box of possible sizes, the closest
        // point must be either (max width, min height) or (max height, min width). So all we have
        // to do is check which one of these has the closest aspect ratio.

        // Check each possible intersection (or not) of the aspect ratio line with the constraints
        if aspect_ratio < max_w_min_h {
            // outside min height max width
            Size::new(self.exact.width, 0.0)
        } else {
            // final case is where we hit constraints on the min height line
            if width < 0.0 {
                // take the point on the min height
                Size::new(0.0 * aspect_ratio.recip(), 0.0)
            } else if aspect_ratio > max_w_max_h {
                // exit thru max height
                Size::new(self.exact.height * aspect_ratio.recip(), self.exact.height)
            } else {
                // exit thru max width
                Size::new(self.exact.width, self.exact.width * aspect_ratio)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bc(max_width: f64, max_height: f64) -> BoxConstraints {
        BoxConstraints::new(
            Size::new(max_width, max_height),
        )
    }

    #[test]
    fn constrain_aspect_ratio() {
        for (bc, aspect_ratio, width, output) in [
            // The ideal size lies within the constraints
            (bc(100.0, 100.0), 1.0, 50.0, Size::new(50.0, 50.0)),
            (bc(90.0, 100.0), 1.0, 50.0, Size::new(50.0, 50.0)),
            // The correct aspect ratio is available (but not width)
            // min height
            (
                bc(100.0, 100.0),
                1.0,
                5.0,
                Size::new(10.0, 10.0),
            ),
            (
                bc(60.0, 100.0),
                2.0,
                30.0,
                Size::new(45.0, 90.0),
            ),
            (
                bc(100.0, 100.0),
                0.5,
                5.0,
                Size::new(20.0, 10.0),
            ),
            // min width
            (
                bc(100.0, 100.0),
                2.0,
                5.0,
                Size::new(10.0, 20.0),
            ),
            (
                bc(100.0, 60.0),
                0.5,
                60.0,
                Size::new(90.0, 45.0),
            ),
            (
                bc(50.0, 100.0),
                1.0,
                100.0,
                Size::new(50.0, 50.0),
            ),
            // max height
            (
                bc(100.0, 100.0),
                2.0,
                105.0,
                Size::new(50.0, 100.0),
            ),
            (
                bc(100.0, 100.0),
                0.5,
                105.0,
                Size::new(100.0, 50.0),
            ),
            // The correct aspect ratio is not available
            (
                bc(40.0, 40.0),
                10.0,
                30.0,
                Size::new(20.0, 40.0),
            ),
            (bc(40.0, 40.0), 0.1, 30.0, Size::new(40.0, 20.0)),
            // non-finite
            (
                bc(50.0, f64::INFINITY),
                1.0,
                100.0,
                Size::new(50.0, 50.0),
            ),
        ]
        .iter()
        {
            assert_eq!(
                bc.constrain_aspect_ratio(*aspect_ratio, *width),
                *output,
                "bc:{bc:?}, aspect_ratio:{aspect_ratio}, width:{width}",
            );
        }
    }
}
