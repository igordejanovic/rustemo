pub(crate) mod actions;

use quote::format_ident;
use rustemo::index::{StateIndex, TermIndex};
use std::{
    iter::repeat,
    path::{Path, PathBuf},
};
use syn::parse_quote;

use crate::{
    api::{settings::Settings, BuilderType, LexerType},
    error::{Error, Result},
    grammar::{
        types::{choice_name, to_pascal_case, to_snake_case},
        Grammar, NonTerminal, Production,
    },
    lang::rustemo_actions::Recognizer,
    table::{Action, LRTable},
};

use self::actions::generate_parser_actions;

fn action_name(nonterminal: &NonTerminal, prod: &Production) -> syn::Ident {
    format_ident!(
        "{}",
        to_snake_case(format!("{}_{}", nonterminal.name, choice_name(prod)))
    )
}

fn prod_kind(grammar: &Grammar, prod: &Production) -> String {
    format!(
        "{}{}",
        prod.nonterminal(grammar).name,
        if let Some(ref kind) = prod.kind {
            kind.clone()
        } else {
            format!("P{}", prod.ntidx + 1)
        }
    )
}

fn prod_kind_ident(grammar: &Grammar, prod: &Production) -> syn::Ident {
    format_ident!("{}", prod_kind(grammar, prod))
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
    let file_name = grammar_path.file_name().ok_or_else(|| {
        Error::Error("Invalid grammar file name.".to_string())
    })?;

    let grammar_dir = PathBuf::from(
        grammar_path
            .parent()
            .expect("Cannot deduce parent directory of the grammar file."),
    );

    let out_dir = match out_dir {
        Some(dir) => dir,
        None => &grammar_dir,
    };
    let out_dir_actions = match out_dir_actions {
        Some(dir) => dir,
        None => &grammar_dir,
    };

    let grammar_input = std::fs::read_to_string(grammar_path)?;
    let grammar: Grammar = grammar_input.parse()?;

    let table = LRTable::new(&grammar, settings);

    let conflicts = table.get_conflicts();
    if !conflicts.is_empty() {
        table.print_conflicts_report(&conflicts);
        return Err(Error::Error(
            "Grammar is not deterministic. There are conflicts.".to_string(),
        ));
    }

    // Generate parser definition
    let out_file = out_dir.join(file_name).with_extension("rs");
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
    let parser = format!("{}Parser", parser_name);
    let layout_parser = format!("{}LayoutParser", parser_name);
    let builder = format!("{}Builder", parser_name);
    let layout_builder = format!("{}LayoutBuilder", parser_name);
    let parser_definition = format!("{}Definition", parser);
    let lexer = format!("{}Lexer", parser_name);
    let lexer_definition = format!("{}Definition", lexer);
    let actions_file = format!("{}_actions", file_name);
    let lexer_file = format!("{}_lexer", file_name);
    let builder_file = format!("{}_builder", file_name);
    let root_symbol = grammar.symbol_name(grammar.start_index);

    let mut ast: syn::File = generate_parser_header(
        &grammar,
        &table,
        &actions_file,
        &lexer_file,
        &lexer,
        &builder_file,
        &builder,
        settings,
    )?;

    ast.items.extend(generate_parser_types(&grammar)?);

    if let BuilderType::Default = settings.builder_type {
        ast.items
            .extend(generate_parser_symbols(&grammar, &actions_file)?);
    }

    ast.items.extend(generate_parser_definition(
        &grammar,
        &table,
        &parser,
        &layout_parser,
        &parser_definition,
        file_name,
        &lexer,
        &builder_file,
        &builder,
        &actions_file,
        &root_symbol,
        settings,
    )?);

    if grammar.has_layout() {
        ast.items.extend(generate_layout_parser(
            &layout_parser,
            &parser_definition,
            &layout_builder,
            table.layout_state.unwrap(),
        )?);

        ast.items
            .extend(generate_layout_builder(&grammar, &layout_builder)?);
    }

    if let LexerType::Default = settings.lexer_type {
        ast.items.extend(generate_lexer_definition(
            &grammar,
            &table,
            &lexer_definition,
        )?);
    }

    if let BuilderType::Default = settings.builder_type {
        ast.items.extend(generate_builder(
            &grammar,
            &builder,
            &actions_file,
            &root_symbol,
        )?);

        // Generate actions
        if settings.actions {
            generate_parser_actions(
                &grammar,
                file_name,
                out_dir_actions,
                settings,
            )?;
        }
    }

    std::fs::create_dir_all(out_dir).map_err(|e| {
        Error::Error(format!(
            "Cannot create folders for path '{out_dir:?}': {e:?}."
        ))
    })?;
    std::fs::write(&out_file, prettyplease::unparse(&ast)).map_err(|e| {
        Error::Error(format!("Cannot write parser file '{out_file:?}': {e:?}."))
    })?;

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn generate_parser_header(
    grammar: &Grammar,
    table: &LRTable,
    actions_file: &str,
    lexer_file: &str,
    lexer: &str,
    builder_file: &str,
    builder: &str,
    settings: &Settings,
) -> Result<syn::File> {
    let max_actions = table
        .states
        .iter()
        .map(|x| x.actions.iter().filter(|x| !x.is_empty()).count())
        .max()
        .unwrap();

    let term_count = grammar.terminals.len();
    let nonterm_count = grammar.nonterminals.len();
    let states_count = table.states.len();
    let actions_file = format_ident!("{}", actions_file);
    let lexer_file = format_ident!("{}", lexer_file);
    let lexer = format_ident!("{}", lexer);
    let builder_file = format_ident!("{}", builder_file);
    let builder = format_ident!("{}", builder);

    let mut header: syn::File = parse_quote! {
        /// Generated by rustemo. Do not edit manually!
        use regex::Regex;
        use std::fmt::Debug;

        use rustemo::lexer::{self, Token, AsStr};
        use rustemo::parser::Parser;
        use rustemo::builder::Builder;
        use rustemo::Result;
        use rustemo::lr::lexer::{LRStringLexer, LexerDefinition, RecognizerIterator};
        use rustemo::lr::builder::LRBuilder;
        use rustemo::lr::parser::{LRParser, ParserDefinition};
        use rustemo::lr::parser::Action::{self, Shift, Reduce, Accept, Error};
        use rustemo::index::{StateIndex, TermIndex, NonTermIndex, ProdIndex};
        use rustemo::grammar::TerminalsState;
        use rustemo::debug::{log, logn};

        const TERMINAL_NO: usize = #term_count;
        const NONTERMINAL_NO: usize = #nonterm_count;
        const STATE_NO: usize = #states_count;
        #[allow(dead_code)]
        const MAX_ACTIONS: usize = #max_actions;

    };

    if let LexerType::Custom = settings.lexer_type {
        header.items.push(parse_quote! {
            use super::#lexer_file::#lexer;
        });
    }

    header.items.push(match settings.builder_type {
        BuilderType::Default => parse_quote! {
            use super::#actions_file;
        },
        BuilderType::Generic => parse_quote! {
            use rustemo::lr::builder::{TreeNode, TreeBuilder as #builder};
        },
        BuilderType::Custom => parse_quote! {
            use super::#builder_file::{self, #builder};
        },
    });

    header.items.push(match settings.lexer_type {
        LexerType::Default => parse_quote! {
            pub type Input = str;
        },
        LexerType::Custom => parse_quote! {
            use super::#lexer_file::Input;
        },
    });

    header.items.push(parse_quote! {
        pub type Context<'i> = lexer::Context<'i, Input, StateIndex>;
    });

    // Lazy init of regexes
    let (regex_names, regex_matches): (Vec<_>, Vec<_>) = grammar
        .terminals
        .iter()
        .filter_map(|t| {
            if let Some(Recognizer::RegexTerm(regex_match)) = &t.recognizer {
                let regex_name =
                    format_ident!("REGEX_{}", t.name.to_uppercase());
                Some((regex_name, regex_match))
            } else {
                None
            }
        })
        .unzip();
    if !regex_names.is_empty() {
        header.items.push(parse_quote! {
            use lazy_static::lazy_static;
        });
        header.items.push(parse_quote! {
           lazy_static! {
               #(static ref #regex_names: Regex = Regex::new(concat!("^", #regex_matches)).unwrap();
               )*
           }
        })
    }

    Ok(header)
}

fn generate_parser_types(grammar: &Grammar) -> Result<Vec<syn::Item>> {
    let mut ast: Vec<syn::Item> = vec![];

    let term_kind_variants: Vec<syn::Variant> = grammar.terminals[1..]
        .iter()
        .map(|t| {
            let name = format_ident!("{}", t.name);
            parse_quote! { #name }
        })
        .collect();

    ast.push(parse_quote! {
        #[allow(clippy::upper_case_acronyms)]
        #[derive(Debug, Copy, Clone)]
        pub enum TokenKind {
            #(#term_kind_variants),*
        }
    });

    let as_str_arms: Vec<syn::Arm> = grammar.terminals[1..]
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

    let (from_arms, into_arms): (Vec<syn::Arm>, Vec<syn::Arm>) = grammar
        .terminals[1..]
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

    let prodkind_variants: Vec<syn::Variant> = grammar
        .productions()
        .iter()
        .map(|prod| {
            let prod_kind = prod_kind_ident(grammar, prod);
            parse_quote! {#prod_kind}
        })
        .collect();
    ast.push(parse_quote! {
        #[allow(clippy::enum_variant_names)]
        #[derive(Copy, Clone)]
        pub enum ProdKind {
            #(#prodkind_variants),*
        }
    });

    let (as_str_arms, display_arms): (Vec<syn::Arm>, Vec<syn::Arm>) = grammar
        .productions()
        .iter()
        .map(|&prod| {
            let prod_kind = prod_kind(grammar, prod);
            let prod_kind_ident = prod_kind_ident(grammar, prod);
            let prod_str = prod.to_string(grammar);
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

    let from_arms: Vec<syn::Arm> = grammar
        .productions()
        .iter()
        .map(|&prod| {
            let prod_kind = prod_kind_ident(grammar, prod);
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

fn generate_parser_symbols(
    grammar: &Grammar,
    actions_file: &str,
) -> Result<Vec<syn::Item>> {
    let mut ast: Vec<syn::Item> = vec![];
    let actions_file = format_ident!("{}", actions_file);

    ast.push(parse_quote! {
        #[derive(Debug)]
        pub enum Symbol {
            Terminal(Terminal),
            NonTerminal(NonTerminal)
        }
    });

    let term_variants: Vec<syn::Variant> = grammar.terminals[1..]
        .iter()
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

    let nonterm_variants: Vec<syn::Variant> = grammar
        .nonterminals()
        .iter()
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

#[allow(clippy::too_many_arguments)]
fn generate_parser_definition(
    grammar: &Grammar,
    table: &LRTable,
    parser: &str,
    layout_parser: &str,
    parser_definition: &str,
    parser_file: &str,
    lexer: &str,
    builder_file: &str,
    builder: &str,
    actions_file: &str,
    root_symbol: &str,
    settings: &Settings,
) -> Result<Vec<syn::Item>> {
    let mut ast: Vec<syn::Item> = vec![];
    let parser = format_ident!("{parser}");
    let layout_parser = format_ident!("{layout_parser}");
    let parser_definition = format_ident!("{parser_definition}");
    let parser_file = format_ident!("{parser_file}");
    let lexer = format_ident!("{lexer}");
    let builder_file = format_ident!("{builder_file}");
    let builder = format_ident!("{builder}");
    let actions_file = format_ident!("{actions_file}");
    let root_symbol = format_ident!("{root_symbol}");

    ast.push(parse_quote! {
        pub struct #parser_definition {
            actions: [[Action; TERMINAL_NO]; STATE_NO],
            gotos: [[Option<StateIndex>; NONTERMINAL_NO]; STATE_NO]
        }

    });

    let actions: Vec<syn::Expr> = table
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

    let gotos: Vec<syn::Expr> = table
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

    ast.push(
        parse_quote! {
            pub(in crate) static PARSER_DEFINITION: #parser_definition = #parser_definition {
                actions: [#(#actions),*],
                gotos: [#(#gotos),*],
            };
        });

    ast.push(
        parse_quote! {
            impl ParserDefinition for #parser_definition {
                fn action(&self, state_index: StateIndex, term_index: TermIndex) -> Action {
                    PARSER_DEFINITION.actions[state_index.0][term_index.0]
                }
                fn goto(&self, state_index: StateIndex, nonterm_index: NonTermIndex) -> StateIndex {
                    PARSER_DEFINITION.gotos[state_index.0][nonterm_index.0].unwrap()
                }
            }
        });

    ast.push(parse_quote! {
        pub struct #parser(LRParser<#parser_definition>);
    });

    let partial_parse: syn::Expr = if settings.partial_parse {
        parse_quote! { true }
    } else {
        parse_quote! { false }
    };

    let mut parse_stmt: Vec<syn::Stmt> = vec![];
    if grammar.has_layout() {
        parse_stmt.push(parse_quote! {
            let mut parser = #parser::default();
        });
        parse_stmt.push(parse_quote! {
            loop {
                log!("** Parsing content");
                let result = parser.0.parse(&mut context, &lexer, &mut builder);
                if result.is_err() {
                    let pos = context.position;
                    log!("** Parsing layout");
                    if #layout_parser::parse_layout(&mut context) && context.position > pos {
                        continue;
                    }
                }
                return result;
            }
        });
    } else {
        let ret_expr: syn::Expr = parse_quote! {
            #parser::default().0.parse(&mut context, &lexer, &mut builder)
        };
        parse_stmt.push(syn::Stmt::Expr(ret_expr));
    }

    let skip_ws = settings.skip_ws && !grammar.has_layout();

    let parse_result: syn::Type = match settings.builder_type {
        BuilderType::Default => parse_quote! {
            Result<#actions_file::#root_symbol>
        },
        BuilderType::Generic => parse_quote! {
            Result<TreeNode<str, super::#parser_file::TokenKind>>
        },
        BuilderType::Custom => parse_quote! {
            Result<#builder_file::#root_symbol>
        },
    };

    let lexer_instance: syn::Stmt = match settings.lexer_type {
        LexerType::Default => parse_quote! {
            let lexer = LRStringLexer::new(&LEXER_DEFINITION, #partial_parse, #skip_ws);
        },
        LexerType::Custom => parse_quote! {
            let lexer = #lexer::new();
        },
    };

    ast.push(parse_quote! {
        #[allow(dead_code)]
        impl #parser
        {
            pub fn parse(input: &Input) -> #parse_result {
                let mut context = Context::new("<str>".to_string(), input);
                #lexer_instance
                let mut builder = #builder::new();
                #(#parse_stmt)*
            }
        }
    });

    ast.push(parse_quote! {
        impl Default for #parser {
            fn default() -> Self {
                Self(LRParser::new(&PARSER_DEFINITION, StateIndex(0)))
            }
        }
    });

    Ok(ast)
}

fn generate_layout_parser(
    layout_parser: &str,
    parser_definition: &str,
    layout_builder: &str,
    layout_state: StateIndex,
) -> Result<Vec<syn::Item>> {
    let mut ast: Vec<syn::Item> = vec![];
    let layout_parser = format_ident!("{}", layout_parser);
    let parser_definition = format_ident!("{}", parser_definition);
    let layout_builder = format_ident!("{}", layout_builder);
    let layout_state = layout_state.0;
    let layout_state: syn::Expr = parse_quote! { StateIndex(#layout_state) };

    ast.push(parse_quote! {
        pub struct #layout_parser(LRParser<#parser_definition>);
    });

    ast.push(
        parse_quote! {
            #[allow(dead_code)]
            impl #layout_parser
            {
                pub fn parse_layout(context: &mut Context) -> bool {
                    let lexer = LRStringLexer::new(&LEXER_DEFINITION, true, false);
                    let mut builder = #layout_builder::new();
                    #layout_parser::default().0.parse(context, &lexer, &mut builder).is_ok()
                }
            }
        });

    ast.push(parse_quote! {
        impl Default for #layout_parser {
            fn default() -> Self {
                Self(LRParser::new(&PARSER_DEFINITION, #layout_state))
            }
        }
    });
    Ok(ast)
}

fn generate_lexer_definition(
    grammar: &Grammar,
    table: &LRTable,
    lexer_definition: &str,
) -> Result<Vec<syn::Item>> {
    let mut ast: Vec<syn::Item> = vec![];
    let lexer_definition = format_ident!("{}", lexer_definition);

    ast.push(parse_quote! {
        pub struct #lexer_definition {
            terminals_for_state: TerminalsState<MAX_ACTIONS, STATE_NO>,
            recognizers: [fn(&str) -> Option<&str>; TERMINAL_NO]
        }
    });

    let max_actions = table
        .states
        .iter()
        .map(|x| x.actions.iter().filter(|x| !x.is_empty()).count())
        .max()
        .unwrap();
    let terminals_for_state: Vec<syn::Expr> = table
        .states
        .iter()
        .map(|state| {
            let terminals: Vec<syn::Expr> = state
                .sorted_terminals
                .iter()
                .map(|x| {
                    let x = x.0;
                    parse_quote! { Some(#x) }
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

    let mut recognizers: Vec<syn::Expr> = vec![];
    for terminal in &grammar.terminals {
        let term_name = &terminal.name;
        let term_ident = format_ident!("REGEX_{}", term_name.to_uppercase());
        if let Some(recognizer) = &terminal.recognizer {
            match recognizer {
                Recognizer::StrConst(str_match) => {
                    recognizers.push(parse_quote! {
                        |input: &str| {
                            logn!("Recognizing <{}> -- ", #term_name);
                            if input.starts_with(#str_match){
                                log!("recognized");
                                Some(#str_match)
                            } else {
                                log!("not recognized");
                                None
                            }
                        }
                    });
                }
                Recognizer::RegexTerm(_) => {
                    recognizers.push(parse_quote! {
                        |input: &str| {
                            logn!("Recognizing <{}> -- ", #term_name);
                            let match_str = #term_ident.find(input);
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
                        }
                    });
                }
            }
        } else if terminal.idx == TermIndex(0) {
            recognizers.push(parse_quote! {
                |input: &str| {
                    logn!("Recognizing <STOP> -- ");
                    if input.is_empty() {
                        log!("recognized");
                        Some("")
                    } else {
                        log!("not recognized");
                        None
                    }
                }
            });
        } else {
            // TODO: Custom recognizers?
            unreachable!()
        }
    }

    ast.push(
        parse_quote!{
            #[allow(clippy::single_char_pattern)]
            pub(in crate) static LEXER_DEFINITION: #lexer_definition = #lexer_definition {
                terminals_for_state: [#(#terminals_for_state),*],
                recognizers: [#(#recognizers),*],
            };
        }
    );

    ast.push(
        parse_quote!{
            impl LexerDefinition for #lexer_definition {
                type Recognizer = for<'i> fn(&'i str) -> Option<&'i str>;

                fn recognizers(&self, state_index: StateIndex) -> RecognizerIterator<Self::Recognizer> {
                    RecognizerIterator {
                        terminals_for_state: &LEXER_DEFINITION.terminals_for_state[state_index.0][..],
                        recognizers: &LEXER_DEFINITION.recognizers,
                        index: 0
                    }
                }
            }
        }
    );

    Ok(ast)
}

fn generate_builder(
    grammar: &Grammar,
    builder: &str,
    actions_file: &str,
    root_symbol: &str,
) -> Result<Vec<syn::Item>> {
    let mut ast: Vec<syn::Item> = vec![];
    let builder = format_ident!("{}", builder);
    let actions_file = format_ident!("{}", actions_file);
    let root_symbol = format_ident!("{}", root_symbol);
    let context_var = format_ident!("context");

    ast.push(parse_quote! {
        struct #builder {
            res_stack: Vec<Symbol>,
        }
    });

    ast.push(parse_quote! {
        impl Builder for #builder
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

    let shift_match_arms: Vec<syn::Arm> = grammar.terminals[1..].iter().map(|terminal| {
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

    let reduce_match_arms: Vec<syn::Arm> = grammar.productions().iter().map(|production| {
        let nonterminal = &grammar.nonterminals[production.nonterminal];
        let rhs_len = production.rhs.len();
        let action = action_name(nonterminal, production);
        let prod_kind = prod_kind_ident(grammar, production);
        let nonterminal = format_ident!("{}", nonterminal.name);

        if rhs_len == 0 {
            // Handle EMPTY reduction
            parse_quote!{
                ProdKind::#prod_kind => NonTerminal::#nonterminal(#actions_file::#action(#context_var))
            }
        } else {
            // Special handling of production with only str match terms in RHS
            if production.rhs_with_content(grammar).is_empty() {
                parse_quote! {
                    ProdKind::#prod_kind => {
                        let _ = self.res_stack.split_off(self.res_stack.len()-#rhs_len).into_iter();
                        NonTerminal::#nonterminal(#actions_file::#action(#context_var))
                    }
                }
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
                    if grammar.symbol_has_content(symbol) {
                        param_count += 1;
                        if grammar.is_term(symbol){
                            let terminal = format_ident!("{}", grammar.symbol_to_term(symbol).name);
                            parse_quote!{ Symbol::Terminal(Terminal::#terminal(#param)) }
                        } else {
                            let nonterminal = format_ident!("{}", grammar.symbol_to_nonterm(symbol).name);
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

                let params: Vec<syn::Ident> = (0..production.rhs_with_content(grammar).len())
                    .map( |idx| format_ident! { "p{}", idx }).collect();

                parse_quote! {
                    ProdKind::#prod_kind => {
                        let mut i = self.res_stack.split_off(self.res_stack.len()-#rhs_len).into_iter();
                        match #match_expr {
                            #match_lhs => NonTerminal::#nonterminal(#actions_file::#action(context, #(#params),*)),
                            _ => panic!("Invalid symbol parse stack data.")
                        }

                    }
                }
            }
        }
    }).collect();

    ast.push(
        parse_quote! {
            impl<'i> LRBuilder<'i, Input, TokenKind> for #builder
            {

                #![allow(unused_variables)]
                fn shift_action(
                    &mut self,
                    #context_var: &mut Context<'i>,
                    token: Token<'i, Input, TokenKind>) {
                    let kind = match token.kind {
                        lexer::TokenKind::Kind(kind) => kind,
                        lexer::TokenKind::STOP => panic!("Cannot shift STOP token!"),
                    };
                    let val = match kind {
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
        }
    );

    Ok(ast)
}

fn generate_layout_builder(
    grammar: &Grammar,
    layout_builder: &str,
) -> Result<Vec<syn::Item>> {
    let mut ast: Vec<syn::Item> = vec![];
    let layout_builder = format_ident!("{}", layout_builder);

    // Find all ProdIndex for Layout rule.
    let prods = &grammar.nonterm_by_name("Layout").productions;
    let prods_len = prods.len();
    let prods: Vec<syn::Expr> = prods
        .iter()
        .map(|x| {
            let x = x.0;
            parse_quote!(ProdIndex(#x))
        })
        .collect();

    ast.push(parse_quote! {
        struct #layout_builder([ProdIndex;#prods_len]);
    });

    ast.push(parse_quote! {
        impl Builder for #layout_builder {
            type Output = ();

            fn new() -> Self {
                #layout_builder([#(#prods),*])
            }

            fn get_result(&mut self) -> Self::Output {
            }
        }
    });

    ast.push(parse_quote! {
        impl<'i> LRBuilder<'i, Input, TokenKind> for #layout_builder {
            fn shift_action(
                &mut self,
                _context: &mut Context<'i>,
                _token: Token<'i, Input, TokenKind>,
            ) {
            }

            fn reduce_action(
                &mut self,
                context: &mut Context<'i>,
                prod_idx: ProdIndex,
                _prod_len: usize,
            ) {
                if self.0.contains(&prod_idx) {
                    context.layout = Some(&context.input[context.start_pos..context.end_pos]);
                }
            }
        }
    });

    Ok(ast)
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
