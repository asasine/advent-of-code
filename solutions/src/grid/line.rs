//! [`Line`] forms a line segment between two points in a 2D plane.

use super::Coordinate;

/// A line segment between two points in a 2D plane.
#[derive(Debug, Clone, Copy)]
pub struct Line(pub Coordinate, pub Coordinate);

impl From<Line> for (Coordinate, Coordinate) {
    fn from(line: Line) -> Self {
        (line.0, line.1)
    }
}

impl<'a> From<&'a Line> for (&'a Coordinate, &'a Coordinate) {
    fn from(line: &'a Line) -> (&'a Coordinate, &'a Coordinate) {
        (&line.0, &line.1)
    }
}

impl From<(Coordinate, Coordinate)> for Line {
    fn from((a, b): (Coordinate, Coordinate)) -> Self {
        Line(a, b)
    }
}
