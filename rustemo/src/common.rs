/// Loads generated parser modules from the Cargo OUT_DIR location.
///
/// Used when the parser is generated from the `build.rs` script.
///
/// This macro and the general idea of bootstrapping approach is based on idea
/// from [lalrpop project](https://github.com/lalrpop/lalrpop)
#[macro_export]
macro_rules! rustemo_mod {
    ($(#[$attr:meta])* $vis:vis $modname:ident, $source:expr) => {
        $(#[$attr])* $vis mod $modname { include!(concat!(env!("OUT_DIR"),
                                                          $source, "/",
                                                          stringify!($modname), ".rs")); }
    };
}
