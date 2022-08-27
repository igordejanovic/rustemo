use crate::index::TermIndex;

pub type TerminalInfos<const T: usize> = [TerminalInfo; T];
pub type TerminalsState<const T: usize, const S: usize> =
    [[Option<usize>; T]; S];

#[derive(Debug)]
pub struct TerminalInfo {
    pub id: TermIndex,
    pub name: &'static str,
}
