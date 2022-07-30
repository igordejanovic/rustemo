#[derive(Debug, PartialEq)]
pub enum LRTableType {
    LALR, // http://publications.csail.mit.edu/lcs/pubs/pdf/MIT-LCS-TR-065.pdf
    LALR_PAGERW, // https://doi.org/10.1007/BF00290336
    LALR_RN, // https://doi.org/10.1145/1146809.1146810
}

impl Default for LRTableType {
    fn default() -> Self {
        LRTableType::LALR_PAGERW
    }
}

#[derive(Debug, Default)]
pub struct Settings {
    pub prefer_shifts: bool,
    pub prefer_shifts_over_empty: bool,
    pub lr_table_type: LRTableType,
}
