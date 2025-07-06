use crate::{
    error::Result,
    position::{LineColumn, Position, SourceSpan},
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
    fn context_str(&self, position: Position) -> String;

    /// Returns the length of the input.
    fn len(&self) -> usize;

    /// Determines if the input is an empty sequence.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Implement for types which may cause panic on slicing with full `Range`
    /// (e.g. `str`).
    #[inline]
    fn slice(&self, range: Range<usize>) -> &<Self as Index<Range<usize>>>::Output {
        &self[range]
    }

    /// Read the file from the given path into owned version of the input.
    fn read_file<P: AsRef<Path>>(path: P) -> Result<Self::Owned>;

    fn start_position() -> Position {
        Position {
            pos: 0,
            line_col: None,
        }
    }

    /// Given the current location returns the location at the end of self.
    fn position_after(&self, position: Position) -> Position;

    /// Given the current position returns a span starting from the current
    /// position and extending over self.
    fn span_from(&self, position: Position) -> SourceSpan {
        SourceSpan {
            start: position,
            end: self.position_after(position),
        }
    }
}

impl Input for str {
    fn context_str(&self, position: Position) -> String {
        self[..position.pos]
            .chars()
            .rev()
            .take(15)
            .collect::<String>()
            .chars()
            .rev()
            .chain("-->".chars())
            .chain(self[position.pos..].chars().take(15))
            .collect::<String>()
    }

    #[inline]
    fn len(&self) -> usize {
        str::len(self)
    }

    /// Slicing for string works by taking a byte position of range.start and
    /// slicing by a range.end-range.start chars.
    #[inline]
    fn slice(&self, range: Range<usize>) -> &<Self as Index<Range<usize>>>::Output {
        &self[range.start
            ..range.start
                + self[range.start..]
                    .char_indices()
                    .take(range.end - range.start + 1)
                    .map(|(idx, _)| idx)
                    .last()
                    .unwrap_or(range.start)]
    }

    fn start_position() -> Position {
        Position {
            pos: 0,
            line_col: Some(LineColumn { line: 1, column: 0 }),
        }
    }

    fn position_after(&self, position: Position) -> Position {
        let line_col = if let Some(LineColumn {
            mut line,
            mut column,
        }) = position.line_col
        {
            line += self.as_bytes().iter().filter(|&c| *c == b'\n').count();
            if let Some(new_col) = self.as_bytes().iter().rposition(|&c| c == b'\n') {
                column = self.len() - new_col - 1;
            } else {
                column += self.len();
            }
            Some(LineColumn { line, column })
        } else {
            None
        };

        let pos = position.pos + self.len();

        Position { pos, line_col }
    }

    fn read_file<P: AsRef<Path>>(path: P) -> Result<Self::Owned> {
        Ok(std::fs::read_to_string(path)?)
    }
}

impl Input for [u8] {
    fn context_str(&self, position: Position) -> String {
        format!(
            "{:?}",
            self[position.pos - min(15, position.pos)..position.pos]
                .iter()
                .map(|x| format!("{x}"))
                .chain(once("-->".to_string()))
                .chain(self[position.pos..].iter().map(|x| format!("{x}")).take(15))
                .collect::<Vec<_>>()
        )
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn position_after(&self, position: Position) -> Position {
        Position {
            pos: position.pos + self.len(),
            line_col: None,
        }
    }

    fn read_file<P: AsRef<Path>>(path: P) -> Result<Self::Owned> {
        Ok(std::fs::read(path)?)
    }
}

impl<T, I> Input for T
where
    Self: Deref<Target = I> + ToOwned<Owned = I::Owned> + Index<Range<usize>, Output = Self>,
    I: Input + ?Sized,
{
    #[inline]
    fn context_str(&self, position: Position) -> String {
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
    fn position_after(&self, position: Position) -> Position {
        (**self).position_after(position)
    }
}
