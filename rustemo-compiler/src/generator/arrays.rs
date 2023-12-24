use std::iter::repeat;

use quote::format_ident;
use syn::parse_quote;

use crate::{
    error::Result, grammar::Terminal, BuilderType, LexerType, ParserAlgo,
};

use super::{base::BasePartGenerator, ParserGenerator, PartGenerator};

pub(crate) struct ArrayPartGenerator {
    delegate: BasePartGenerator,
}

impl ArrayPartGenerator {
    pub fn new() -> Self {
        ArrayPartGenerator {
            delegate: BasePartGenerator::new(),
        }
    }
}

impl<'g, 's> PartGenerator<'g, 's> for ArrayPartGenerator {
    fn header(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        self.delegate.header(generator)
    }

    fn parser_header(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        let max_actions = generator.table.max_actions();
        let max_recognizers = generator.table.max_recognizers();
        let term_count = generator.grammar.terminals.len();
        let nonterm_count = generator.grammar.nonterminals.len();
        let states_count = generator.table.states.len();
        Ok(parse_quote! {
            const TERMINAL_COUNT: usize = #term_count;
            const NONTERMINAL_COUNT: usize = #nonterm_count;
            const STATE_COUNT: usize = #states_count;
            #[allow(dead_code)]
            const MAX_ACTIONS: usize = #max_actions;
            const MAX_RECOGNIZERS: usize = #max_recognizers;
        })
    }

    fn symbols(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        self.delegate.symbols(generator)
    }

    fn types(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        self.delegate.types(generator)
    }

    fn lexer_definition(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        self.delegate.lexer_definition(generator)
    }

    fn parser_definition(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        let parser = &generator.parser;
        let parser_definition = &generator.parser_definition;
        let mut ast: Vec<syn::Stmt> = vec![];

        ast.push(parse_quote! {
            pub struct #parser_definition {
                actions: [[[Action<State, ProdKind>; MAX_ACTIONS]; TERMINAL_COUNT]; STATE_COUNT],
                gotos: [[Option<State>; NONTERMINAL_COUNT]; STATE_COUNT],
                token_kinds: [[Option<TokenKind>; MAX_RECOGNIZERS]; STATE_COUNT],
            }
        });

        let max_actions = generator.table.max_actions();
        let actions: Vec<syn::Expr> = generator
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
                            .map(|a| generator.action_to_syntax(&a))
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

        let gotos: Vec<syn::Expr> = generator
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
                                generator.state_kind_ident(*state);
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

        let max_recognizers = generator.table.max_recognizers();
        let token_kinds: Vec<syn::Expr> = generator
            .table
            .states
            .iter()
            .map(|state| {
                let terminals: Vec<syn::Expr> = state
                    .sorted_terminals
                    .iter()
                    .map(|x| {
                        let term: &Terminal = &generator.grammar.terminals[*x];
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

    fn builder(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        self.delegate.builder(generator)
    }
}
