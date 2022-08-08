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
pub struct Settings {
    pub prefer_shifts: bool,
    pub prefer_shifts_over_empty: bool,
    pub lr_table_type: LRTableType,
    pub actions: bool,
    pub force: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            prefer_shifts: Default::default(),
            prefer_shifts_over_empty: Default::default(),
            lr_table_type: Default::default(),
            actions: true,
            force: Default::default(),
        }
    }
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
    pub fn with_actions(mut self, actions: bool) -> Self {
        self.actions = actions;
        self
    }
    pub fn with_force(mut self, force: bool) -> Self {
        self.force = force;
        self
    }
}
