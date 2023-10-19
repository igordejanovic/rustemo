use crate::{
    error::Result,
    location::{LineColumn, Location, Position},
};
use std::{
    borrow::ToOwned,
    cmp::min,
    iter::once,
    ops::{Deref, Index, Range},
    path::Path,
};
/// Input is a sliceable sequence-like type with a concept of length.
///
/// This trait must be implemented by all types that should be parsed by
/// Rustemo.
pub trait Input: ToOwned + Index<Range<usize>, Output = Self> {
    /// Returns a string context for the given position. Used in debugging outputs.
    fn context_str(&self, position: usize) -> String;

    /// Returns the length of the input.
    fn len(&self) -> usize;

    /// Determines if the input is an empty sequence.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Implement for types which may cause panic on slicing with full `Range`
    /// (e.g. `str`).
    #[inline]
    fn slice(
        &self,
        range: Range<usize>,
    ) -> &<Self as Index<Range<usize>>>::Output {
        &self[range]
    }

    /// Read the file from the given path into owned version of the input.
    fn read_file<P: AsRef<Path>>(path: P) -> Result<Self::Owned>;

    fn start_location() -> Location {
        Location {
            start: Position::Position(0),
            end: None,
        }
    }

    /// Given the current location returns the location at the end of self.
    /// Location is an input-specific concept. E.g. for text it is line/column.
    fn location_after(&self, location: Location) -> Location;

    /// Given the current location returns a span starting from the current
    /// location and extending over self.
    fn location_span(&self, location: Location) -> Location {
        Location {
            start: location.start,
            end: Some(self.location_after(location).start),
        }
    }
}

impl Input for str {
    fn context_str(&self, position: usize) -> String {
        self[..position]
            .chars()
            .rev()
            .take(15)
            .collect::<String>()
            .chars()
            .rev()
            .chain("-->".chars())
            .chain(self[position..].chars().take(15))
            .collect::<String>()
    }

    #[inline]
    fn len(&self) -> usize {
        str::len(self)
    }

    /// Slicing for string works by taking a byte position of range.start and
    /// slicing by a range.end-range.start chars.
    #[inline]
    fn slice(
        &self,
        range: Range<usize>,
    ) -> &<Self as Index<Range<usize>>>::Output {
        &self[range.start
            ..range.start
                + self[range.start..]
                    .char_indices()
                    .take(range.end - range.start + 1)
                    .map(|(idx, _)| idx)
                    .last()
                    .unwrap_or(range.start)]
    }

    fn start_location() -> Location {
        Location {
            start: Position::LineBased(LineColumn { line: 1, column: 0 }),
            end: None,
        }
    }

    fn location_after(&self, location: Location) -> Location {
        let (mut line, mut column) = match location {
            Location {
                start: Position::LineBased(lb),
                ..
            } => (lb.line, lb.column),
            _ => panic!("Location not in line/column format!"),
        };

        line += self.as_bytes().iter().filter(|&c| *c == b'\n').count();
        if let Some(new_col) = self.as_bytes().iter().rposition(|&c| c == b'\n')
        {
            column = self.len() - new_col - 1;
        } else {
            column += self.len();
        }

        Location {
            start: Position::LineBased(LineColumn { line, column }),
            end: None,
        }
    }

    fn read_file<P: AsRef<Path>>(path: P) -> Result<Self::Owned> {
        Ok(std::fs::read_to_string(path)?)
    }
}

impl Input for [u8] {
    fn context_str(&self, position: usize) -> String {
        format!(
            "{:?}",
            self[position - min(15, position)..position]
                .iter()
                .map(|x| format!("{x}"))
                .chain(once("-->".to_string()))
                .chain(self[position..].iter().map(|x| format!("{x}")).take(15))
                .collect::<Vec<_>>()
        )
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn location_after(&self, location: Location) -> Location {
        if let Location {
            start: Position::Position(p),
            ..
        } = location
        {
            Location {
                start: Position::Position(p + self.len()),
                end: None,
            }
        } else {
            Location {
                start: Position::Position(self.len()),
                end: None,
            }
        }
    }

    fn read_file<P: AsRef<Path>>(path: P) -> Result<Self::Owned> {
        Ok(std::fs::read(path)?)
    }
}

impl<T, I> Input for T
where
    Self: Deref<Target = I>
        + ToOwned<Owned = I::Owned>
        + Index<Range<usize>, Output = Self>,
    I: Input + ?Sized,
{
    #[inline]
    fn context_str(&self, position: usize) -> String {
        (**self).context_str(position)
    }

    #[inline]
    fn len(&self) -> usize {
        (**self).len()
    }

    fn read_file<P: AsRef<Path>>(path: P) -> Result<Self::Owned> {
        I::read_file(path)
    }

    #[inline]
    fn location_after(&self, location: Location) -> Location {
        (**self).location_after(location)
    }
}
