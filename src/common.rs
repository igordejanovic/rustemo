type URI = String;

#[derive(PartialEq, Debug, Clone)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

/// Location defines the textual object inside a textual file. The file is
/// identified by its `URI` while the object is defined as a `Range`.
#[derive(PartialEq, Debug, Clone)]
pub struct Location {
    pub uri: URI,
    pub range: Range,
}
