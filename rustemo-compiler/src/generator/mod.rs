pub(crate) mod actions;
mod arrays;
mod functions;

use quote::format_ident;
use rustemo::Parser;
use std::{
    fs,
    path::{Path, PathBuf},
};
use syn::{parse_quote, Ident};

use crate::grammar::{
    types::{to_pascal_case, to_snake_case, Choice, SymbolTypes},
    Grammar, NonTerminal, Production,
};
use crate::{
    error::{Error, Result},
    index::{StateIndex, TermIndex},
    lang::rustemo::RustemoParser,
    settings::{BuilderType, GeneratorTableType, LexerType, Settings},
    table::{Action, LRTable},
};
use crate::{grammar::builder::GrammarBuilder, ParserAlgo};

/// Generator for parser implementation parts. Different types can implement
/// different parser implementation strategies.
trait PartGenerator<'g, 's> {
    fn header(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>>;
    fn symbols(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>>;
    fn types(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>>;
    fn lexer_definition(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>>;
    fn parser_definition(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>>;
    fn builder(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>>;
}

/// Main entry point for the parser generator.
pub fn generate_parser(
    grammar_path: &Path,
    out_dir: Option<&Path>,
    out_dir_actions: Option<&Path>,
    settings: &Settings,
) -> Result<()> {
    if !grammar_path.exists() {
        return Err(Error::Error("Grammar file doesn't exist.".to_string()));
    }

    let grammar_dir =
        PathBuf::from(grammar_path.parent().ok_or_else(|| {
            Error::Error(
                "Cannot deduce parent directory of the grammar file."
                    .to_string(),
            )
        })?);

    let out_dir = out_dir.unwrap_or(&grammar_dir);
    let out_dir_actions = out_dir_actions.unwrap_or(&grammar_dir);

    let mut parser = RustemoParser::new();
    let file = parser.parse_file(grammar_path)?;
    let grammar: Grammar =
        GrammarBuilder::new().try_from_file(file, Some(grammar_path))?;

    // Check recognizers definition. If default string lexer is used all
    // recognizers must be defined. If custom lexer is used no recognizer should
    // be defined.
    if let LexerType::Default = settings.lexer_type {
        for term in &grammar.terminals {
            if term.idx != TermIndex(0) && term.recognizer.is_none() {
                return Err(Error::Error(format!(
                    "Recognizer not defined for terminal '{}'.",
                    term.name
                )));
            }
        }
    }

    let table = LRTable::new(&grammar, settings)?;
    if settings.dot {
        fs::write(grammar_path.with_extension("dot"), table.to_dot())?;
    }

    if let ParserAlgo::LR = settings.parser_algo {
        let conflicts = table.get_conflicts();
        if !conflicts.is_empty() {
            table.print_conflicts_report(&conflicts);
            return Err(Error::Error(
                "Grammar is not deterministic. There are conflicts."
                    .to_string(),
            ));
        }
    }

    let generator = ParserGenerator::new(
        grammar_path,
        out_dir.to_owned(),
        out_dir_actions.to_owned(),
        &grammar,
        table,
        settings,
    )?;

    generator.generate(out_dir)?;
    Ok(())
}

/// A generator for the parser code.
// Some fields are used in parse_quote macros, that's why dead code is allowed.
#[allow(dead_code)]
struct ParserGenerator<'g, 's> {
    file_name: String,
    root_symbol: Ident,
    parser: Ident,
    layout_parser: Ident,
    parser_definition: Ident,
    actions_file: Ident,
    lexer_file: Ident,
    builder_file: Ident,
    grammar: &'g Grammar,
    out_dir: PathBuf,
    out_dir_actions: PathBuf,
    table: LRTable<'g, 's>,
    settings: &'s Settings,
    input_type: syn::Type,
    part_generator: Box<dyn PartGenerator<'g, 's>>,
    types: Option<SymbolTypes>,
}

impl<'g, 's> ParserGenerator<'g, 's> {
    fn new(
        grammar_path: &Path,
        out_dir: PathBuf,
        out_dir_actions: PathBuf,
        grammar: &'g Grammar,
        table: LRTable<'g, 's>,
        settings: &'s Settings,
    ) -> Result<Self> {
        let file_name = grammar_path
            .file_stem()
            .ok_or_else(|| {
                Error::Error(format!(
                    "Cannot deduce base file name from {:?}",
                    grammar_path
                ))
            })?
            .to_str()
            .ok_or_else(|| {
                Error::Error(format!(
                    "Cannot deduce base file name from {:?}",
                    grammar_path
                ))
            })?;
        let parser_name = to_pascal_case(file_name);
        let root_symbol =
            format_ident!("{}", grammar.symbol_name(grammar.start_index));
        let parser = format_ident!("{}Parser", parser_name);
        let layout_parser = format_ident!("{}LayoutParser", parser_name);
        let parser_definition = format_ident!("{}Definition", parser);
        let actions_file = format_ident!("{}_actions", file_name);
        let lexer_file = format_ident!("{}_lexer", file_name);
        let builder_file = format_ident!("{}_builder", file_name);

        // Choose parser implementation strategy.
        let part_generator: Box<dyn PartGenerator> =
            match settings.generator_table_type {
                GeneratorTableType::Arrays => {
                    Box::new(arrays::ArrayPartGenerator::new())
                }
                GeneratorTableType::Functions => {
                    Box::new(functions::FunctionPartGenerator::new())
                }
            };

        let input_type = syn::parse_str(&settings.input_type)?;

        let types = if let BuilderType::Default = settings.builder_type {
            // Deduce AST types
            Some(SymbolTypes::new(grammar))
        } else {
            None
        };

        Ok(Self {
            file_name: file_name.to_string(),
            root_symbol,
            parser,
            layout_parser,
            parser_definition,
            actions_file,
            lexer_file,
            builder_file,
            grammar,
            out_dir,
            out_dir_actions,
            table,
            settings,
            input_type,
            part_generator,
            types,
        })
    }

    fn generate(&self, out_dir: &Path) -> Result<()> {
        let mut ast: Vec<syn::Stmt> = vec![];
        ast.extend(self.part_generator.header(self)?);
        ast.extend(self.part_generator.types(self)?);
        ast.extend(self.part_generator.symbols(self)?);
        ast.extend(self.part_generator.parser_definition(self)?);
        ast.extend(self.part_generator.lexer_definition(self)?);
        ast.extend(self.part_generator.builder(self)?);

        std::fs::create_dir_all(out_dir).map_err(|e| {
            Error::Error(format!(
                "Cannot create directories for path '{out_dir:?}': {e:?}."
            ))
        })?;

        let mut file: syn::File = parse_quote!();

        file.items.extend(ast.into_iter().map(|s| match s {
            syn::Stmt::Item(i) => i,
            _ => panic!("Invalid item."),
        }));

        let out_file = out_dir.join(&self.file_name).with_extension("rs");
        std::fs::write(&out_file, prettyplease::unparse(&file)).map_err(
            |e| {
                Error::Error(format!(
                    "Cannot write parser file '{out_file:?}': {e:?}."
                ))
            },
        )?;

        Ok(())
    }

    fn prod_kind(&self, prod: &Production) -> String {
        format!(
            "{}{}",
            prod.nonterminal(self.grammar).name,
            if let Some(ref kind) = prod.kind {
                kind.clone()
            } else {
                format!("P{}", prod.ntidx + 1)
            }
        )
    }

    fn prod_kind_ident(&self, prod: &Production) -> syn::Ident {
        format_ident!("{}", self.prod_kind(prod))
    }

    fn state_kind_ident(&self, state: StateIndex) -> syn::Ident {
        format_ident!(
            "{}S{}",
            self.grammar.symbol_name(self.table.states[state].symbol),
            state.0
        )
    }

    fn action_to_syntax(&self, action: &Option<Action>) -> syn::Expr {
        match action {
            Some(action) => match action {
                Action::Shift(state) => {
                    let state_kind_ident = self.state_kind_ident(*state);
                    parse_quote! { Shift(State::#state_kind_ident) }
                }
                Action::Reduce(prod, len) => {
                    let prod_kind =
                        self.prod_kind_ident(&self.grammar.productions[*prod]);
                    parse_quote! { Reduce(ProdKind::#prod_kind, #len) }
                }
                Action::Accept => parse_quote! { Accept },
            },
            None => parse_quote! { Error },
        }
    }
}

fn action_name(nonterminal: &NonTerminal, choice: &Choice) -> String {
    to_snake_case(format!("{}_{}", nonterminal.name, &choice.name))
}
