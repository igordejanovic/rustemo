use std::env;
use rustemo::rustemo_mod;
use rustemo::Parser;

rustemo_mod!(
    #[allow(non_camel_case_types)]
    c,
    "/src"
);
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(clippy::enum_variant_names)]
pub mod c_actions;
pub mod analysis;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: clang <c_file>");
        return;
    }

    let file_path = &args[1];
    let mut parser = c::CParser::new();
    let mut builder = c::DefaultBuilder::new();
    let result = parser.parse_file(file_path);
    match result {
        Ok(forest) => {
            let tree = forest.get_first_tree().unwrap();
            let tu = tree.build::<c::DefaultBuilder, c::State>(&mut builder);
            println!("File {} has {} loops.", file_path, analysis::count_loops(&tu));
        }
        Err(e) => eprintln!("Error parsing file: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use crate::c::{ProdKind, TokenKind, State};
    use rustemo::{Parser, TreeBuilder};
    use crate::c::{CParser, DefaultBuilder};
    use rustemo_compiler::{local_file, output_cmp};
    use crate::analysis::count_loops;

    #[test]
    fn loop_counting() {
        // fib.c has 2 loops (1 in fib, 1 in main)
        {
            let mut parser = CParser::new();
            let mut builder = DefaultBuilder::new();
            let result = parser.parse_file(local_file!(file!(), "fib.c")).unwrap();
            let tu = result.get_first_tree().unwrap()
                .build::<DefaultBuilder, State>(&mut builder);
            assert_eq!(count_loops(&tu), 2);
        }

        // complex.c has 3 loops
        {
            let mut parser = CParser::new();
            let mut builder = DefaultBuilder::new();
            let result = parser.parse_file(local_file!(file!(), "complex.c")).unwrap();
            let tu = result.get_first_tree().unwrap()
                .build::<DefaultBuilder, State>(&mut builder);
            assert_eq!(count_loops(&tu), 3);
        }
    }
}
