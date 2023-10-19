pub(crate) mod actions;

use quote::format_ident;
use rustemo::parser::Parser;
use std::{
    fs,
    iter::repeat,
    path::{Path, PathBuf},
};
use syn::{parse_quote, Ident};

use crate::{
    error::{Error, Result},
    grammar::{
        types::{to_pascal_case, to_snake_case, Choice, SymbolTypes},
        Grammar, NonTerminal, Production, Terminal,
    },
    index::{StateIndex, TermIndex},
    lang::{rustemo::RustemoParser, rustemo_actions::Recognizer},
    settings::{BuilderType, LexerType, Settings},
    table::{Action, LRTable},
};
use crate::{grammar::builder::GrammarBuilder, ParserAlgo};

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
        let mut ast: Vec<syn::Stmt> = vec![];
        ast.extend(self.generate_parser_types()?);

        if let BuilderType::Default = self.settings.builder_type {
            ast.extend(self.generate_parser_symbols()?);
        }

        ast.extend(self.generate_parser_definition()?);

        if let LexerType::Default = self.settings.lexer_type {
            ast.extend(self.generate_lexer_definition()?);
        }

        if let BuilderType::Default = self.settings.builder_type {
            let types = SymbolTypes::new(grammar);
            ast.extend(self.generate_builder(&types)?);

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
                "Cannot create directories for path '{out_dir:?}': {e:?}."
            ))
        })?;

        let mut file: syn::File = self.generate_parser_header()?;
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

    fn generate_parser_header(&self) -> Result<syn::File> {
        create_idents!(self, actions_file,);

        let input_type: syn::Type = syn::parse_str(&self.settings.input_type)?;
        let max_actions = self.table.max_actions();
        let max_recognizers = self.table.max_recognizers();
        let term_count = self.grammar.terminals.len();
        let nonterm_count = self.grammar.nonterminals.len();
        let states_count = self.table.states.len();

        let mut imports: Vec<syn::Stmt> = vec![];

        if let LexerType::Default = self.settings.lexer_type {
            imports.extend::<Vec<syn::Stmt>>(parse_quote! {
                use regex::Regex;
                use once_cell::sync::Lazy;
                use rustemo::StringLexer;
            });
        }

        imports.push(parse_quote! {
            use rustemo::LRBuilder;
        });
        imports.extend::<Vec<syn::Stmt>>(match self.settings.builder_type {
            BuilderType::Default => parse_quote! {
                use super::#actions_file;
            },
            BuilderType::Generic => parse_quote! {
                use rustemo::{TreeNode, TreeBuilder};
            },
            BuilderType::Custom => parse_quote! {
                use std::cell::RefCell;
            },
        });

        imports.extend::<Vec<syn::Stmt>>(match self.settings.parser_algo {
            ParserAlgo::LR => parse_quote! {
                use rustemo::{LRParser, LRContext};
            },
            ParserAlgo::GLR => parse_quote! {
                use rustemo::{GlrParser, Forest, GssHead};
            },
        });

        let header: syn::File = parse_quote! {
            /// Generated by rustemo. Do not edit manually!
            use std::fmt::Debug;
            use std::hash::Hash;
            use std::rc::Rc;

            use rustemo::{Result, Input as InputT, Lexer, Token,
                          TokenRecognizer as TokenRecognizerT,
                          Parser, ParserDefinition, State as StateT, Builder};
            #(#imports)*
            use rustemo::Action::{self, Shift, Reduce, Accept, Error};
            #[allow(unused_imports)]
            use rustemo::debug::{log, logn};
            #[allow(unused_imports)]
            #[cfg(debug_assertions)]
            use colored::*;

            pub type Input = #input_type;
            const TERMINAL_COUNT: usize = #term_count;
            const NONTERMINAL_COUNT: usize = #nonterm_count;
            const STATE_COUNT: usize = #states_count;
            #[allow(dead_code)]
            const MAX_ACTIONS: usize = #max_actions;
            const MAX_RECOGNIZERS: usize = #max_recognizers;
        };

        Ok(header)
    }

    fn generate_parser_types(&self) -> Result<Vec<syn::Stmt>> {
        let mut ast: Vec<syn::Stmt> = vec![];

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
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub enum TokenKind {
                #[default]
                #(#token_kind_variants),*
            }
        });

        ast.push(parse_quote! {
            impl From<TokenKind> for usize {
                fn from(t: TokenKind) -> Self {
                    t as usize
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
            #[derive(Clone, Copy, PartialEq)]
            pub enum ProdKind {
                #(#prodkind_variants),*
            }
        });

        let display_arms: Vec<syn::Arm> = self
            .grammar
            .productions()
            .iter()
            .map(|&prod| {
                let prod_kind_ident = self.prod_kind_ident(prod);
                let prod_str = prod.to_string(self.grammar);
                parse_quote! { ProdKind::#prod_kind_ident => #prod_str }
            })
            .collect();
        ast.push(parse_quote! {
            impl std::fmt::Debug for ProdKind {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let name = match self {
                        #(#display_arms),*
                    };
                    write!(f, "{}", name)
                }
            }
        });

        let nonterm_kind_variants: Vec<syn::Variant> = self
            .grammar
            .nonterminals
            .iter()
            .map(|nt| {
                let nt_kind = format_ident!("{}", nt.name);
                parse_quote! {#nt_kind}
            })
            .collect();
        ast.push(parse_quote! {
            #[allow(clippy::upper_case_acronyms)]
            #[allow(dead_code)]
            #[derive(Clone, Copy, Debug)]
            pub enum NonTermKind {
                #(#nonterm_kind_variants),*
            }
        });

        let from_arms: Vec<syn::Arm> = self
            .grammar
            .productions()
            .iter()
            .map(|&prod| {
                let prod_kind = self.prod_kind_ident(prod);
                let nt_kind =
                    format_ident!("{}", prod.nonterminal(self.grammar).name);
                parse_quote! { ProdKind::#prod_kind => NonTermKind::#nt_kind }
            })
            .collect();
        ast.push(parse_quote! {
            impl From<ProdKind> for NonTermKind {
                fn from(prod: ProdKind) -> Self {
                    match prod {
                        #(#from_arms),*,
                    }
                }
            }
        });

        let state_variants: Vec<syn::Variant> = self
            .table
            .states
            .iter()
            .map(|state| {
                let state_kind = self.state_kind_ident(state.idx);
                parse_quote! {#state_kind}
            })
            .collect();
        ast.push(parse_quote! {
            #[allow(clippy::enum_variant_names)]
            #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
            pub enum State {
                #[default]
                #(#state_variants),*
            }
        });

        let layout_state =
            self.table.layout_state.map(|s| self.state_kind_ident(s));
        let layout_state: syn::Expr = match layout_state {
            Some(state) => parse_quote! { Some(State::#state) },
            None => parse_quote! { None },
        };
        ast.push(parse_quote! {
            impl StateT for State {
                fn default_layout() -> Option<Self> {
                    #layout_state
                }
            }
        });

        ast.push(parse_quote! {
            impl From<State> for usize {
                fn from(s: State) -> Self {
                    s as usize
                }
            }
        });

        let state_display_arms: Vec<syn::Arm> = self
            .table
            .states
            .iter()
            .map(|state| {
                let state_kind_ident = self.state_kind_ident(state.idx);
                let state_index_str = format!(
                    "{}:{}",
                    state.idx,
                    self.grammar.symbol_name(state.symbol)
                );
                parse_quote! { State::#state_kind_ident => #state_index_str }
            })
            .collect();

        ast.push(parse_quote!{
            impl std::fmt::Debug for State {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let name = match self {
                        #(#state_display_arms),*,
                    };
                    write!(f, "{name}")
                }
            }
        });

        Ok(ast)
    }

    fn generate_parser_symbols(&self) -> Result<Vec<syn::Stmt>> {
        create_idents!(self, actions_file);

        let mut ast: Vec<syn::Stmt> = vec![];

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

    fn generate_parser_definition(&self) -> Result<Vec<syn::Stmt>> {
        create_idents!(self, parser, parser_definition,);
        let mut ast: Vec<syn::Stmt> = vec![];

        ast.push(parse_quote! {
            pub struct #parser_definition {
                actions: [[[Action<State, ProdKind>; MAX_ACTIONS]; TERMINAL_COUNT]; STATE_COUNT],
                gotos: [[Option<State>; NONTERMINAL_COUNT]; STATE_COUNT],
                token_kinds: [[Option<TokenKind>; MAX_RECOGNIZERS]; STATE_COUNT],
            }
        });

        let max_actions = self.table.max_actions();
        let actions: Vec<syn::Expr> = self
            .table
            .states
            .iter()
            .map(|state| {
                let actions_for_state: Vec<syn::Expr> = state
                    .actions
                    .iter()
                    .map(|action| {
                        // Create a vector of actions and add `Empty` up to the max_actions
                        // as the actions are generated in static arrays of the fixed length
                        let l = action.len();
                        let actions: Vec<syn::Expr> = action
                            .iter()
                            .cloned()
                            .map(Some)
                            .chain(repeat(None).take(max_actions - l))
                            .map(|a| self.action_to_syntax(&a))
                            .collect();
                        parse_quote! {
                            [#(#actions),*]
                        }
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
                            let state_kind_ident =
                                self.state_kind_ident(*state);
                            parse_quote! { Some(State::#state_kind_ident) }
                        }
                        None => parse_quote! { None },
                    })
                    .collect();
                parse_quote! {
                    [#(#gotos_for_state),*]
                }
            })
            .collect();

        let max_recognizers = self.table.max_recognizers();
        let token_kinds: Vec<syn::Expr> = self
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
                        parse_quote! {
                            Some(TokenKind::#token_kind)
                        }
                    })
                    .chain(
                        // Fill the rest with "None"
                        repeat(parse_quote! {None}).take(
                            max_recognizers - state.sorted_terminals.len(),
                        ),
                    )
                    .collect();

                parse_quote! {
                    [#(#terminals),*]
                }
            })
            .collect();

        ast.push(parse_quote! {
            pub(in crate) static PARSER_DEFINITION: #parser_definition = #parser_definition {
                actions: [#(#actions),*],
                gotos: [#(#gotos),*],
                token_kinds: [#(#token_kinds),*],
            };
        });

        ast.push(parse_quote! {
            impl ParserDefinition<State, ProdKind, TokenKind, NonTermKind> for #parser_definition {
                fn actions(&self, state: State, token: TokenKind) -> &'static [Action<State, ProdKind>] {
                    &PARSER_DEFINITION.actions[state as usize][token as usize]
                }
                fn goto(&self, state: State, nonterm: NonTermKind) -> State {
                    PARSER_DEFINITION.gotos[state as usize][nonterm as usize].unwrap()
                }
                fn expected_token_kinds(&self, state: State) -> &'static [Option<TokenKind>] {
                    &PARSER_DEFINITION.token_kinds[state as usize]
                }
            }
        });

        // Context type
        ast.push(match self.settings.parser_algo {
            ParserAlgo::LR => parse_quote!{
                pub(crate) type Context<'i, I> = LRContext<'i, I, State, TokenKind>;
            },
            ParserAlgo::GLR => parse_quote!{
                pub(crate) type Context<'i, I> = GssHead<'i, I, State, TokenKind>;
            },
        });

        let partial_parse: syn::Expr = if self.settings.partial_parse {
            parse_quote! { true }
        } else {
            parse_quote! { false }
        };

        let skip_ws = self.settings.skip_ws && !self.grammar.has_layout();

        // let parse_result: syn::Type = match self.settings.parser_algo {
        //     ParserAlgo::LR => match self.settings.builder_type {
        //         BuilderType::Default => parse_quote! {
        //             <DefaultBuilder as Builder<'i, Input>>::Output
        //         },
        //         BuilderType::Generic => parse_quote! {
        //             <TreeBuilder<'i, Input, ProdKind, TokenKind> as Builder<'i, Input>>::Output
        //         },
        //         BuilderType::Custom => parse_quote! {
        //             B::Output
        //         },
        //     },
        //     ParserAlgo::GLR => parse_quote! {
        //         Forest<'i, Input, ProdKind, TokenKind>
        //     },
        // };

        let lexer_instance: syn::Expr = match self.settings.lexer_type {
            LexerType::Default => parse_quote! {
                StringLexer::new(#skip_ws, &RECOGNIZERS)
            },
            LexerType::Custom => parse_quote! {
                lexer
            },
        };

        let builder_instance: syn::Expr = match self.settings.builder_type {
            BuilderType::Default => parse_quote! {
                DefaultBuilder::new()
            },
            BuilderType::Generic => parse_quote! {
                TreeBuilder::new()
            },
            BuilderType::Custom => {
                parse_quote! { builder }
            }
        };

        let has_layout = self.grammar.has_layout();
        let parser_instance: syn::Expr = match self.settings.parser_algo {
            ParserAlgo::LR => parse_quote! {
                LRParser::new(&PARSER_DEFINITION, State::default(), #partial_parse, #has_layout,
                              Rc::new(#lexer_instance), #builder_instance)
            },
            ParserAlgo::GLR => parse_quote! {
                GlrParser::new(&PARSER_DEFINITION, #partial_parse,
                               #has_layout, #lexer_instance)
            },
        };

        let mut new_parameters: Vec<syn::FnArg> = vec![];
        let mut parser_impl_generics: syn::Generics = parse_quote! {};
        let mut parser_type_params: Vec<syn::TypeParamBound> = vec![];
        let mut where_clause: Vec<syn::WherePredicate> = vec![];
        parser_impl_generics.params.push(parse_quote! { 'i });
        parser_type_params.push(parse_quote! { 'i });
        parser_type_params.push(parse_quote! { Input });
        match self.settings.lexer_type {
            LexerType::Default => {
                parser_type_params.push(parse_quote! {
                    StringLexer<Context<'i, Input>, State, TokenKind, TokenRecognizer,
                                TERMINAL_COUNT>
                });
            }
            LexerType::Custom => {
                parser_impl_generics.params.push(parse_quote! { L });
                parser_type_params.push(parse_quote! { L });
                where_clause.push(parse_quote!{L: Lexer<'i, Context<'i, Input>, State, TokenKind, Input = Input> });
                new_parameters.push(parse_quote! { lexer: L });
            }
        }
        match self.settings.builder_type {
            BuilderType::Default => {
                parser_type_params.push(parse_quote! { DefaultBuilder });
            }
            BuilderType::Generic => {
                parser_type_params.push(
                    parse_quote! { TreeBuilder<'i, Input, ProdKind, TokenKind> },
                );
            }
            BuilderType::Custom => {
                parser_impl_generics.params.push(parse_quote! { B });
                parser_type_params.push(parse_quote! { B });
                where_clause.push(
                    parse_quote! { B: LRBuilder<'i, Input, Context<'i, Input>,
                    State, ProdKind, TokenKind> },
                );
                new_parameters.push(parse_quote! { builder: B });
            }
        }

        let parser_type: syn::Type =
            if let ParserAlgo::LR = self.settings.parser_algo {
                parse_quote! {
                    LRParser<'i, Context<'i, I>, State, ProdKind,
                        TokenKind, NonTermKind, #parser_definition, L, B, I>
                }
            } else {
                parse_quote! {
                    GlrParser<'i, State, L, ProdKind, TokenKind, NonTermKind,
                              #parser_definition, I, B>
                }
            };
        ast.push(parse_quote! {
            pub struct #parser <'i, I: InputT + ?Sized, L: Lexer<'i, Context<'i, I>,
                                State, TokenKind, Input = I>, B>(#parser_type);
        });

        ast.push(if where_clause.is_empty() {
            parse_quote! {
                #[allow(dead_code)]
                impl #parser_impl_generics #parser <#(#parser_type_params),*>
                {
                    pub fn new(#(#new_parameters),*) -> Self {
                        Self(#parser_instance)
                    }
                }
            }
        } else {
            parse_quote! {
                #[allow(dead_code)]
                impl #parser_impl_generics #parser <#(#parser_type_params),*>
                where
                    #(#where_clause),*
                {
                    pub fn new(#(#new_parameters),*) -> Self {
                        Self(#parser_instance)
                    }
                }
            }
        });

        let output_type: syn::Type =
            if let ParserAlgo::GLR = self.settings.parser_algo {
                parse_quote! {
                    Forest<'i, I, ProdKind, TokenKind>
                }
            } else {
                parse_quote! {
                    B::Output
                }
            };

        ast.push(parse_quote! {
            #[allow(dead_code)]
            impl<'i, I, L, B> Parser<'i, I, Context<'i, I>, L, State, TokenKind> for #parser <'i, I, L, B>
            where
                I: InputT + ?Sized + Debug,
                L: Lexer<'i, Context<'i, I>, State, TokenKind, Input = I>,
                B: LRBuilder<'i, I, Context<'i, I>, State, ProdKind, TokenKind>
            {
                type Output = #output_type;

                fn parse(&self, input: &'i I) -> Result<Self::Output> {
                    self.0.parse(input)
                }

                fn parse_with_context(
                    &self,
                    context: &mut Context<'i, I>,
                    input: &'i I,
                ) -> Result<Self::Output> {
                    self.0.parse_with_context(context, input)
                }

                fn parse_file<'a, F: AsRef<std::path::Path>>(
                    &'a mut self,
                    file: F,
                ) -> Result<Self::Output>
                where
                    'a: 'i {
                    self.0.parse_file(file)
                }
            }
        });

        Ok(ast)
    }

    // fn generate_layout_parser(&self) -> Result<Vec<syn::Stmt>> {
    //     create_idents!(self, parser_definition, layout_parser,);
    //     let layout_state = &self.table.layout_state.expect("No Layout state!");
    //     let mut ast: Vec<syn::Stmt> = vec![];
    //     let layout_state = self.state_kind_ident(*layout_state);
    //     let layout_state: syn::Expr = parse_quote! { State::#layout_state };

    //     // TODO: If GLR is used LRParser can still be used to parse layout.
    //     ast.push(parse_quote! {
    //         pub struct #layout_parser(LRParser<State, ProdKind, TokenKind, NonTermKind, #parser_definition, TokenRecognizer>);
    //     });

    //     ast.push(parse_quote! {
    //         impl Default for #layout_parser {
    //             fn default() -> Self {
    //                 Self(LRParser::new(&PARSER_DEFINITION, #layout_state, true))
    //             }
    //         }
    //     });
    //     Ok(ast)
    // }

    fn generate_lexer_definition(&self) -> Result<Vec<syn::Stmt>> {
        let mut ast: Vec<syn::Stmt> = vec![];

        ast.push(parse_quote! {
            #[allow(dead_code)]
            #[derive(Debug)]
            pub enum Recognizer {
                Stop,
                StrMatch(&'static str),
                RegexMatch(Lazy<Regex>)
            }
        });
        ast.push(parse_quote! {
            #[allow(dead_code)]
            #[derive(Debug)]
            pub struct TokenRecognizer(TokenKind, Recognizer);
        });
        ast.push(parse_quote!{
            impl<'i> TokenRecognizerT<'i> for TokenRecognizer {
                fn recognize(&self, input: &'i str) -> Option<&'i str> {
                    match &self {
                        #[allow(unused_variables)]
                        TokenRecognizer(token_kind, Recognizer::StrMatch(s)) => {
                            logn!("{} {:?} -- ", "    Recognizing".green(), token_kind);
                            if input.starts_with(s){
                                log!("{}", "recognized".bold().green());
                                Some(s)
                            } else {
                                log!("{}", "not recognized".red());
                                None
                            }
                        },
                        #[allow(unused_variables)]
                        TokenRecognizer(token_kind, Recognizer::RegexMatch(r)) => {
                            logn!("{} {:?} -- ", "    Recognizing".green(), token_kind);
                            let match_str = r.find(input);
                            match match_str {
                                Some(x) => {
                                    let x_str = x.as_str();
                                    log!("{} '{}'", "recognized".bold().green(), x_str);
                                    Some(x_str)
                                },
                                None => {
                                    log!("{}", "not recognized".red());
                                    None
                                }
                            }
                        },
                        TokenRecognizer(_, Recognizer::Stop) => {
                            logn!("{} STOP -- ","    Recognizing".green());
                            if input.is_empty() {
                                log!("{}", "recognized".bold().green());
                                Some("")
                            } else {
                                log!("{}", "not recognized".red());
                                None
                            }
                        },
                    }
                }
            }
        });
        let regex_recognizers: Vec<syn::Expr> = self
            .grammar
            .terminals
            .iter()
            .map(|term| {
                let token_kind = format_ident!("{}", &term.name);
                if term.name == "STOP" {
                   parse_quote! { TokenRecognizer(TokenKind::STOP, Recognizer::Stop) }
                } else {
                    match &term.recognizer {
                        Some(r) => match r {
                            Recognizer::StrConst(s) => {
                                let s = s.as_ref();
                                parse_quote! {
                                    TokenRecognizer(TokenKind::#token_kind, Recognizer::StrMatch(#s))
                                }
                            },
                            Recognizer::RegexTerm(r) => {
                                let r = r.as_ref();
                                parse_quote! {
                                    TokenRecognizer(TokenKind::#token_kind, Recognizer::RegexMatch(Lazy::new(|| {
                                        Regex::new(concat!("^", #r)).unwrap()
                                    })))
                                }
                            },
                        },
                        // This should never happen as we check that all
                        // recognizers are defined when default lexer is used
                        None => panic!("Undefined recognizer for terminal {}", term.name)
                    }
                }
            })
            .collect();

        ast.push(parse_quote!{
            pub(crate) static RECOGNIZERS: [TokenRecognizer; TERMINAL_COUNT]  = [
                #(#regex_recognizers,)*
            ];
        });

        Ok(ast)
    }

    fn generate_builder(&self, types: &SymbolTypes) -> Result<Vec<syn::Stmt>> {
        create_idents!(self, actions_file, root_symbol,);
        let mut ast: Vec<syn::Stmt> = vec![];
        let context_var = format_ident!("context");

        ast.extend::<Vec<syn::Stmt>>(parse_quote! {
            pub struct DefaultBuilder {
                res_stack: Vec<Symbol>,
            }

            impl DefaultBuilder {
                #[allow(dead_code)]
                pub fn new() -> Self {
                    Self {
                        res_stack: vec![]
                    }
                }
            }

            impl Builder for DefaultBuilder
            {
                type Output = #actions_file::#root_symbol;

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
                    TokenKind::#term => Terminal::#term(#actions_file::#action(&*context, token))
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
                        nonterminal.idx.symbol_index(self.grammar.terminals.len()))
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
                                    #match_lhs => NonTerminal::#nonterminal(#actions_file::#action(&*context, #(#params),*)),
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
            impl<'i> LRBuilder<'i, Input,
                 Context<'i, Input>, State, ProdKind, TokenKind> for DefaultBuilder
            {

                #![allow(unused_variables)]
                fn shift_action(
                    &mut self,
                    #context_var: &mut Context<'i, Input>,
                    token: Token<'i, Input, TokenKind>) {
                    let val = match token.kind {
                        TokenKind::STOP => panic!("Cannot shift STOP token!"),
                        #(#shift_match_arms),*
                    };
                    self.res_stack.push(Symbol::Terminal(val));
                }

                fn reduce_action(
                    &mut self,
                    #context_var: &mut Context<'i, Input>,
                    prod: ProdKind,
                    _prod_len: usize) {
                    let prod = match prod {
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
