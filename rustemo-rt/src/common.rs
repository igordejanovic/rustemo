use std::fmt::Display;

/// Define a module using the generated parser from a `.rustemo` file.
///
/// You have to specify the name of the module and the path of the file
/// generated by Rustemo. If the input is in the root directory, you can
/// omit it.
///
/// # Example
/// ```ignore
/// // load parser in src/parser.rustemo
/// rustemo_mod!(parser);
///
/// // load parser in src/lex/parser.rustemo
/// rustemo_mod!(parser, "/lex/parser.rs");
///
/// // define a public module
/// rustemo_mod!(pub parser);
/// ```
///
/// This macro and the general idea of bootstrapping approach is taken from the
/// lalrpop project (https://github.com/lalrpop/lalrpop)
#[macro_export]
macro_rules! rustemo_mod {
    ($(#[$attr:meta])* $vis:vis $modname:ident) => {
        rustemo_mod!($(#[$attr])* $vis $modname, concat!("/", stringify!($modname), ".rs"));
    };

    ($(#[$attr:meta])* $vis:vis $modname:ident, $source:expr) => {
        $(#[$attr])* $vis mod $modname { include!(concat!(env!("OUT_DIR"), $source)); }
    };
}

#[derive(PartialEq, Debug, Clone)]
pub struct LineBased {
    pub line: u32,
    pub column: u32,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Position {
    Position(u32),
    LineBased(LineBased),
}

impl Position {
    pub fn from_lc(line: u32, column: u32) -> Self {
        Self::LineBased(LineBased {
            line, column
        })
    }

    pub fn from_pos(pos: u32) -> Self {
        Self::Position(pos)
    }

    #[inline]
    pub fn line(&self) -> u32 {
        match self {
            Position::Position(pos) => *pos,
            Position::LineBased(lb) => lb.line,
        }
    }

    #[inline]
    pub fn column(&self) -> u32 {
        match self {
            Position::Position(_) => 0,
            Position::LineBased(lb) => lb.column,
        }
    }

    #[inline]
    pub fn position(&self) -> u32 {
        match self {
            Position::Position(pos) => *pos,
            Position::LineBased(lb) => lb.line,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Position::Position(pos) => write!(f, "{pos}"),
            Position::LineBased(lb) => write!(f, "{}:{}", lb.line, lb.column),
        }

    }
}


/// `Range` describes a span from start till end.
///
/// Start is mandatory while the end is not.
#[derive(PartialEq, Debug, Clone)]
pub struct Range {
    /// The start position of the range.
    pub start: Position,
    /// The end position. Sometimes it is not known or relevant.
    pub end: Option<Position>,
}

impl Range {
    pub fn new(start: Position, end:Position) -> Self {
        Self {start, end: Some(end)}
    }
    pub fn from_start(start: Position) -> Self {
        Self {start, end: None}
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.end {
            Some(ref end) => write!(f, "{}-{}", self.start, end),
            None => write!(f, "{}", self.start),
        }

    }
}

/// Location defines the textual object inside a textual file. The file is
/// identified by its `URI` while the object is defined as a `Range`.
#[derive(PartialEq, Debug, Clone)]
pub struct Location {
    /// URI of this location. E.g. a file path.
    pub uri: String,
    pub range: Range,
}

impl Location {
    pub fn new(uri: String, range: Range) -> Self {
        Self {uri, range}
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.uri, self.range)
    }
}


#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{Position, Range, Location};

    #[test]
    pub fn test_position_linebased() {
        let p = Position::from_lc(2, 4);

        assert_eq!(p.line(), 2);
        assert_eq!(p.column(), 4);
        assert_eq!(format!("{}", p), "2:4");

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
    pub fn test_range() {
        let r = Range::new(
            Position::from_lc(5, 15),
            Position::from_lc(13, 27)
        );

        assert_eq!(format!("{}", r), "5:15-13:27");

        let r = Range::new(
            Position::from_lc(5, 15),
            Position::from_pos(49)
        );

        assert_eq!(format!("{}", r), "5:15-49");
    }

    #[test]
    pub fn test_location() {
        let l = Location::new(
            String::from(Path::new("/some/path/and/file.rs").to_str().unwrap()),
            Range::new(
                Position::from_lc(5, 18),
                Position::from_lc(10, 16)));

        assert_eq!(format!("{}", l), "/some/path/and/file.rs:5:18-10:16");
    }

}
