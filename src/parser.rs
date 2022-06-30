use super::{
    grammar::Grammar,
    rustemo::{RustemoBuilder, RustemoLexer, RustemoParser},
    rustemo_types::{NonTerminal, Symbol},
};

use rustemort::{builder::Builder, lexer::Lexer, parser::Parser};

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
    use std::fmt::Write;

    use crate::{tests::utils::{string_difference, type_of}, output_cmp};

    use super::*;

    #[test]
    fn type_of_return() {
        let grammar = RustemoParser::default().parse(
            r#"
             S: A B;
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

        let mut path = PathBuf::from(file!());
        path.pop();
        path.push("rustemo.rustemo");
        let content: String =
            fs::read_to_string(&path).expect("Cannot load rustemo grammar!");
        let grammar = RustemoParser::default().parse(content.as_str().into());

        output_cmp!("rustemo.parse_tree", format!("{:#?}", grammar));
    }
}
