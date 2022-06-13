use super::{
    grammar::Grammar,
    rustemo::{
        RustemoBuilder, RustemoLexerDefinition, RustemoParserDefinition, LEXER_DEFINITION,
        PARSER_DEFINITION,
    },
    rustemo_types::{NonTerminal, Symbol},
};

use rustemort::{
    builder::Builder,
    index::StateIndex,
    lexer::{Lexer, DefaultLexer},
    lr::{LRContext, LRParser},
    parser::Parser,
};

pub struct RustemoLexer<'i>(DefaultLexer<'i, RustemoLexerDefinition>);

pub struct RustemoParser<'i>(LRParser<&'i str, RustemoParserDefinition>);

type RBuilder<'i> = RustemoBuilder<'i, <RustemoLexer<'i> as Lexer>::Input>;

impl<'i> RustemoParser<'i> {
    pub(in crate) fn parse(&mut self, lexer: RustemoLexer<'i>) -> Grammar {
        let builder = RBuilder::new();
        let pgfile = match self.0.parse(lexer, builder) {
            Symbol::NonTerminal(NonTerminal::PGFile(p)) => p,
            _ => {
                panic!("Invalid return type of inner parse.")
            }
        };
        Grammar::from_pgfile(pgfile)
    }
}

impl<'i> Default for RustemoParser<'i> {
    fn default() -> Self {
        Self(LRParser {
            context: LRContext {
                parse_stack: vec![StateIndex(0)],
                current_state: StateIndex(0),
                position: 0,
                token: None,
            },
            definition: &PARSER_DEFINITION,
        })
    }
}

impl<'i> Lexer for RustemoLexer<'i> {
    type Input = &'i str;

    fn next_token(&self, context: &mut impl rustemort::parser::Context<Self::Input>) -> Option<rustemort::lexer::Token<Self::Input>> {
        self.0.next_token(context)
    }
}

// Enables creating a lexer from a reference to an object that can be converted
// to a string reference.
impl<'i, T> From<&'i T> for RustemoLexer<'i>
where
    T: AsRef<str> + ?Sized,
{
    fn from(input: &'i T) -> Self {
        Self (DefaultLexer::new(input.as_ref(), &LEXER_DEFINITION))
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use crate::tests::utils::{string_difference, type_of};

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
        let content: String = fs::read_to_string(&path).expect("Cannot load rustemo grammar!");
        let grammar = RustemoParser::default().parse(content.as_str().into());

        path.pop();
        path.push("rustemo.parse_tree");
        if path.exists() {
            let content: String = fs::read_to_string(&path).expect("Cannot load tree output file.");
            let mut output = String::new();
            write!(&mut output, "{:#?}", grammar).expect("Error formatting output tree.");
            if let Some(diff) = string_difference(&content, &output) {
                assert!(false, "Strings differ at: {:?}", diff)
            }
        } else {
            let mut output = String::new();
            write!(&mut output, "{:#?}", grammar).expect("Error formatting output tree.");
            fs::write(path, output).expect("Error writing file");
        }
    }
}
