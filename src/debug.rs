#[allow(unused_macros)]

/// See https://stackoverflow.com/questions/38141056/does-rust-have-a-debug-macro
#[cfg(debug_assertions)]
macro_rules! logn {
    ($( $args:expr ),*) => { print!( $( $args ),* ); }
}

#[cfg(debug_assertions)]
macro_rules! log {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

#[cfg(not(debug_assertions))]
macro_rules! log {
    ($( $args:expr ),*) => {
        ()
    };
}

#[cfg(not(debug_assertions))]
macro_rules! logn {
    ($( $args:expr ),*) => {
        ()
    };
}

// See: https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files
pub(crate) use log;
pub(crate) use logn;
