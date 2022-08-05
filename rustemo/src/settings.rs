#[allow(non_camel_case_types)]
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

#[derive(Debug)]
pub enum GenActions {
    ProductionBased,
    RuleBased,
}

impl Default for GenActions {
    fn default() -> Self {
        GenActions::ProductionBased
    }
}

#[derive(Debug, Default)]
pub struct Settings {
    pub prefer_shifts: bool,
    pub prefer_shifts_over_empty: bool,
    pub lr_table_type: LRTableType,
    pub gen_actions: GenActions,
}

impl Settings {
    pub fn with_prefer_shifts(mut self) -> Self {
        self.prefer_shifts = true;
        self
    }
    pub fn with_prefer_shifts_over_empty(mut self) -> Self {
        self.prefer_shifts_over_empty = true;
        self
    }
    pub fn with_lr_table_type(mut self, table_type: LRTableType) -> Self {
        self.lr_table_type = table_type;
        self
    }
    pub fn with_gen_actions(mut self, gen_actions: GenActions) -> Self {
        self.gen_actions = gen_actions;
        self
    }
}
