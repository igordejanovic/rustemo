use std::iter::{once, repeat};

use super::{arrays::ArrayPartGenerator, ParserGenerator, PartGenerator};

use crate::{error::Result, grammar::Terminal, table::LRState};
use quote::format_ident;
use syn::parse_quote;

pub(crate) struct FunctionPartGenerator {
    delegate: ArrayPartGenerator,
}

impl FunctionPartGenerator {
    pub fn new() -> Self {
        FunctionPartGenerator {
            delegate: ArrayPartGenerator::new(),
        }
    }
}

impl<'g, 's> PartGenerator<'g, 's> for FunctionPartGenerator {
    fn parser_header(
        &self,
        generator: &ParserGenerator<'g, 's>,
    ) -> Result<Vec<syn::Stmt>> {
        let states_count = generator.table.states.len();
        Ok(parse_quote! {
            const STATE_COUNT: usize = #states_count;
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
                //actions: [[[Action<State, ProdKind>; MAX_ACTIONS]; TERMINAL_COUNT]; STATE_COUNT],
                actions: [fn(token: TokenKind) -> impl Iterator<Item=Action<State, ProdKind>>; STATE_COUNT],
                //gotos: [[Option<State>; NONTERMINAL_COUNT]; STATE_COUNT],
                gotos: [fn(nonterm: NonTermKind) -> State; STATE_COUNT],
                token_kinds: [[Option<TokenKind>; MAX_RECOGNIZERS]; STATE_COUNT],
            }
        });

        let action_state_fn_name = |state: &LRState| -> syn::Ident {
            format_ident!("action_{}s{}",
                          generator.grammar.symbol_name(state.symbol).to_lowercase(),
                          state.idx.to_string())
        };

        let goto_state_fn_name = |state: &LRState| -> syn::Ident {
            format_ident!("goto_{}s{}",
                          generator.grammar.symbol_name(state.symbol).to_lowercase(),
                          state.idx.to_string())
        };

        ast.extend(generator
            .table
            .states
            .iter()
            .map(|state| {
                let match_arms: Vec<syn::Arm> = state
                    .actions
                    .iter()
                    .enumerate()
                    .filter(|(_, actions)| !actions.is_empty())
                    .map(|(term_idx, actions)| {
                        let token_kind = generator.term_kind_ident(term_idx.into());
                        let actions: Vec<syn::Expr> = actions.iter()
                                                             .map(|action|
                                                                         generator.action_to_syntax(&Some(action.clone()))).collect();
                        parse_quote! {
                            TokenKind::#token_kind => [#(#actions),*].iter()
                        }
                    })
                    .collect();
                let action_state_fn = action_state_fn_name(state);
                parse_quote! {
                    fn #action_state_fn(token_kind: TokenKind)
                                        -> impl Iterator<Item=Action<State, ProdKind>> {
                        match token_kind {
                            #(#match_arms),*,
                            _ => Action::Error
                        }
                    }
                }
            })
            .collect::<Vec<syn::Stmt>>());

        ast.extend(generator
            .table
            .states
            .iter()
            .map(|state| {
                let match_arms: Vec<syn::Arm> = state
                    .gotos
                    .iter()
                    .enumerate()
                    .filter(|(_, &state_idx)|{state_idx.is_some()})
                    .map(|(nonterm_idx, &state_index)| {
                        let nonterm_kind = generator.nonterm_kind_ident(nonterm_idx.into());
                        let state_kind = generator.state_kind_ident(state_index.unwrap());
                        parse_quote!{
                            NonTermKind::#nonterm_kind => State::#state_kind
                        }
                    }).chain(once({
                        let state_kind = generator.state_kind_ident(state.idx);
                        parse_quote!{
                        _ => panic!("Invalid terminal kind ({nonterm_kind}) for GOTO state ({}).", State::#state_kind)
                        }
                    }))
                    .collect();
                let goto_state_fn = goto_state_fn_name(state);
                parse_quote! {
                    fn #goto_state_fn(nonterm_kind: NonTermKind) -> State {
                        match nonterm_kind {
                            #(#match_arms),*
                        }
                    }
                }
            })
            .collect::<Vec<_>>());

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

        let actions_fn_names: Vec<syn::Ident> = generator
            .table
            .states
            .iter()
            .map(|state| action_state_fn_name(state))
            .collect();
        let goto_fn_names: Vec<syn::Ident> = generator
            .table
            .states
            .iter()
            .map(|state| goto_state_fn_name(state))
            .collect();

        ast.push(parse_quote! {
            pub(in crate) static PARSER_DEFINITION: #parser_definition = #parser_definition {
                actions: [#(#actions_fn_names),*],
                gotos: [#(#goto_fn_names),*],
                token_kinds: [#(#token_kinds),*],
            };
        });

        ast.push(parse_quote! {
            impl ParserDefinition<State, ProdKind, TokenKind, NonTermKind> for #parser_definition {
                fn actions(&self, state: State, token: TokenKind) -> &'static [Action<State, ProdKind>] {
                    &PARSER_DEFINITION.actions[state as usize](token)
                }
                fn goto(&self, state: State, nonterm: NonTermKind) -> State {
                    PARSER_DEFINITION.gotos[state as usize](nonterm)
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
