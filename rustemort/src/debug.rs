#[allow(unused_macros)]

#[macro_export]
/// See <https://stackoverflow.com/questions/38141056/does-rust-have-a-debug-macro>
#[cfg(debug_assertions)]
macro_rules! logn {
    ($( $args:expr ),*) => { print!( $( $args ),* ); }
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! log {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! log {
    ($( $args:expr ),*) => {
        ()
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! logn {
    ($( $args:expr ),*) => {
        ()
    };
}

// See: <https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files>
pub use log;
pub use logn;
