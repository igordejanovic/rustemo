use super::{
    grammar::Grammar,
    lexer::DefaultLexer,
    rustemo::{
        RustemoBuilder, RustemoLexerDefinition, RustemoParserDefinition, LEXER_DEFINITION,
        PARSER_DEFINITION,
    },
    rustemo_types::{NonTerminal, Symbol},
};

use crate::{
    lexer::Lexer,
    lr::{LRContext, LRParser},
    parser::Parser, index::StateIndex,
};

pub type GrammarLexer<'i> = DefaultLexer<'i, RustemoLexerDefinition>;

pub struct GrammarParser(LRParser<RustemoParserDefinition>);

type RBuilder<'i> = RustemoBuilder<'i, <GrammarLexer<'i> as Lexer>::Input>;

impl<'i> GrammarParser {
    pub(in crate::lang) fn parse(&mut self, lexer: GrammarLexer<'i>) -> Grammar {
        let pgfile = match <LRParser<RustemoParserDefinition> as Parser<
            GrammarLexer<'i>,
            RBuilder<'i>,
        >>::parse(&mut self.0, lexer)
        {
            Symbol::NonTerminal(NonTerminal::PGFile(p)) => p,
            _ => {
                panic!("Invalid return type of inner parse.")
            }
        };
        Grammar::from_pgfile(pgfile)
    }
}

impl GrammarParser {
    pub fn default() -> Self {
        Self(LRParser {
            context: LRContext {
                parse_stack: vec![StateIndex(0)],
                current_state: StateIndex(0),
                position: 0,
            },
            definition: &PARSER_DEFINITION,
        })
    }
}

// Enables creating a lexer from a reference to an object that can be converted
// to a string reference.
impl<'i, T> From<&'i T> for GrammarLexer<'i>
where
    T: AsRef<str> + 'i + ?Sized,
{
    fn from(input: &'i T) -> Self {
        Self {
            input: input.as_ref(),
            token_ahead: None,
            definition: &LEXER_DEFINITION,
        }
    }
}


#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use crate::tests::utils::{string_difference, type_of};

    use super::*;


    #[test]
    fn type_of_return() {
        let grammar = GrammarParser::default().parse(
            r#"
             S: A B;
             A: "a";
             B: "b";
            "#
            .into(),
        );
        assert!(type_of(&grammar) == "rustemo::lang::grammar::Grammar");
    }

    #[test]
    fn test_parse_rustemo_grammar() {
        use std::fs;
        use std::path::PathBuf;

        let mut path = PathBuf::from(file!());
        path.pop();
        path.push("rustemo.rustemo");
        let content: String = fs::read_to_string(&path).expect("Cannot load rustemo grammar!");
        let grammar = GrammarParser::default().parse(content.as_str().into());

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
