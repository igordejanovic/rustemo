//! Parser/generator for actions file
//!
//! Provides default semantics actions implementation but allow for manual
//! changes.

use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span};
use syn::{self, parse::Parser, parse_quote};

use crate::error::{Error, Result};
use crate::grammar::{res_symbol, Grammar, NonTerminal};

pub(crate) fn generate_parser_actions<F>(
    grammar: &Grammar,
    grammar_path: F,
) -> Result<()>
where
    F: AsRef<Path> + core::fmt::Debug,
{
    let mut file_name = grammar_path
        .as_ref()
        .file_stem()
        .ok_or(Error::Error("Invalid file name.".into()))?
        .to_os_string();
    file_name.push("_actions.rs");
    let action_file =
        PathBuf::from(grammar_path.as_ref()).with_file_name(file_name);

    let mut ast = if action_file.exists() {
        log!("Parsing action file with Syn: {:?}", action_file);
        syn::parse_file(&std::fs::read_to_string(&action_file)?)?
    } else {
        // Create new empty file with common uses.
        log!("Action file not found. Creating: {:?}", action_file);
        parse_quote! {
            use std::collections::BTreeMap;
            use rustemo_rt::lexer::Token;
        }
    };

    // Collect function and type names
    let mut type_names = BTreeSet::new();
    let mut action_names = BTreeSet::new();
    for item in &ast.items {
        match item {
            // Used for grammar rules of the form:
            // NT: First | Second | Third;
            // TODO: Are non-terminals allowed in the RHS?
            syn::Item::Enum(e) => type_names.insert(e.ident.to_string()),
            // Used for actions
            syn::Item::Fn(f) => action_names.insert(f.sig.ident.to_string()),
            // Used for types produced for rules LHS
            syn::Item::Type(t) => type_names.insert(t.ident.to_string()),
            _ => false,
        };
    }

    // Generate types and actions for terminals
    for terminal in grammar.terminals() {
        // Add terminal types
        let type_name = &terminal.name;
        let type_name_ident = Ident::new(type_name, Span::call_site());
        if !type_names.contains(type_name) {
            log!("Create type for terminal '{type_name}'.");
            ast.items.push(parse_quote! {
                pub type #type_name_ident = String;
            });
        }
        // Add terminal actions
        let action_name = terminal.name.to_case(Case::Snake);
        let action_name_ident = Ident::new(&action_name, Span::call_site());
        if !action_names.contains(&action_name) {
            log!("Create action function for terminal '{type_name}'.");
            ast.items.push(parse_quote! {
                pub fn #action_name_ident<'a>(token: Token<&'a str>) -> #type_name_ident {
                    token.value
                }
            })
        }
    }

    // Generate types and actions for non-terminals
    for nonterminal in grammar.nonterminals() {
        // Add nonterminal type
        let type_name = &nonterminal.name;
        if !type_names.contains(type_name) {
            ast.items.push(nonterminal_type(grammar, nonterminal));
        }

        // Add nonterminal actions
        // let action_name = nonterminal.name.to_case(Case::Snake);
        // if !action_names.contains(&action_name) {
        //     ast.items.push(parse_quote! {})
        // }
    }

    log!("Writing action file {:?}", action_file);
    std::fs::write(action_file, prettyplease::unparse(&ast))?;

    Ok(())
}

struct Field {
    inner: syn::Field,
    count: usize,
}

/// Create Rust type for the given non-terminal.
fn nonterminal_type(grammar: &Grammar, nonterminal: &NonTerminal) -> syn::Item {
    // Inspect each production RHS and find the superset of all possible
    // assignments which will be mapped to stuct fields.
    //
    // A special case is when each RHS has only one assignment without LHS
    // defined. In that case construct Enum instead of struct.
    let prods = nonterminal.productions(grammar);
    let type_name_ident = Ident::new(&nonterminal.name, Span::call_site());

    return if grammar.is_enum(nonterminal) {
        // Variants will be named after RHS symbol names
        let variants = grammar
            .symbol_names(prods.iter().map(|p| res_symbol(&p.rhs[0])))
            .into_iter()
            .map(|s| quote::format_ident!("{}", &s));
        parse_quote! {
            pub enum #type_name_ident {
                #(#variants(#variants)),*
            }
        }
    } else {
        let mut f: BTreeMap<String, Field> = BTreeMap::new();
        for prod in nonterminal.productions(grammar) {
            for assig in &prod.rhs {
                let symbol = res_symbol(assig);
                let name = grammar.nt_field_name(assig, symbol);
                let name_ident = Ident::new(
                    &grammar.nt_field_name(assig, symbol),
                    Span::call_site(),
                );
                let field_type: syn::Type =
                    syn::parse_str(&grammar.symbol_name(symbol)).unwrap();
                f.entry(name.clone())
                    .and_modify(|e| e.count += 1)
                    .or_insert(Field {
                        inner: syn::Field::parse_named
                            .parse2(
                                quote::quote! { pub #name_ident: #field_type },
                            )
                            .unwrap(),
                        count: 0,
                    });
            }
        }

        // Those fields that don't show up in each productions are optional.
        f.values_mut().for_each(|v| {
            if v.count < nonterminal.productions.len() {
                let base_type = &v.inner.ty;
                v.inner.ty =
                    syn::parse2(quote::quote! { Option<#base_type> }).unwrap();
            }
        });

        let fields = f.into_iter().map(|(_, f)| f.inner);

        parse_quote! {
            pub struct #type_name_ident {
                #(#fields),*
            }
        }
    };
}
