use std::hash::{Hash, Hasher};
use crate::axis::Axis;
use crate::biaxial::BiAxial;

// TODO: Document
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ContentFill {
    /// Minimum intrinsic size.
    /// Fit as small as possible on the specified axis. Shrink to the minimum wrappable component.
    Min,
    /// Maximum intrinsic size.
    /// Take up as much space as the content allows on the specified axis without wrapping.
    Max,
    /// Expand as desired up to the constraints on that axis, then wrap.
    Constrain(f64),
    /// The maximum size the widget can expand to (max width or height).
    /// Can return infinity if a child can expand infinitely. Will return the max
    /// size if known, or will compute the max intrinsic size.
    /// Can be replaced with a way to retrieve style info from children.
    MaxStretch,
}

impl ContentFill {
    pub fn shrink(&self, amount: f64) -> Self {
        match self {
            ContentFill::Min | ContentFill::Max | ContentFill::MaxStretch => {
                *self
            }
            ContentFill::Constrain(original_constraint) => {
                ContentFill::Constrain((original_constraint - amount).max(0.0))
            }
        }
    }
}

impl Hash for ContentFill {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            ContentFill::Max => {
                state.write_u8(1);
            }
            ContentFill::Min => {
                state.write_u8(2);
            }
            ContentFill::MaxStretch => {
                state.write_u8(3);
            }

            ContentFill::Constrain(constraint) => {
                state.write_u8(4);
                state.write_u64(constraint.to_bits())
            }
        }
    }
}

impl Eq for ContentFill {

}

impl BiAxial<ContentFill> {
    pub fn both_axes_constrained(&self) -> bool {
        match (self.horizontal, self.vertical) {
            (ContentFill::Constrain(_), ContentFill::Constrain(_)) => true,
            _ => false,
        }
    }

    pub fn horizontal_constrained(&self) -> bool {
        match self.horizontal {
            ContentFill::Constrain(_) => true,
            _ => false,
        }
    }

    pub fn constrain_aspect_ratio(&self, aspect_ratio: f64, axis: Axis) -> Option<f64> {
        match (self.horizontal, self.vertical, axis) {
            (ContentFill::Constrain(h), ContentFill::Constrain(v), axis) => {
                let constraint_ratio = h / v;
                if constraint_ratio > aspect_ratio { // limited by height
                    if axis == Axis::Vertical {
                        Some(h)
                    } else {
                        Some(aspect_ratio.recip() * h)
                    }
                } else {
                    if axis == Axis::Horizontal {
                        Some(h)
                    } else {
                        Some(aspect_ratio * v)
                    }
                }
            },
            (ContentFill::Constrain(h), _, Axis::Horizontal) => Some(h),
            (_, ContentFill::Constrain(v), Axis::Vertical) => Some(v),
            _ => None,
        }
    }

    pub fn shrink_constraints(&self, shrink_amount: &BiAxial<f64>) -> Self {
        let horizontal = self.horizontal.shrink(shrink_amount.horizontal);
        let vertical = self.vertical.shrink(shrink_amount.vertical);
        BiAxial::new(horizontal, vertical)
    }
}