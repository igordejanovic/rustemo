use std::{
    fmt::{Debug, Display},
    ops::Deref,
    ops::Range,
};

/// A line-column based location for use where applicable (e.g. plain text).
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
pub struct LineColumn {
    pub line: usize,
    pub column: usize,
}

/// A position in the input file.
#[derive(PartialEq, PartialOrd, Ord, Eq, Copy, Clone, Default)]
pub struct Position {
    /// Absolute position in bytes.
    pub pos: usize,
    /// Line-column position where applicable (e.g. textual inputs)
    pub line_col: Option<LineColumn>,
}

impl Position {
    pub fn new(pos: usize, line: usize, column: usize) -> Self {
        Self {
            pos,
            line_col: Some(LineColumn { line, column }),
        }
    }

    #[inline]
    pub fn line(&self) -> Option<usize> {
        Some(self.line_col?.line)
    }

    #[inline]
    pub fn column(&self) -> Option<usize> {
        Some(self.line_col?.column)
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(line_col) = self.line_col {
            write!(f, "{:?}({},{})", self.pos, line_col.line, line_col.column)
        } else {
            write!(f, "{:?}", self.pos)
        }
    }
}

impl From<usize> for Position {
    fn from(pos: usize) -> Self {
        Self {
            pos,
            line_col: None,
        }
    }
}

/// Describes a span from start till end in the parsed input.
///
/// The location doesn't keep the path of the parsed file on purpose as it will
/// be the same for the single parse tree so it would either unnccessary waste
/// memory, in case of owned strings, or propagate lifetime information throught
/// the API in case of borrowed string slice.
///
/// The path is kept on the parsing context and there is the method on the
/// context to produce the display of the location with the full file path.
#[derive(PartialEq, Eq, Clone, Copy, Default)]
pub struct SourceSpan {
    /// The start position of the span.
    pub start: Position,

    /// The end position of the span.
    pub end: Position,
}

impl SourceSpan {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
    /// Creates a new span which merges Self.start and span_other.end.
    pub fn merge(&self, span_other: Self) -> Self {
        Self {
            start: self.start,
            end: span_other.end,
        }
    }
}

impl From<Position> for SourceSpan {
    fn from(start: Position) -> Self {
        Self { start, end: start }
    }
}

impl From<SourceSpan> for Position {
    fn from(span: SourceSpan) -> Self {
        span.start
    }
}

impl From<(usize, usize)> for SourceSpan {
    fn from(value: (usize, usize)) -> Self {
        Self {
            start: Position {
                pos: value.0,
                line_col: None,
            },
            end: Position {
                pos: value.1,
                line_col: None,
            },
        }
    }
}

impl From<SourceSpan> for Range<usize> {
    fn from(span: SourceSpan) -> Self {
        Range {
            start: span.start.pos,
            end: span.end.pos,
        }
    }
}

impl Debug for SourceSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.start == self.end {
            write!(f, "{:?}", self.start)
        } else {
            write!(f, "[{:?}-{:?}]", self.start, self.end)
        }
    }
}

/// Value with span. Used in place of parsed values which need span to report
/// errors during semantic analysis.
#[derive(Debug, Clone)]
pub struct ValSpan<T> {
    value: T,
    pub span: Option<SourceSpan>,
}

impl<T: Display> Display for ValSpan<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> AsRef<T> for ValSpan<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T> From<T> for ValSpan<T> {
    fn from(value: T) -> Self {
        Self { value, span: None }
    }
}

impl<T> Deref for ValSpan<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<ValSpan<String>> for String {
    fn from(value: ValSpan<String>) -> Self {
        value.value
    }
}
impl<T> ValSpan<T> {
    pub fn new(value: T, location: Option<SourceSpan>) -> Self {
        Self {
            value,
            span: location,
        }
    }
}
macro_rules! from_valspan {
    ($type:ty) => {
        impl From<$crate::position::ValSpan<$type>> for $type {
            fn from(value: $crate::position::ValSpan<Self>) -> Self {
                value.value
            }
        }
        impl From<&$crate::position::ValSpan<$type>> for $type
          where $type: Copy
          {
            fn from(value: &$crate::position::ValSpan<Self>) -> Self {
                value.value
            }
        }
    };
    ($($type:ty),+) => {
        $(from_valspan!($type);)+
    };
}
// Implement value with location support for all primitive types.
from_valspan!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, bool, char
);

#[cfg(test)]
mod tests {
    use super::{Position, SourceSpan};

    #[test]
    pub fn test_position_linebased() {
        let p = Position::new(0, 2, 4);

        assert_eq!(p.line().unwrap(), 2);
        assert_eq!(p.column().unwrap(), 4);
        assert_eq!(format!("{p:?}"), "0(2,4)");
    }

    #[test]
    pub fn test_position() {
        let p: Position = 5.into();

        assert!(p.line().is_none());
        assert!(p.column().is_none());
        assert_eq!(p.pos, 5);
        assert_eq!(format!("{p:?}"), "5");
    }

    #[test]
    pub fn test_source_span() {
        let r = SourceSpan::new(Position::new(20, 5, 15), Position::new(40, 13, 27));

        assert_eq!(format!("{r:?}"), "[20(5,15)-40(13,27)]");

        let r = SourceSpan::new(Position::new(49, 5, 15), 70.into());

        assert_eq!(format!("{r:?}"), "[49(5,15)-70]");
    }
}
