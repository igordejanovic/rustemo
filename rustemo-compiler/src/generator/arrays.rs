use std::iter::repeat;

use quote::format_ident;
use syn::parse_quote;

use crate::{error::Result, grammar::Terminal};

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
    fn parser_header(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        let max_actions = generator.table.max_actions();
        let max_recognizers = generator.table.max_recognizers();
        let nonterm_count = generator.grammar.nonterminals.len();
        let states_count = generator.table.states.len();
        Ok(parse_quote! {
            const NONTERMINAL_COUNT: usize = #nonterm_count;
            const STATE_COUNT: usize = #states_count;
            #[allow(dead_code)]
            const MAX_ACTIONS: usize = #max_actions;
            const MAX_RECOGNIZERS: usize = #max_recognizers;
        })
    }

    fn parser_definition(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
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
                    .map(|term_index| {
                        let term: &Terminal =
                            generator.grammar.term_by_index(*term_index);
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

        Ok(ast)
    }

    fn delegate(&self) -> &dyn PartGenerator<'g, 's> {
        &self.delegate
    }
}
