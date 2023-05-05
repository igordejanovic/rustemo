pub(crate) mod actions;

use quote::format_ident;
use quote::quote;
use rustemo::index::TermIndex;
use std::{
    iter::repeat,
    path::{Path, PathBuf},
};
use syn::{parse::Parser, parse_quote, Ident};

use crate::{
    error::{Error, Result},
    grammar::{
        types::{to_pascal_case, to_snake_case, Choice, SymbolTypes},
        Grammar, NonTerminal, Production, Terminal,
    },
    lang::{rustemo::RustemoParser, rustemo_actions::Recognizer},
    settings::{BuilderType, LexerType, Settings},
    table::{Action, LRTable},
};

use self::actions::generate_parser_actions;

macro_rules! create_idents {
   ($self:ident, $($id:ident),+ $(,)?) => {
        $(let $id = &$self.$id;)+
    }
}

pub fn generate_parser(
    grammar_path: &Path,
    out_dir: Option<&Path>,
    out_dir_actions: Option<&Path>,
    settings: &Settings,
) -> Result<()> {
    if !grammar_path.exists() {
        return Err(Error::Error("Grammar file doesn't exist.".to_string()));
    }

    let grammar_dir = PathBuf::from(
        grammar_path
            .parent()
            .expect("Cannot deduce parent directory of the grammar file."),
    );

    let out_dir = out_dir.unwrap_or(&grammar_dir);
    let out_dir_actions = out_dir_actions.unwrap_or(&grammar_dir);

    let mut parser = RustemoParser::new();
    let file = parser.parse_file(grammar_path)?;
    let grammar: Grammar = file.try_into()?;

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

    let table = LRTable::new(&grammar, settings);

    let conflicts = table.get_conflicts();
    if !conflicts.is_empty() {
        table.print_conflicts_report(&conflicts);
        return Err(Error::Error(
            "Grammar is not deterministic. There are conflicts.".to_string(),
        ));
    }

    let generator =
        ParserGenerator::new(grammar_path, &grammar, table, settings)?;
    generator.generate(out_dir, out_dir_actions, &grammar)?;
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
    table: LRTable<'g, 's>,
    settings: &'s Settings,
}

impl<'g, 's> ParserGenerator<'g, 's> {
    fn new(
        grammar_path: &Path,
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
            table,
            settings,
        })
    }

    fn generate(
        &self,
        out_dir: &Path,
        out_dir_actions: &Path,
        grammar: &Grammar,
    ) -> Result<()> {
        let mut ast: syn::File = self.generate_parser_header()?;
        ast.items.extend(self.generate_parser_types()?);

        if let BuilderType::Default = self.settings.builder_type {
            ast.items.extend(self.generate_parser_symbols()?);
        }

        ast.items.extend(self.generate_parser_definition()?);

        if self.grammar.has_layout() {
            ast.items.extend(self.generate_layout_parser()?);
        }

        ast.items.extend(self.generate_lexer_definition()?);

        if let BuilderType::Default = self.settings.builder_type {
            let types = SymbolTypes::new(grammar);
            ast.items.extend(self.generate_builder(&types)?);

            // Generate actions
            if self.settings.actions {
                generate_parser_actions(
                    self.grammar,
                    &types,
                    &self.file_name,
                    out_dir_actions,
                    self.settings,
                )?;
            }
        }

        std::fs::create_dir_all(out_dir).map_err(|e| {
            Error::Error(format!(
                "Cannot create folders for path '{out_dir:?}': {e:?}."
            ))
        })?;

        let out_file = out_dir.join(&self.file_name).with_extension("rs");
        std::fs::write(&out_file, prettyplease::unparse(&ast)).map_err(
            |e| {
                Error::Error(format!(
                    "Cannot write parser file '{out_file:?}': {e:?}."
                ))
            },
        )?;

        Ok(())
    }

    fn generate_parser_header(&self) -> Result<syn::File> {
        create_idents!(self, lexer_file, actions_file,);

        let max_actions = self
            .table
            .states
            .iter()
            .map(|x| x.actions.iter().filter(|x| !x.is_empty()).count())
            .max()
            .unwrap();

        let term_count = self.grammar.terminals.len();
        let nonterm_count = self.grammar.nonterminals.len();
        let states_count = self.table.states.len();

        let builder_import: syn::Stmt = if self.grammar.has_layout() {
            parse_quote! {
                use rustemo::lr::builder::{LRBuilder, SliceBuilder};
            }
        } else {
            parse_quote! {
                use rustemo::lr::builder::LRBuilder;
            }
        };

        let mut header: syn::File = parse_quote! {
            /// Generated by rustemo. Do not edit manually!
            use std::fmt::Debug;
            use std::hash::{Hash, Hasher};

            use rustemo::Result;
            use rustemo::lexer::{self, Token, AsStr, StringLexer};
            use rustemo::parser::Parser;
            use rustemo::builder::Builder;
            #builder_import
            use rustemo::lr::parser::{LRParser, ParserDefinition};
            use rustemo::lr::parser::Action::{self, Shift, Reduce, Accept, Error};
            use rustemo::index::{StateIndex, TermIndex, NonTermIndex, ProdIndex};
            #[allow(unused_imports)]
            use rustemo::debug::{log, logn};

            const TERMINAL_COUNT: usize = #term_count;
            const NONTERMINAL_COUNT: usize = #nonterm_count;
            const STATE_COUNT: usize = #states_count;
            #[allow(dead_code)]
            const MAX_ACTIONS: usize = #max_actions;

        };

        if let LexerType::Default = self.settings.lexer_type {
            header.items.push(parse_quote! {
                use regex::Regex;
            });
            header.items.push(parse_quote! {
                use once_cell::sync::Lazy;
            });
        } else {
            header.items.push(parse_quote! {
                use rustemo::lexer::Lexer;
            });
        }

        match self.settings.builder_type {
            BuilderType::Default => header.items.push(parse_quote! {
                use super::#actions_file;
            }),
            BuilderType::Generic => header.items.push(parse_quote! {
                use rustemo::lr::builder::{TreeNode, TreeBuilder};
            }),
            BuilderType::Custom => header.items.push(parse_quote! {
                use std::cell::RefCell;
            }),
        }

        header.items.push(match self.settings.lexer_type {
            LexerType::Default => parse_quote! {
                pub type Input = str;
            },
            LexerType::Custom => parse_quote! {
                use super::#lexer_file::Input;
            },
        });

        header.items.push(parse_quote! {
            pub type Context<'i> = lexer::Context<'i, Input>;
        });

        Ok(header)
    }

    fn generate_parser_types(&self) -> Result<Vec<syn::Item>> {
        let mut ast: Vec<syn::Item> = vec![];

        let token_kind_variants: Vec<syn::Variant> = self
            .grammar
            .terminals
            .iter()
            .map(|t| {
                let name = format_ident!("{}", t.name);
                parse_quote! { #name }
            })
            .collect();

        ast.push(parse_quote! {
            #[allow(clippy::upper_case_acronyms)]
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
            pub enum TokenKind {
                #[default]
                #(#token_kind_variants),*
            }
        });

        let as_str_arms: Vec<syn::Arm> = self
            .grammar
            .terminals
            .iter()
            .map(|t| {
                let name = format_ident!("{}", t.name);
                let name_str = &t.name;
                parse_quote! { TokenKind::#name => #name_str }
            })
            .collect();
        ast.push(parse_quote! {
            impl AsStr for TokenKind {
                #[allow(dead_code)]
                fn as_str(&self) -> &'static str {
                    match self {
                        #(#as_str_arms),*
                    }
                }
            }
        });

        let (from_arms, into_arms): (Vec<syn::Arm>, Vec<syn::Arm>) = self
            .grammar
            .terminals
            .iter()
            .map(|t| {
                let name = format_ident!("{}", t.name);
                let idx = t.idx.0;
                (
                    parse_quote! { #idx => TokenKind::#name },
                    parse_quote! { TokenKind::#name => TermIndex(#idx) },
                )
            })
            .collect::<Vec<_>>()
            .into_iter()
            .unzip();
        ast.push(parse_quote! {
            impl From<TermIndex> for TokenKind {
                fn from(term_index: TermIndex) -> Self {
                    match term_index.0 {
                        #(#from_arms),*,
                        _ => unreachable!()
                    }
                }
            }
        });
        ast.push(parse_quote! {
            impl From<TokenKind> for TermIndex {
                fn from(token_kind: TokenKind) -> Self {
                    match token_kind {
                        #(#into_arms),*
                    }
                }
            }
        });

        let prodkind_variants: Vec<syn::Variant> = self
            .grammar
            .productions()
            .iter()
            .map(|prod| {
                let prod_kind = self.prod_kind_ident(prod);
                parse_quote! {#prod_kind}
            })
            .collect();
        ast.push(parse_quote! {
            #[allow(clippy::enum_variant_names)]
            #[derive(Clone, Copy)]
            pub enum ProdKind {
                #(#prodkind_variants),*
            }
        });

        let (as_str_arms, display_arms): (Vec<syn::Arm>, Vec<syn::Arm>) = self
            .grammar
            .productions()
            .iter()
            .map(|&prod| {
                let prod_kind = self.prod_kind(prod);
                let prod_kind_ident = self.prod_kind_ident(prod);
                let prod_str = prod.to_string(self.grammar);
                (
                    parse_quote! { ProdKind::#prod_kind_ident => #prod_kind },
                    parse_quote! { ProdKind::#prod_kind_ident => #prod_str },
                )
            })
            .collect::<Vec<_>>()
            .into_iter()
            .unzip();
        ast.push(parse_quote! {
            impl AsStr for ProdKind {
                #[allow(dead_code)]
                fn as_str(&self) -> &'static str {
                    match self {
                        #(#as_str_arms),*
                    }
                }
            }
        });
        ast.push(parse_quote! {
        impl std::fmt::Display for ProdKind {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let name = match self {
                    #(#display_arms),*
                };
                write!(f, "{}", name)
            }
        }
    });

        let from_arms: Vec<syn::Arm> = self
            .grammar
            .productions()
            .iter()
            .map(|&prod| {
                let prod_kind = self.prod_kind_ident(prod);
                let idx = prod.idx.0;
                parse_quote! { #idx => ProdKind::#prod_kind }
            })
            .collect();
        ast.push(parse_quote! {
            impl From<ProdIndex> for ProdKind {
                fn from(prod_index: ProdIndex) -> Self {
                    match prod_index.0 {
                        #(#from_arms),*,
                        _ => unreachable!()
                    }
                }
            }
        });

        Ok(ast)
    }

    fn generate_parser_symbols(&self) -> Result<Vec<syn::Item>> {
        create_idents!(self, actions_file);

        let mut ast: Vec<syn::Item> = vec![];

        ast.push(parse_quote! {
            #[derive(Debug)]
            pub enum Symbol {
                Terminal(Terminal),
                NonTerminal(NonTerminal)
            }
        });

        let term_variants: Vec<syn::Variant> = self.grammar.terminals[1..]
            .iter()
            .filter(|t| t.reachable.get())
            .map(|t| {
                let name = format_ident!("{}", t.name);
                if t.has_content {
                    parse_quote! {
                        #name(#actions_file::#name)
                    }
                } else {
                    parse_quote! {
                        #name
                    }
                }
            })
            .collect();

        ast.push(parse_quote! {
            #[allow(clippy::upper_case_acronyms)]
            #[derive(Debug)]
            pub enum Terminal {
                #(#term_variants),*
            }
        });

        let nonterm_variants: Vec<syn::Variant> = self
            .grammar
            .nonterminals()
            .iter()
            .filter(|nt| nt.reachable.get())
            .map(|nt| {
                let name = format_ident!("{}", nt.name);
                parse_quote! {
                    #name(#actions_file::#name)
                }
            })
            .collect();

        ast.push(parse_quote! {
            #[derive(Debug)]
            pub enum NonTerminal {
                #(#nonterm_variants),*
            }
        });

        Ok(ast)
    }

    fn generate_parser_definition(&self) -> Result<Vec<syn::Item>> {
        create_idents!(self, parser, parser_definition, layout_parser,);
        let mut ast: Vec<syn::Item> = vec![];

        let max_actions = self
            .table
            .states
            .iter()
            .map(|x| x.actions.iter().filter(|x| !x.is_empty()).count())
            .max()
            .unwrap();
        ast.push(parse_quote! {
            pub struct #parser_definition {
                actions: [[Action; TERMINAL_COUNT]; STATE_COUNT],
                gotos: [[Option<StateIndex>; NONTERMINAL_COUNT]; STATE_COUNT],
                token_recognizers: [[Option<TokenRecognizer>; #max_actions]; STATE_COUNT]
            }

        });

        let actions: Vec<syn::Expr> = self
            .table
            .states
            .iter()
            .map(|state| {
                let actions_for_state: Vec<syn::Expr> = state
                    .actions
                    .iter()
                    .map(|action| match action.len() {
                        0 => parse_quote! { Error },
                        1 => action_to_syntax(&action[0]),
                        _ => panic!("Multiple actions for state {}", state.idx),
                    })
                    .collect();
                parse_quote! {
                    [#(#actions_for_state),*]
                }
            })
            .collect();

        let gotos: Vec<syn::Expr> = self
            .table
            .states
            .iter()
            .map(|state| {
                let gotos_for_state: Vec<syn::Expr> = state
                    .gotos
                    .iter()
                    .map(|x| match x {
                        Some(state) => {
                            let idx = state.0;
                            parse_quote! { Some(StateIndex(#idx))}
                        }
                        None => parse_quote! { None },
                    })
                    .collect();
                parse_quote! {
                    [#(#gotos_for_state),*]
                }
            })
            .collect();

        let terminals_for_state: Vec<syn::Expr> = self
            .table
            .states
            .iter()
            .map(|state| {
                let terminals: Vec<syn::Expr> = state
                    .sorted_terminals
                    .iter()
                    .map(|x| {
                        let term: &Terminal = &self.grammar.terminals[*x];
                        let token_kind = format_ident!("{}", &term.name);
                        if let LexerType::Default = self.settings.lexer_type {
                            let recognizer: syn::Expr = match term.recognizer {
                                Some(ref rec) => {
                                    match rec {
                                        Recognizer::StrConst(ref s) =>
                                            {
                                                let s: &String = s.as_ref();
                                                parse_quote! { Recognizer::StrMatch(#s) }
                                            }
                                        Recognizer::RegexTerm(_) =>
                                            {
                                                let idx: usize = (*x).into();
                                                parse_quote! { Recognizer::RegexMatch(#idx) }
                                            }
                                    }
                                },
                                None => if term.idx == TermIndex(0) {
                                    parse_quote! { Recognizer::Stop }
                                } else {
                                    panic!("This shouldn't happen. Recognizer must be defined!");
                                }
                            };
                            parse_quote! {
                                Some(TokenRecognizer{
                                    token_kind: TokenKind::#token_kind,
                                    recognizer: #recognizer,
                                    finish: true
                                })
                            }
                        } else {
                            parse_quote! {
                                Some(TokenRecognizer{
                                    token_kind: TokenKind::#token_kind,
                                })
                            }
                        }
                    })
                    .chain(
                        // Fill the rest with "None"
                        repeat(parse_quote! {None})
                            .take(max_actions - state.sorted_terminals.len()),
                    )
                    .collect();

                parse_quote! {
                    [#(#terminals),*]
                }
            })
            .collect();

        ast.push(
        parse_quote! {
            pub(in crate) static PARSER_DEFINITION: #parser_definition = #parser_definition {
                actions: [#(#actions),*],
                gotos: [#(#gotos),*],
                token_recognizers: [#(#terminals_for_state),*],
            };
        });

        ast.push(
        parse_quote! {
            impl ParserDefinition<TokenRecognizer> for #parser_definition {
                fn action(&self, state_index: StateIndex, term_index: TermIndex) -> Action {
                    PARSER_DEFINITION.actions[state_index.0][term_index.0]
                }
                fn goto(&self, state_index: StateIndex, nonterm_index: NonTermIndex) -> StateIndex {
                    PARSER_DEFINITION.gotos[state_index.0][nonterm_index.0].unwrap()
                }

                fn recognizers(&self, state_index: StateIndex) -> Vec<&TokenRecognizer> {
                    PARSER_DEFINITION.token_recognizers[state_index.0]
                        .iter()
                        .map_while(|tr| tr.as_ref())
                        .collect()
                }
            }
        });

        let partial_parse: syn::Expr = if self.settings.partial_parse {
            parse_quote! { true }
        } else {
            parse_quote! { false }
        };

        let lexer_type: syn::Type =
            if let LexerType::Default = self.settings.lexer_type {
                parse_quote! { StringLexer }
            } else {
                parse_quote! { L }
            };

        let mut parse_stmt: Vec<syn::Stmt> = vec![];
        if self.grammar.has_layout() {
            parse_stmt.push(parse_quote! {
                loop {
                    log!("** Parsing content");
                    let result = parser.parse(context, lexer, builder);
                    if result.is_err() {
                        let pos = context.position;
                        log!("** Parsing layout");
                        let mut builder = SliceBuilder::new();
                        context.layout_ahead = <LRParser<#parser_definition, TokenRecognizer>
                                                as rustemo::parser::Parser<'_, Input, #lexer_type,
                                                    SliceBuilder<'_, Input>,
                                                    TokenRecognizer>>::parse(&mut #layout_parser::default().0,
                                                                       context, lexer,
                                                                       &mut builder).unwrap_or_default();
                        if context.position > pos {
                            continue;
                        }
                    }
                    return result;
                }
            });
        } else {
            let ret_expr: syn::Expr = parse_quote! {
                parser.parse(context, lexer, builder)
            };
            parse_stmt.push(syn::Stmt::Expr(ret_expr));
        }

        let skip_ws = self.settings.skip_ws && !self.grammar.has_layout();

        let parse_result: syn::Type = match self.settings.builder_type {
            BuilderType::Default => parse_quote! {
                <DefaultBuilder as Builder>::Output
            },
            BuilderType::Generic => parse_quote! {
                <TreeBuilder<'i, Input, TokenKind> as Builder>::Output
            },
            BuilderType::Custom => parse_quote! {
                B::Output
            },
        };

        let mut lexer_instance: Vec<syn::Stmt> = vec![];
        match self.settings.lexer_type {
            LexerType::Default => {
                lexer_instance.push(parse_quote! {
                    let local_lexer = StringLexer::new(#skip_ws);
                });
                lexer_instance.push(parse_quote! {
                    let lexer = &local_lexer;
                })
            }
            LexerType::Custom => lexer_instance.push(parse_quote! {
                let lexer = &self.lexer;
            }),
        };

        let mut builder_instance: Vec<syn::Stmt> = vec![];
        match self.settings.builder_type {
            BuilderType::Default => {
                builder_instance.push(
                    parse_quote!{let mut local_builder = DefaultBuilder::new();}
                );
                builder_instance.push(
                    parse_quote!{let builder = &mut local_builder;}
                )
            },
            BuilderType::Generic => {
                builder_instance.push(
                    parse_quote!{let mut local_builder = TreeBuilder::new();}
                );
                builder_instance.push(
                    parse_quote!{let builder = &mut local_builder;}
                )
            },
            BuilderType::Custom =>
                // In case of the custom builder we use "interior mutability"
                // pattern thorough RefCell type as we need parser to use shared
                // reference while the inner builder needs to be mutable as it
                // keeps the output of the build process. But, we know for sure
                // that there will be only one mutable reference to the builder.
                builder_instance.push(parse_quote!{let builder = &mut *self.builder.borrow_mut();})
        };

        let mut new_parameters: Vec<syn::FnArg> = vec![];
        let mut parser_fields: Vec<syn::Field> = vec![];
        let mut parser_fields_values: Vec<syn::FieldValue> = vec![];
        let mut parser_generics: syn::Generics = parse_quote! {};
        let mut parser_impl_generics: syn::Generics = parse_quote! {};
        parser_impl_generics.params.push(parse_quote! { 'i });
        if let LexerType::Custom = self.settings.lexer_type {
            new_parameters.push(parse_quote! { lexer: L });
            parser_fields.push(
                syn::Field::parse_named.parse2(quote! { lexer: L }).unwrap(),
            );
            parser_fields_values.push(parse_quote! { lexer });
            parser_generics.params.push(parse_quote! { L });
            parser_impl_generics
                .params
                .push(parse_quote! { L: Lexer<Input, TokenRecognizer> });
        }
        if let BuilderType::Custom = self.settings.builder_type {
            new_parameters.push(parse_quote! { builder: B });
            parser_fields.push(
                syn::Field::parse_named
                    .parse2(quote! { builder: RefCell<B> })
                    .unwrap(),
            );
            parser_fields_values
                .push(parse_quote! { builder: RefCell::new(builder) });
            parser_generics.params.push(parse_quote! { B });
            parser_impl_generics
                .params
                .push(parse_quote! { B: LRBuilder<'i, Input, TokenKind> });
        }

        ast.push(parse_quote! {
            #[derive(Default)]
            pub struct #parser #parser_generics{
                content: Option<<Input as ToOwned>::Owned>,
                #(#parser_fields),*
            }
        });

        ast.push(parse_quote! {
            #[allow(dead_code)]
            impl #parser_impl_generics #parser #parser_generics
            {
                pub fn new(#(#new_parameters),*) -> Self {
                    Self {
                        content: None,
                        #(#parser_fields_values),*
                    }
                }

                #[allow(clippy::needless_lifetimes)]
                pub fn parse_file<P: AsRef<std::path::Path>>(&'i mut self, file: P)
                                                             -> Result<#parse_result> {
                    self.content = Some(<Input as rustemo::lexer::Input>::read_file(&file)?);
                    let mut context = Context::new(
                        file.as_ref().to_string_lossy().to_string(),
                        self.content.as_ref().unwrap());
                    self.inner_parse(&mut context)
                }
                #[allow(clippy::needless_lifetimes)]
                pub fn parse(&self, input: &'i Input) -> Result<#parse_result> {
                    let mut context = Context::new("<str>".to_string(), input);
                    self.inner_parse(&mut context)
                }
                #[allow(clippy::needless_lifetimes)]
                fn inner_parse(&self, context: &mut Context<'i>) -> Result<#parse_result> {
                    #(#lexer_instance);*
                    #(#builder_instance);*
                    let mut parser = LRParser::new(&PARSER_DEFINITION, StateIndex(0), #partial_parse);
                    #(#parse_stmt)*
                }
            }
        });

        Ok(ast)
    }

    fn generate_layout_parser(&self) -> Result<Vec<syn::Item>> {
        create_idents!(self, parser_definition, layout_parser,);
        let layout_state = &self.table.layout_state.expect("No Layout state!");
        let mut ast: Vec<syn::Item> = vec![];
        let layout_state = layout_state.0;
        let layout_state: syn::Expr =
            parse_quote! { StateIndex(#layout_state) };

        ast.push(parse_quote! {
            pub struct #layout_parser(LRParser<#parser_definition, TokenRecognizer>);
        });

        ast.push(parse_quote! {
            impl Default for #layout_parser {
                fn default() -> Self {
                    Self(LRParser::new(&PARSER_DEFINITION, #layout_state, true))
                }
            }
        });
        Ok(ast)
    }

    fn generate_lexer_definition(&self) -> Result<Vec<syn::Item>> {
        let mut ast: Vec<syn::Item> = vec![];

        if let LexerType::Default = self.settings.lexer_type {
            let regex_recognizers: Vec<syn::Expr> = self
                .grammar
                .terminals
                .iter()
                .map(|term| {
                    if let Some(Recognizer::RegexTerm(r)) = &term.recognizer {
                        let r = r.as_ref();
                        parse_quote! {
                            Some(Lazy::new(|| {
                                Regex::new(concat!("^", #r)).unwrap()
                            }))
                        }
                    } else {
                        parse_quote! { None }
                    }
                })
                .collect();

            ast.push(parse_quote!{
                pub(crate) static RECOGNIZERS: [Option<Lazy<Regex>>; TERMINAL_COUNT]  = [
                    #(#regex_recognizers,)*
                ];
            });

            ast.push(parse_quote! {
                #[allow(dead_code)]
                #[derive(Debug)]
                pub enum Recognizer {
                    Stop,
                    StrMatch(&'static str),
                    RegexMatch(usize)
                }
            });
            ast.push(parse_quote! {
                #[derive(Debug)]
                pub struct TokenRecognizer {
                    token_kind: TokenKind,
                    recognizer: Recognizer,
                    finish: bool
                }
            });
            ast.push(parse_quote!{
                impl lexer::TokenRecognizer for TokenRecognizer {
                    type TokenKind = TokenKind;
                    type Input = str;

                    fn recognize<'i>(&self, input: &'i str) -> Option<&'i str> {
                        match &self.recognizer {
                            Recognizer::StrMatch(s) => {
                                logn!("Recognizing <{:?}> -- ", self.token_kind());
                                if input.starts_with(s){
                                    log!("recognized");
                                    Some(s)
                                } else {
                                    log!("not recognized");
                                    None
                                }
                            },
                            Recognizer::RegexMatch(r) => {
                                logn!("Recognizing <{:?}> -- ", self.token_kind());
                                let match_str = RECOGNIZERS[*r].as_ref().unwrap().find(input);
                                match match_str {
                                    Some(x) => {
                                        let x_str = x.as_str();
                                        log!("recognized <{}>", x_str);
                                        Some(x_str)
                                    },
                                    None => {
                                        log!("not recognized");
                                        None
                                    }
                                }
                            },
                            Recognizer::Stop=> {
                                logn!("Recognizing <STOP> -- ");
                                if input.is_empty() {
                                    log!("recognized");
                                    Some("")
                                } else {
                                    log!("not recognized");
                                    None
                                }
                            },
                        }
                    }

                    #[inline]
                    fn token_kind(&self) -> TokenKind {
                        self.token_kind
                    }

                    #[inline]
                    fn finish(&self) -> bool {
                        self.finish
                    }
                }
            })
        } else {
            ast.push(parse_quote! {
                #[allow(dead_code)]
                #[derive(Debug)]
                pub struct TokenRecognizer {
                    pub token_kind: TokenKind,
                }
            });
        }

        ast.push(parse_quote! {
            impl PartialEq for TokenRecognizer {
                fn eq(&self, other: &Self) -> bool {
                    self.token_kind == other.token_kind
                }
            }
        });

        ast.push(parse_quote! {
            impl Eq for TokenRecognizer {}
        });

        ast.push(parse_quote! {
            impl Hash for TokenRecognizer {
                fn hash<H: Hasher>(&self, state: &mut H) {
                        self.token_kind.hash(state);
                }
            }
        });

        Ok(ast)
    }

    fn generate_builder(&self, types: &SymbolTypes) -> Result<Vec<syn::Item>> {
        create_idents!(self, actions_file, root_symbol,);
        let mut ast: Vec<syn::Item> = vec![];
        let context_var = format_ident!("context");

        ast.push(parse_quote! {
            pub struct DefaultBuilder {
                res_stack: Vec<Symbol>,
            }
        });

        ast.push(parse_quote! {
            impl Builder for DefaultBuilder
            {
                type Output = #actions_file::#root_symbol;

                fn new() -> Self {
                    Self {
                        res_stack: vec![],
                    }
                }

                fn get_result(&mut self) -> Self::Output {
                    match self.res_stack.pop().unwrap() {
                        Symbol::NonTerminal(NonTerminal::#root_symbol(r)) => r,
                        _ => panic!("Invalid result on the parse stack!"),
                    }
                }
            }
        });

        let mut shift_match_arms: Vec<syn::Arm> =
            self.grammar.terminals[1..].iter().filter(|t| t.reachable.get())
                                              .map(|terminal| {
            let action = format_ident!("{}", to_snake_case(&terminal.name));
            let term = format_ident!("{}", terminal.name);
            if let Some(Recognizer::StrConst(_)) = terminal.recognizer {
                parse_quote!{
                    TokenKind::#term => Terminal::#term
                }
            } else {
                parse_quote!{
                    TokenKind::#term => Terminal::#term(#actions_file::#action(context, token))
                }
            }
        }).collect();

        if self.grammar.terminals[1..]
            .iter()
            .any(|t| !t.reachable.get())
        {
            shift_match_arms.push(parse_quote! {
                _ => panic!("Shift of unreachable terminal!")
            })
        }
        let shift_match_arms = shift_match_arms;

        let mut has_nonreachable_nonterminals = false;
        let mut reduce_match_arms: Vec<syn::Arm> =
            self.grammar.productions().iter()
                                      .filter_map(|production| {
                let nonterminal = &self.grammar.nonterminals[production.nonterminal];
                if !nonterminal.reachable.get() {
                    has_nonreachable_nonterminals = true;
                    return None
                }
                let rhs_len = production.rhs.len();
                let choice = &types.get_type(
                        nonterminal.idx.to_symbol_index(self.grammar.terminals.len()))
                                       .choices[production.ntidx];
                let action = format_ident!("{}", action_name(nonterminal, choice));

                let prod_kind = self.prod_kind_ident(production);
                let nonterminal = format_ident!("{}", nonterminal.name);

                if rhs_len == 0 {
                    // Handle EMPTY reduction
                    Some(parse_quote!{
                        ProdKind::#prod_kind => NonTerminal::#nonterminal(#actions_file::#action(#context_var))
                    })
                } else {
                    // Special handling of production with only str match terms in RHS
                    if production.rhs_with_content(self.grammar).is_empty() {
                        Some(parse_quote! {
                            ProdKind::#prod_kind => {
                                let _ = self.res_stack.split_off(self.res_stack.len()-#rhs_len).into_iter();
                                NonTerminal::#nonterminal(#actions_file::#action(#context_var))
                            }
                        })
                    } else {
                        let mut next_rep: Vec<syn::Expr> = repeat(
                            parse_quote!{ i.next().unwrap() }
                        ).take(rhs_len).collect();

                        let match_expr: syn::Expr = if rhs_len > 1 {
                            parse_quote!{ (#(#next_rep),*) }
                        } else {
                            next_rep.pop().unwrap()
                        };

                        let mut param_count = 0usize;
                        let match_lhs_items: Vec<syn::Expr> = production.rhs_symbols()
                                                .iter()
                                                .map( |&symbol| {
                            let param = format_ident!("p{}", param_count);
                            if self.grammar.symbol_has_content(symbol) {
                                param_count += 1;
                                if self.grammar.is_term(symbol){
                                    let terminal = format_ident!("{}", self.grammar.symbol_to_term(symbol).name);
                                    parse_quote!{ Symbol::Terminal(Terminal::#terminal(#param)) }
                                } else {
                                    let nonterminal = format_ident!("{}", self.grammar.symbol_to_nonterm(symbol).name);
                                    parse_quote!{ Symbol::NonTerminal(NonTerminal::#nonterminal(#param)) }
                                }
                            } else {
                                parse_quote! { _ }
                            }
                        }).collect();

                        let match_lhs: syn::Expr = if rhs_len > 1 {
                            parse_quote! { (#(#match_lhs_items),*) }
                        } else {
                            parse_quote! { #(#match_lhs_items),* }
                        };

                        let params: Vec<syn::Ident> = (0..production.rhs_with_content(self.grammar).len())
                            .map( |idx| format_ident! { "p{}", idx }).collect();

                        Some(parse_quote! {
                            ProdKind::#prod_kind => {
                                let mut i = self.res_stack.split_off(self.res_stack.len()-#rhs_len).into_iter();
                                match #match_expr {
                                    #match_lhs => NonTerminal::#nonterminal(#actions_file::#action(context, #(#params),*)),
                                    _ => panic!("Invalid symbol parse stack data.")
                                }

                            }
                        })
                    }
                }
        }).collect();

        if has_nonreachable_nonterminals {
            reduce_match_arms.push(parse_quote!(
                 _ => panic!("Reduce of unreachable nonterminal!")
            ))
        }
        let reduce_match_arms = reduce_match_arms;

        ast.push(parse_quote! {
            impl<'i> LRBuilder<'i, Input, TokenKind> for DefaultBuilder
            {

                #![allow(unused_variables)]
                fn shift_action(
                    &mut self,
                    #context_var: &mut Context<'i>,
                    token: Token<'i, Input, TokenKind>) {
                    let val = match token.kind {
                        TokenKind::STOP => panic!("Cannot shift STOP token!"),
                        #(#shift_match_arms),*
                    };
                    self.res_stack.push(Symbol::Terminal(val));
                }

                fn reduce_action(
                    &mut self,
                    #context_var: &mut Context<'i>,
                    prod_idx: ProdIndex,
                    _prod_len: usize) {
                    let prod = match ProdKind::from(prod_idx) {
                        #(#reduce_match_arms),*
                    };
                    self.res_stack.push(Symbol::NonTerminal(prod));
                }

            }
        });

        Ok(ast)
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
}

fn action_to_syntax(action: &Action) -> syn::Expr {
    match action {
        Action::Shift(state) => {
            let state = state.0;
            parse_quote! { Shift(StateIndex(#state)) }
        }
        Action::Reduce(prod, len, nonterm) => {
            let prod = prod.0;
            let nonterm = nonterm.0;
            parse_quote! { Reduce(ProdIndex(#prod), #len, NonTermIndex(#nonterm)) }
        }
        Action::Accept => parse_quote! { Accept },
    }
}

fn action_name(nonterminal: &NonTerminal, choice: &Choice) -> String {
    to_snake_case(format!("{}_{}", nonterminal.name, &choice.name))
}
