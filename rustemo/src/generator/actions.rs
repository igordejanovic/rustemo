//! Parser/generator for actions file
//!
//! Provides default semantics actions implementation but allow for manual
//! changes.

use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};

use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{self, parse::Parser, parse_quote};

use crate::error::{Error, Result};
use crate::grammar::{Grammar, NonTerminal, Production};

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
        // Create new empty file with common uses statements.
        log!("Action file not found. Creating: {:?}", action_file);
        parse_quote! {
            ///! This file is maintained by rustemo but can be modified manually.
            ///! All manual changes will be preserved except non-doc comments.
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
            syn::Item::Enum(e) => {
                let type_name = e.ident.to_string();
                log!("Found enum type '{}'", type_name);
                type_names.insert(e.ident.to_string())
            }
            syn::Item::Struct(e) => {
                let type_name = e.ident.to_string();
                log!("Found struct type '{}'", type_name);
                type_names.insert(e.ident.to_string())
            }
            // Used for actions
            syn::Item::Fn(f) => {
                let type_name = f.sig.ident.to_string();
                log!("Found action function '{}'", type_name);
                action_names.insert(type_name)
            }
            syn::Item::Type(t) => {
                let type_name = t.ident.to_string();
                log!("Found type '{}'", type_name);
                type_names.insert(type_name)
            }
            _ => false,
        };
    }

    // Generate types and actions for terminals
    for terminal in grammar.terminals().iter().filter(|t| t.has_content) {
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
                    token.value.into()
                }
            })
        }
    }

    // Generate types and actions for non-terminals
    for nonterminal in grammar.nonterminals().iter().filter(|&nt| {
        let nt_symbol = grammar.nonterm_to_symbol_index(nt.idx);
        nt_symbol != grammar.augmented_index && nt_symbol != grammar.empty_index
    }) {
        // Add non-terminal type
        let type_name = &nonterminal.name;
        if !type_names.contains(type_name) {
            log!("Creating type for non-terminal '{type_name}'.");
            ast.items.push(nonterminal_type(grammar, nonterminal));
        }

        // Add non-terminal actions
        for (action_name, action) in nonterminal_actions(grammar, nonterminal) {
            if !action_names.contains(&action_name) {
                log!("Creating action '{action_name}'.");
                ast.items.push(action);
            }
        }
    }

    log!("Writing action file {:?}", action_file);
    std::fs::write(action_file, prettyplease::unparse(&ast))?;

    Ok(())
}

/// Represents a field in a struct type constructed for a NonTerminal
struct NTTypeField {
    name: String,
    type_: String,
    boxed: bool,
    optional: bool,
    count: usize,
}

impl NTTypeField {
    fn to_syn_field(&self, nonterminal: &NonTerminal) -> syn::Field {
        let name_ident = Ident::new(&self.name, Span::call_site());
        let field_type: syn::Type = syn::parse_str(&self.type_).unwrap();
        let mut field = syn::Field::parse_named
            .parse2(if self.boxed {
                // Handle direct recursion
                quote! { pub #name_ident: Box<#field_type> }
            } else {
                quote! { pub #name_ident: #field_type }
            })
            .unwrap();

        // If reference is not available in each production the value is
        // optional
        if self.count < nonterminal.productions.len() {
            let base_type = field.ty;
            field.ty = syn::parse2(quote! { Option<#base_type> }).unwrap();
        }
        field
    }
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
        let variants: Vec<syn::Variant> = prods
            .iter()
            .map(|prod| {
                let assign = &prod.rhs_assign()[0];
                let type_name = grammar.symbol_name(assign.symbol);
                if grammar.symbol_has_content(assign.symbol) {
                    syn::parse_str(&format!("{type_name}({type_name})"))
                        .unwrap()
                } else {
                    syn::parse_str(&type_name).unwrap()
                }
            })
            .collect::<Vec<_>>();

        parse_quote! {
            #[derive(Debug, Clone)]
            pub enum #type_name_ident {
                #(#variants),*
            }
        }
    } else {
        let fields = nonterminal_type_fields(&grammar, nonterminal)
            .into_iter()
            .map(|f| f.to_syn_field(nonterminal));

        parse_quote! {
            #[derive(Debug, Clone)]
            pub struct #type_name_ident {
                #(#fields),*
            }
        }
    };
}

fn nonterminal_type_fields(
    grammar: &Grammar,
    nonterminal: &NonTerminal,
) -> Vec<NTTypeField> {
    let mut fields: Vec<NTTypeField> = vec![];
    for prod in nonterminal.productions(grammar) {
        for prod_field in production_type_fields(grammar, prod) {
            match fields.iter_mut().find(|f| f.name == prod_field.name) {
                Some(f) => f.count += 1,
                None => fields.push(prod_field),
            }
        }
    }

    // Those fields that don't show up in each productions are optional.
    fields
        .iter_mut()
        .for_each(|v| v.optional = v.count < nonterminal.productions.len());

    fields
}

fn production_type_fields(
    grammar: &Grammar,
    prod: &Production,
) -> Vec<NTTypeField> {
    let nt_name = &grammar.nonterminals()[prod.nonterminal].name;
    let mut names = vec![];
    let mut fields = vec![];
    for assign in prod.rhs_with_content(grammar) {
        // If assignment name is not given use referenced NonTerminal name.
        let type_name = grammar.symbol_name(assign.symbol);
        let mut name = assign.name.unwrap_or(type_name.to_case(Case::Snake));
        let name_count = names.iter().filter(|&n| *n == name).count();
        if name_count > 0 {
            name = format!("{}{}", name, name_count);
        }
        names.push(name.clone());
        fields.push(NTTypeField {
            name,
            count: 0,
            type_: type_name.clone(),
            boxed: type_name == *nt_name,
            optional: false,
        });
    }
    fields
}

/// Creates an action function for each production of the given non-terminal.
fn nonterminal_actions(
    grammar: &Grammar,
    nonterminal: &NonTerminal,
) -> Vec<(String, syn::Item)> {
    let mut actions: Vec<(String, syn::Item)> = vec![];
    let nt_fields = nonterminal_type_fields(grammar, nonterminal);
    let type_name = &nonterminal.name;
    let type_name_ident: syn::Type = syn::parse_str(&type_name).unwrap();
    for (idx, prod) in nonterminal.productions(grammar).iter().enumerate() {
        let fn_name = format!("{}_p{}", type_name.to_case(Case::Snake), idx);
        let fn_name_ident = Ident::new(&fn_name, Span::call_site());
        let prod_fields = production_type_fields(grammar, prod);

        let args: Vec<syn::FnArg> = prod_fields
            .iter()
            .map(|f| {
                let type_name: syn::Type = syn::parse_str(&f.type_).unwrap();
                let name = Ident::new(&f.name, Span::call_site());
                let arg: syn::FnArg =
                    syn::parse2(quote! { #name: #type_name }).unwrap();
                arg
            })
            .collect();

        let body_expr: syn::Expr = if grammar.is_enum(nonterminal) {
            // Enum variant value
            let assign = &prod.rhs_assign()[0];
            let symbol = assign.symbol;
            let symbol_ident =
                Ident::new(&grammar.symbol_name(symbol), Span::call_site());
            let arg_type_name =
                grammar.symbol_name(symbol).to_case(Case::Snake);
            let arg_name = assign.name.as_ref().unwrap_or(&arg_type_name);
            let arg_ident = Ident::new(arg_name, Span::call_site());
            let variant: syn::Variant = if grammar.is_term(symbol)
                && !grammar.symbol_to_term(symbol).has_content
            {
                // Variant witout content
                syn::parse2(quote! { #symbol_ident }).unwrap()
            } else {
                // Variant with content
                syn::parse2(quote! { #symbol_ident(#arg_ident) }).unwrap()
            };

            parse_quote! {
                #type_name_ident::#variant
            }
        } else {
            // Struct value
            let field_values = nt_fields
                .iter()
                .map(|f| {
                    let in_prod =
                        prod_fields.iter().find(|x| x.name == f.name).is_some();
                    let ident = Ident::new(&f.name, Span::call_site());
                    let mut value: syn::Expr = syn::parse2(if in_prod {
                        quote! { #ident }
                    } else {
                        quote! { None }
                    })
                    .unwrap();
                    if in_prod && f.boxed {
                        value =
                            syn::parse2(quote! { Box::new(#value) }).unwrap();
                    }
                    if in_prod && f.optional {
                        value = syn::parse2(quote! { Some(#value) }).unwrap();
                    }
                    syn::parse2(if in_prod && !f.boxed && !f.optional {
                        quote! { #value }
                    } else {
                        quote! { #ident: #value }
                    })
                    .unwrap()
                })
                .collect::<Vec<syn::FieldValue>>();
            parse_quote! {
                #type_name_ident {
                    #(#field_values),*
                }
            }
        };

        let fn_item: syn::Item = parse_quote! {
            pub fn #fn_name_ident(#(#args),*) -> #type_name_ident {
                #body_expr
            }
        };
        actions.push((fn_name, fn_item));
    }
    actions
}
