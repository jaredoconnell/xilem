use crate::{BoxConstraints, Point, Rect, Size, Vec2};

/// An axis in visual space.
///
/// Most often used by widgets to describe the direction in which they grow
/// as their number of children increases.
/// Has some methods for manipulating geometry with respect to the axis.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Axis {
    /// The x axis
    Horizontal,
    /// The y axis
    Vertical,
}

impl Axis {
    /// Get the axis perpendicular to this one.
    pub fn cross(self) -> Axis {
        match self {
            Axis::Horizontal => Axis::Vertical,
            Axis::Vertical => Axis::Horizontal,
        }
    }

    /// Extract from the argument the magnitude along this axis
    pub fn major(self, size: Size) -> f64 {
        match self {
            Axis::Horizontal => size.width,
            Axis::Vertical => size.height,
        }
    }

    /// Extract from the argument the magnitude along the perpendicular axis
    pub fn minor(self, size: Size) -> f64 {
        self.cross().major(size)
    }

    /// Extract the extent of the argument in this axis as a pair.
    pub fn major_span(self, rect: Rect) -> (f64, f64) {
        match self {
            Axis::Horizontal => (rect.x0, rect.x1),
            Axis::Vertical => (rect.y0, rect.y1),
        }
    }

    /// Extract the extent of the argument in the minor axis as a pair.
    pub fn minor_span(self, rect: Rect) -> (f64, f64) {
        self.cross().major_span(rect)
    }

    /// Extract the coordinate locating the argument with respect to this axis.
    pub fn major_pos(self, pos: Point) -> f64 {
        match self {
            Axis::Horizontal => pos.x,
            Axis::Vertical => pos.y,
        }
    }

    /// Extract the coordinate locating the argument with respect to this axis.
    pub fn major_vec(self, vec: Vec2) -> f64 {
        match self {
            Axis::Horizontal => vec.x,
            Axis::Vertical => vec.y,
        }
    }

    /// Extract the coordinate locating the argument with respect to the perpendicular axis.
    pub fn minor_pos(self, pos: Point) -> f64 {
        self.cross().major_pos(pos)
    }

    /// Extract the coordinate locating the argument with respect to the perpendicular axis.
    pub fn minor_vec(self, vec: Vec2) -> f64 {
        self.cross().major_vec(vec)
    }

    // TODO - make_pos, make_size, make_rect
    /// Arrange the major and minor measurements with respect to this axis such that it forms
    /// an (x, y) pair.
    pub fn pack(self, major: f64, minor: f64) -> (f64, f64) {
        match self {
            Axis::Horizontal => (major, minor),
            Axis::Vertical => (minor, major),
        }
    }

    /// Generate constraints with new values on the major axis.
    pub(crate) fn constraints(
        self,
        bc: &BoxConstraints,
        major: f64,
    ) -> BoxConstraints {
        match self {
            Axis::Horizontal => BoxConstraints::new(
                Size::new(major, bc.size().height),
            ),
            Axis::Vertical => BoxConstraints::new(
                Size::new(bc.size().width, major),
            ),
        }
    }
}
/*
impl Axis {
    /// Get the axis perpendicular to this one.
    pub fn cross(self) -> Axis {
        match self {
            Axis::Horizontal => Axis::Vertical,
            Axis::Vertical => Axis::Horizontal,
        }
    }

    /// Extract from the argument the magnitude along this axis
    pub fn major(self, size: Size) -> f64 {
        match self {
            Axis::Horizontal => size.width,
            Axis::Vertical => size.height,
        }
    }

    /// Extract from the argument the magnitude along the perpendicular axis
    pub fn minor(self, size: Size) -> f64 {
        self.cross().major(size)
    }

    /// Extract the extent of the argument in this axis as a pair.
    pub fn major_span(self, rect: Rect) -> (f64, f64) {
        match self {
            Axis::Horizontal => (rect.x0, rect.x1),
            Axis::Vertical => (rect.y0, rect.y1),
        }
    }

    /// Extract the extent of the argument in the minor axis as a pair.
    pub fn minor_span(self, rect: Rect) -> (f64, f64) {
        self.cross().major_span(rect)
    }

    /// Extract the coordinate locating the argument with respect to this axis.
    pub fn major_pos(self, pos: Point) -> f64 {
        match self {
            Axis::Horizontal => pos.x,
            Axis::Vertical => pos.y,
        }
    }

    /// Extract the coordinate locating the argument with respect to this axis.
    pub fn major_vec(self, vec: Vec2) -> f64 {
        match self {
            Axis::Horizontal => vec.x,
            Axis::Vertical => vec.y,
        }
    }

    /// Extract the coordinate locating the argument with respect to the perpendicular axis.
    pub fn minor_pos(self, pos: Point) -> f64 {
        self.cross().major_pos(pos)
    }

    /// Extract the coordinate locating the argument with respect to the perpendicular axis.
    pub fn minor_vec(self, vec: Vec2) -> f64 {
        self.cross().major_vec(vec)
    }

    // TODO - make_pos, make_size, make_rect
    /// Arrange the major and minor measurements with respect to this axis such that it forms
    /// an (x, y) pair.
    pub fn pack(self, major: f64, minor: f64) -> (f64, f64) {
        match self {
            Axis::Horizontal => (major, minor),
            Axis::Vertical => (minor, major),
        }
    }

    /// Generate constraints with new values on the major axis.
    pub(crate) fn constraints(
        self,
        bc: &BoxConstraints,
        major: f64,
    ) -> BoxConstraints {
        match self {
            Axis::Horizontal => BoxConstraints::new(
                Size::new(major, bc.size().height),
            ),
            Axis::Vertical => BoxConstraints::new(
                Size::new(bc.size().width, major),
            ),
        }
    }
}*/