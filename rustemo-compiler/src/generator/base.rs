use std::iter::repeat;

use quote::format_ident;
use syn::parse_quote;

use crate::{
    error::Result, grammar::types::to_snake_case,
    lang::rustemo_actions::Recognizer, BuilderType, LexerType, ParserAlgo,
};

use super::{
    action_name, actions::generate_parser_actions, ParserGenerator,
    PartGenerator,
};

pub(crate) struct BasePartGenerator {}

impl BasePartGenerator {
    pub fn new() -> Self {
        BasePartGenerator {}
    }
}

impl<'g, 's> PartGenerator<'g, 's> for BasePartGenerator {
    fn header(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        let actions_file = &generator.actions_file;
        let input_type = &generator.input_type;

        let mut imports: Vec<syn::Stmt> = vec![];

        if let LexerType::Default = generator.settings.lexer_type {
            let regex: syn::Stmt = if generator.settings.fancy_regex {
                parse_quote! {
                    use fancy_regex::Regex;
                }
            } else {
                parse_quote! {
                    use regex::Regex;
                }
            };
            imports.extend::<Vec<syn::Stmt>>(parse_quote! {
                #regex
                use once_cell::sync::Lazy;
                use rustemo::StringLexer;
            });
        }

        imports.push(parse_quote! {
            use rustemo::LRBuilder;
        });
        imports.extend::<Vec<syn::Stmt>>(
            match generator.settings.builder_type {
                BuilderType::Default => parse_quote! {
                    use super::#actions_file;
                },
                BuilderType::Generic => parse_quote! {
                    use rustemo::{TreeNode, TreeBuilder};
                },
                BuilderType::Custom => parse_quote! {
                    use std::cell::RefCell;
                },
            },
        );

        imports.extend::<Vec<syn::Stmt>>(
            match generator.settings.parser_algo {
                ParserAlgo::LR => parse_quote! {
                    use rustemo::{LRParser, LRContext};
                },
                ParserAlgo::GLR => parse_quote! {
                    use rustemo::{GlrParser, Forest, GssHead};
                },
            },
        );

        let header: Vec<syn::Stmt> = parse_quote! {
            /// Generated by rustemo. Do not edit manually!
            use std::fmt::Debug;
            use std::hash::Hash;

            use rustemo::{Result, Input as InputT, Lexer, Token,
                          TokenRecognizer as TokenRecognizerT,
                          Parser, ParserDefinition, State as StateT, Builder};
            #(#imports)*
            use rustemo::Action::{self, Shift, Reduce, Accept};
            #[allow(unused_imports)]
            use rustemo::debug::{log, logn};
            #[allow(unused_imports)]
            #[cfg(debug_assertions)]
            use colored::*;

            pub type Input = #input_type;
        };

        Ok(header)
    }

    fn symbols(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        let mut ast: Vec<syn::Stmt> = vec![];
        if !matches!(generator.settings.builder_type, BuilderType::Default) {
            return Ok(ast);
        }
        let actions_file = &generator.actions_file;

        ast.push(parse_quote! {
            #[derive(Debug)]
            pub enum Symbol {
                Terminal(Terminal),
                NonTerminal(NonTerminal)
            }
        });

        let term_variants: Vec<syn::Variant> = generator.grammar.terminals[1..]
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

        let nonterm_variants: Vec<syn::Variant> = generator
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

    fn types(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        let mut ast: Vec<syn::Stmt> = vec![];

        let token_kind_variants: Vec<syn::Variant> = generator
            .grammar
            .terminals
            .iter()
            .map(|t| {
                let name = format_ident!("{}", t.name);
                parse_quote! { #name }
            })
            .collect();

        ast.extend::<Vec<_>>(parse_quote! {
            #[allow(clippy::upper_case_acronyms)]
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub enum TokenKind {
                #[default]
                #(#token_kind_variants),*
            }
            use TokenKind as TK;
        });

        ast.push(parse_quote! {
            impl From<TokenKind> for usize {
                fn from(t: TokenKind) -> Self {
                    t as usize
                }
            }
        });

        let prodkind_variants: Vec<syn::Variant> = generator
            .grammar
            .productions()
            .iter()
            .map(|prod| {
                let prod_kind = generator.prod_kind_ident(prod);
                parse_quote! {#prod_kind}
            })
            .collect();
        ast.extend::<Vec<_>>(parse_quote! {
            #[allow(clippy::enum_variant_names)]
            #[derive(Clone, Copy, PartialEq)]
            pub enum ProdKind {
                #(#prodkind_variants),*
            }
            use ProdKind as PK;
        });

        let display_arms: Vec<syn::Arm> = generator
            .grammar
            .productions()
            .iter()
            .map(|&prod| {
                let prod_kind_ident = generator.prod_kind_ident(prod);
                let prod_str = prod.to_string(generator.grammar);
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

        let nonterm_kind_variants: Vec<syn::Variant> = generator
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

        let from_arms: Vec<syn::Arm> = generator
            .grammar
            .productions()
            .iter()
            .map(|&prod| {
                let prod_kind = generator.prod_kind_ident(prod);
                let nt_kind = format_ident!(
                    "{}",
                    prod.nonterminal(generator.grammar).name
                );
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

        let state_variants: Vec<syn::Variant> = generator
            .table
            .states
            .iter()
            .map(|state| {
                let state_kind = generator.state_kind_ident(state.idx);
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

        let layout_state = generator
            .table
            .layout_state
            .map(|s| generator.state_kind_ident(s));
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

        let state_display_arms: Vec<syn::Arm> = generator
            .table
            .states
            .iter()
            .map(|state| {
                let state_kind_ident = generator.state_kind_ident(state.idx);
                let state_index_str = format!(
                    "{}:{}",
                    state.idx,
                    generator.grammar.symbol_name(state.symbol)
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

    fn parser(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        let parser = &generator.parser;
        let parser_definition = &generator.parser_definition;
        let mut ast: Vec<syn::Stmt> = vec![];
        // Context type
        ast.push(match generator.settings.parser_algo {
            ParserAlgo::LR => parse_quote!{
                pub(crate) type Context<'i, I> = LRContext<'i, I, State, TokenKind>;
            },
            ParserAlgo::GLR => parse_quote!{
                pub(crate) type Context<'i, I> = GssHead<'i, I, State, TokenKind>;
            },
        });

        let partial_parse: syn::Expr = if generator.settings.partial_parse {
            parse_quote! { true }
        } else {
            parse_quote! { false }
        };

        let skip_ws =
            generator.settings.skip_ws && !generator.grammar.has_layout();

        let lexer_instance: syn::Expr = match generator.settings.lexer_type {
            LexerType::Default => parse_quote! {
                StringLexer::new(#skip_ws, &RECOGNIZERS)
            },
            LexerType::Custom => parse_quote! {
                lexer
            },
        };

        let builder_instance: syn::Expr = match generator.settings.builder_type
        {
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

        let has_layout = generator.grammar.has_layout();
        let parser_instance: syn::Expr = match generator.settings.parser_algo {
            ParserAlgo::LR => parse_quote! {
                LRParser::new(&PARSER_DEFINITION, State::default(), #partial_parse, #has_layout,
                              #lexer_instance, #builder_instance)
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
        match generator.settings.lexer_type {
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
        match generator.settings.builder_type {
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
            if let ParserAlgo::LR = generator.settings.parser_algo {
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
            if let ParserAlgo::GLR = generator.settings.parser_algo {
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
            impl<'i, I, L, B> Parser<'i, I, Context<'i, I>, State, TokenKind> for #parser <'i, I, L, B>
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

    fn lexer_definition(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        let mut ast: Vec<syn::Stmt> = vec![];
        if !matches!(generator.settings.lexer_type, LexerType::Default) {
            return Ok(ast);
        }

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

        let regex: syn::Expr = if generator.settings.fancy_regex {
            parse_quote! {
                Ok(Some(x))
            }
        } else {
            parse_quote! {
                Some(x)
            }
        };

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
                                #regex => {
                                    let x_str = x.as_str();
                                    log!("{} '{}'", "recognized".bold().green(), x_str);
                                    Some(x_str)
                                },
                                _ => {
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
        let regex_recognizers: Vec<syn::Expr> = generator
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

    fn builder(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        let mut ast: Vec<syn::Stmt> = vec![];
        if !matches!(generator.settings.builder_type, BuilderType::Default) {
            return Ok(ast);
        }

        // Generate actions
        if generator.settings.actions {
            generate_parser_actions(
                generator, // generator.grammar,
                          // generator.types.as_ref().unwrap(),
                          // &generator.file_name,
                          // out_dir_actions,
                          // generator.settings,
            )?;
        }

        let actions_file = &generator.actions_file;
        let root_symbol = &generator.root_symbol;
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
            generator.grammar.terminals[1..].iter().filter(|t| t.reachable.get())
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

        if generator.grammar.terminals[1..]
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
            generator.grammar.productions().iter()
                                      .filter_map(|production| {
                let nonterminal = &generator.grammar.nonterminals[production.nonterminal];
                if !nonterminal.reachable.get() {
                    has_nonreachable_nonterminals = true;
                    return None
                }
                let rhs_len = production.rhs.len();
                let choice = &generator.types.as_ref().unwrap().get_type(
                        nonterminal.idx.symbol_index(generator.grammar.terminals.len()))
                                       .choices[production.ntidx];
                let action = format_ident!("{}", action_name(nonterminal, choice));

                let prod_kind = generator.prod_kind_ident(production);
                let nonterminal = format_ident!("{}", nonterminal.name);

                if rhs_len == 0 {
                    // Handle EMPTY reduction
                    Some(parse_quote!{
                        ProdKind::#prod_kind => NonTerminal::#nonterminal(#actions_file::#action(#context_var))
                    })
                } else {
                    // Special handling of production with only str match terms in RHS
                    if production.rhs_with_content(generator.grammar).is_empty() {
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
                            if generator.grammar.symbol_has_content(symbol) {
                                param_count += 1;
                                if generator.grammar.is_term(symbol){
                                    let terminal = format_ident!("{}", generator.grammar.symbol_to_term(symbol).name);
                                    parse_quote!{ Symbol::Terminal(Terminal::#terminal(#param)) }
                                } else {
                                    let nonterminal = format_ident!("{}", generator.grammar.symbol_to_nonterm(symbol).name);
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

                        let params: Vec<syn::Ident> = (0..production.rhs_with_content(generator.grammar).len())
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

    fn delegate(&self) -> &dyn PartGenerator<'g, 's> {
        unimplemented!("Delegate not defined!")
    }
}
