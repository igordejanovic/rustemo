use crate::table::TableType;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Settings {
    pub out_dir: Option<PathBuf>,
    pub out_dir_actions: Option<PathBuf>,
    pub prefer_shifts: bool,
    pub prefer_shifts_over_empty: bool,
    pub table_type: TableType,
    pub exclude: Vec<String>,
    pub actions: bool,
    /// Should actions file be recreated if exist. Use with care.
    pub force: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            out_dir: None,
            out_dir_actions: None,
            prefer_shifts: false,
            prefer_shifts_over_empty: true,
            table_type: Default::default(),
            actions: true,
            force: false,
            exclude: vec![],
        }
    }
}
