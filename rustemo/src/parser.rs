use super::{
    grammar::Grammar,
    rustemo::{RustemoBuilder, RustemoLexer, RustemoParser},
    rustemo_types::{NonTerminal, Symbol},
};

use rustemo_rt::{builder::Builder, lexer::Lexer, parser::Parser};

impl<'i> RustemoParser<'i> {
    pub fn parse(&mut self, lexer: RustemoLexer<'i>) -> Grammar {
        let builder =
            RustemoBuilder::<'_, <RustemoLexer as Lexer>::Input>::new();
        let pgfile = match self.0.parse(lexer, builder) {
            Symbol::NonTerminal(NonTerminal::PGFile(p)) => p,
            _ => {
                panic!("Invalid return type of inner parse.")
            }
        };
        Grammar::from_pgfile(pgfile)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        output_cmp,
        tests::utils::type_of,
    };

    use super::*;

    #[test]
    fn type_of_return() {
        let grammar = RustemoParser::default().parse(
            r#"
             S: A B;
            terminals
             A: "a";
             B: "b";
            "#
            .into(),
        );
        assert!(type_of(&grammar) == "rustemo::grammar::Grammar");
    }

    #[test]
    fn test_parse_rustemo_grammar() {
        use std::fs;
        use std::path::PathBuf;

        let path: PathBuf =
            [env!("CARGO_MANIFEST_DIR"), "src", "rustemo.rustemo"]
                .iter()
                .collect();
        let content: String =
            fs::read_to_string(&path).expect("Cannot load rustemo grammar!");
        let grammar = RustemoParser::default().parse(content.as_str().into());

        output_cmp!("src/rustemo.parse_tree", format!("{:#?}", grammar));
    }
}
