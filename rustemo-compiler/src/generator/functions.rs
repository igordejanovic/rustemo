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
    fn parser_header(&self, generator: &ParserGenerator<'g, 's>) -> Result<Vec<syn::Stmt>> {
        let states_count = generator.table.states.len();
        let max_recognizers = generator.table.max_recognizers();
        let term_count = generator.grammar.terminals.len();
        Ok(parse_quote! {
            const STATE_COUNT: usize = #states_count;
            const MAX_RECOGNIZERS: usize = #max_recognizers;
            #[allow(dead_code)]
            const TERMINAL_COUNT: usize = #term_count;
        })
    }

    fn parser_definition(&self, generator: &ParserGenerator<'g, 's>) -> Result<Vec<syn::Stmt>> {
        let parser_definition = &generator.parser_definition;
        let mut ast: Vec<syn::Stmt> = vec![];

        ast.extend::<Vec<_>>(parse_quote! {
            type ActionFn = fn(token: TokenKind) -> Vec<Action<State, ProdKind>>;
            pub struct #parser_definition {
                actions: [ActionFn; STATE_COUNT],
                gotos: [fn(nonterm: NonTermKind) -> State; STATE_COUNT],
                token_kinds: [[Option<(TokenKind, bool)>; MAX_RECOGNIZERS]; STATE_COUNT],
            }
        });

        let action_state_fn_name = |state: &LRState| -> syn::Ident {
            format_ident!(
                "action_{}_s{}",
                generator.grammar.symbol_name(state.symbol).to_lowercase(),
                state.idx.to_string()
            )
        };

        let goto_state_fn_name = |state: &LRState| -> syn::Ident {
            format_ident!(
                "goto_{}_s{}",
                generator.grammar.symbol_name(state.symbol).to_lowercase(),
                state.idx.to_string()
            )
        };

        ast.extend(generator
            .table
            .states
            .iter()
            .map(|state| {
                let mut match_arms: Vec<syn::Arm> = state
                    .actions
                    .iter()
                    .enumerate()
                    .filter(|(_, actions)| !actions.is_empty())
                    .map(|(term_idx, actions)| {
                        let token_kind = generator.term_kind_ident(term_idx.into());
                        let actions: Vec<syn::Expr> =
                            actions.iter()
                                   .map(|action|
                                        generator.action_to_syntax(&Some(action.clone()))).collect();
                        parse_quote! {
                            TK::#token_kind => Vec::from(&[#(#actions),*])
                        }
                    })
                    .collect();
                if match_arms.len() < generator.grammar.terminals.len() {
                    match_arms.push(parse_quote!{
                        _ => vec![]
                    });
                }
                let action_state_fn = action_state_fn_name(state);
                parse_quote! {
                    fn #action_state_fn(token_kind: TokenKind) -> Vec<Action<State, ProdKind>> {
                        match token_kind {
                            #(#match_arms),*
                        }
                    }
                }
            })
            .collect::<Vec<syn::Stmt>>());

        for state in &generator.table.states {
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
                    _ => panic!("Invalid terminal kind ({nonterm_kind:?}) for GOTO state ({:?}).", State::#state_kind)
                    }
                }))
                .collect();

            if match_arms.len() > 1 {
                let goto_state_fn = goto_state_fn_name(state);
                ast.push(parse_quote! {
                    fn #goto_state_fn(nonterm_kind: NonTermKind) -> State {
                        match nonterm_kind {
                            #(#match_arms),*
                        }
                    }
                });
            }
        }

        let max_recognizers = generator.table.max_recognizers();
        let token_kinds: Vec<syn::Expr> = generator
            .table
            .states
            .iter()
            .map(|state| {
                let terminals: Vec<syn::Expr> = state
                    .sorted_terminals
                    .iter()
                    .map(|(term_index, finish)| {
                        let term: &Terminal = &generator.grammar.terminals[*term_index];
                        let token_kind = format_ident!("{}", &term.name);
                        let finish = format_ident!("{finish}");
                        parse_quote! {
                            Some((TK::#token_kind, #finish))
                        }
                    })
                    .chain(
                        // Fill the rest with "None"
                        repeat(parse_quote! {None})
                            .take(max_recognizers - state.sorted_terminals.len()),
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
            .map(action_state_fn_name)
            .collect();
        let goto_fn_names: Vec<syn::Ident> = generator
            .table
            .states
            .iter()
            .map(|state| {
                if state.gotos.iter().any(|&state_idx| state_idx.is_some()) {
                    // We have goto transitions for this state
                    goto_state_fn_name(state)
                } else {
                    format_ident!("goto_invalid")
                }
            })
            .collect();

        ast.push(parse_quote! {
            fn goto_invalid(_nonterm_kind: NonTermKind) -> State {
                panic!("Invalid GOTO entry!");
            }
        });

        ast.push(parse_quote! {
            pub(in crate) static PARSER_DEFINITION: #parser_definition = #parser_definition {
                actions: [#(#actions_fn_names),*],
                gotos: [#(#goto_fn_names),*],
                token_kinds: [#(#token_kinds),*],
            };
        });

        let longest_match = format_ident!("{}", generator.settings.lexical_disamb_longest_match);
        let grammar_order = format_ident!("{}", generator.settings.lexical_disamb_grammar_order);
        ast.push(parse_quote! {
            impl ParserDefinition<State, ProdKind, TokenKind, NonTermKind> for #parser_definition {
                fn actions(&self, state: State, token: TokenKind) -> Vec<Action<State, ProdKind>> {
                    PARSER_DEFINITION.actions[state as usize](token)
                }
                fn goto(&self, state: State, nonterm: NonTermKind) -> State {
                    PARSER_DEFINITION.gotos[state as usize](nonterm)
                }
                fn expected_token_kinds(&self, state: State) -> Vec<(TokenKind, bool)> {
                    PARSER_DEFINITION.token_kinds[state as usize].iter().map_while(|t| *t).collect()
                }
                fn longest_match() -> bool {
                    #longest_match
                }
                fn grammar_order() -> bool {
                    #grammar_order
                }
            }
        });

        Ok(ast)
    }

    fn delegate(&self) -> &dyn PartGenerator<'g, 's> {
        &self.delegate
    }
}
