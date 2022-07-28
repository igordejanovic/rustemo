use crate::{location::Location, index::TermIndex};

pub type TerminalInfos<const T: usize> = [TerminalInfo; T];
pub type TerminalsState<const T: usize, const S: usize> =
    [[Option<usize>; T]; S];

#[derive(Debug)]
pub struct TerminalInfo {
    pub id: TermIndex,
    pub name: &'static str,
    pub location: Option<Location>,
}

#[derive(Debug)]
pub struct NonTerminalInfo {
    pub id: usize,
    pub name: &'static str,
    pub location: Option<Location>,
    #[cfg(debug_assertions)]
    pub production_str: &'static str,
}
