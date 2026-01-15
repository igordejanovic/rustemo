use yansi::{Color::*, Condition, Style};

#[cfg(feature = "tty")]
static COLORS: Condition = Condition::from(|| Condition::stderr_is_tty() && Condition::no_color());

#[cfg(not(feature = "tty"))]
static COLORS: Condition = Condition::NEVER;

pub static LOG: Style = Green.whenever(COLORS);
pub static LOG_BOLD: Style = Green.bold().whenever(COLORS);
pub static WARN: Style = Red.whenever(COLORS);
pub static WARN_BOLD: Style = Red.bold().whenever(COLORS);
