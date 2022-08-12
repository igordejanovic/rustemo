/// Loads a parser modules from the Cargo OUT_DIR location.
/// Used during bootstrapping.
/// This macro and the general idea of bootstrapping approach is taken from the
/// lalrpop project (https://github.com/lalrpop/lalrpop)
#[macro_export]
macro_rules! rustemo_mod {
    ($modname:ident) => {
        pub(crate) mod $modname {
            include!(concat!(
                env!("OUT_DIR"),
                "/src/lang/",
                stringify!($modname),
                ".rs"
            ));
        }
    };
}
