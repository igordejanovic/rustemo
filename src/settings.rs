#[derive(Debug, PartialEq)]
pub enum LRTableType {
    LALR,
    LALRP,
    LALRRN,
}

impl Default for LRTableType {
    fn default() -> Self { LRTableType::LALRP }
}



#[derive(Debug, Default)]
pub(in crate) struct Settings {
    pub prefer_shifts: bool,
    pub prefer_shifts_over_empty: bool,
    pub lr_table_type: LRTableType,
}

