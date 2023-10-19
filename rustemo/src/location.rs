use std::fmt::{Debug, Display};

/// A line-column based location for use where applicable (e.g. plain text).
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct LineColumn {
    pub line: usize,
    pub column: usize,
}

/// A position in the input file.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Position {
    Position(usize),
    LineBased(LineColumn),
}

impl Position {
    pub fn from_lc(line: usize, column: usize) -> Self {
        Self::LineBased(LineColumn { line, column })
    }

    pub fn from_pos(pos: usize) -> Self {
        Self::Position(pos)
    }

    #[inline]
    pub fn line(&self) -> usize {
        match self {
            Position::Position(pos) => *pos,
            Position::LineBased(lb) => lb.line,
        }
    }

    #[inline]
    pub fn column(&self) -> usize {
        match self {
            Position::Position(_) => 0,
            Position::LineBased(lb) => lb.column,
        }
    }

    #[inline]
    pub fn position(&self) -> usize {
        match self {
            Position::Position(pos) => *pos,
            Position::LineBased(lb) => lb.line,
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::Position(0)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Position::Position(pos) => write!(f, "{pos}"),
            Position::LineBased(lb) => write!(f, "{},{}", lb.line, lb.column),
        }
    }
}

/// Describes a span from start till end in the parsed input.
///
/// Start is mandatory while the end is not.
///
/// The location doesn't keep the path of the parsed file on purpose as it will
/// be the same for the single parse tree so it would either unnccessary waste
/// memory, in case of owned strings, or propagate lifetime information throught
/// the API in case of borrowed string slice.
///
/// The path is kept on the parsing context and there is the method on the
/// context to produce the display of the location with the full file path.
#[derive(PartialEq, Eq, Clone, Copy, Default)]
pub struct Location {
    /// The start position of the range.
    pub start: Position,
    /// The end position. Sometimes it is not known or relevant.
    pub end: Option<Position>,
}

impl Location {
    pub fn new(start: Position, end: Position) -> Self {
        Self {
            start,
            end: Some(end),
        }
    }
    pub fn from_start(start: Position) -> Self {
        Self { start, end: None }
    }
}

impl Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.end {
            Some(ref end) => write!(f, "[{}-{}]", self.start, end),
            None => write!(f, "[{}]", self.start),
        }
    }
}

// impl<'i, I, S, TK, C> From<C> for Location
// where
//     I: Input + ?Sized,
//     C: Context<'i, I, S, TK>
// {
//     fn from(context: &mut C) -> Self {
//         context.location()
//     }
// }

/// Value with location. Used in place of parsed values which need locations to
/// report errors during semantic analysis.
#[derive(Debug, Clone)]
pub struct ValLoc<T> {
    value: T,
    pub location: Option<Location>,
}

impl<T: Display> Display for ValLoc<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl<T> AsRef<T> for ValLoc<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}
impl<T> From<T> for ValLoc<T> {
    fn from(value: T) -> Self {
        Self {
            value,
            location: None,
        }
    }
}
impl From<ValLoc<String>> for String {
    fn from(value: ValLoc<String>) -> Self {
        value.value
    }
}
impl<T> ValLoc<T> {
    pub fn new(value: T, location: Option<Location>) -> Self {
        Self { value, location }
    }
}
macro_rules! from_valloc {
    ($type:ty) => {
        impl From<$crate::location::ValLoc<$type>> for $type {
            fn from(value: $crate::location::ValLoc<Self>) -> Self {
                value.value
            }
        }
        impl From<&$crate::location::ValLoc<$type>> for $type
          where $type: Copy
          {
            fn from(value: &$crate::location::ValLoc<Self>) -> Self {
                value.value
            }
        }
    };
    ($($type:ty),+) => {
        $(from_valloc!($type);)+
    };
}
// Implement value with location support for all primitive types.
from_valloc!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64,
    bool, char
);

#[cfg(test)]
mod tests {
    use super::{Location, Position};

    #[test]
    pub fn test_position_linebased() {
        let p = Position::from_lc(2, 4);

        assert_eq!(p.line(), 2);
        assert_eq!(p.column(), 4);
        assert_eq!(format!("{}", p), "2,4");
    }

    #[test]
    pub fn test_position() {
        let p = Position::from_pos(5);

        assert_eq!(p.line(), 5);
        assert_eq!(p.column(), 0);
        assert_eq!(p.position(), 5);
        assert_eq!(format!("{}", p), "5");
    }

    #[test]
    pub fn test_location() {
        let r =
            Location::new(Position::from_lc(5, 15), Position::from_lc(13, 27));

        assert_eq!(format!("{r:?}"), "[5,15-13,27]");

        let r = Location::new(Position::from_lc(5, 15), Position::from_pos(49));

        assert_eq!(format!("{r:?}"), "[5,15-49]");
    }
}
