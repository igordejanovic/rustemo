use convert_case::{Casing, Case};
use proc_macro2::{Span, Ident};
use syn::{parse_quote, parse::Parser};
use quote::quote;

use crate::grammar::{Grammar, NonTerminal, Production};

use super::ActionsGenerator;

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

pub(crate) struct RuleActionsGenerator<'a> {
    grammar: &'a Grammar,
}

impl<'a> ActionsGenerator for RuleActionsGenerator<'a> {
    fn nonterminal_types(&self, nonterminal: &NonTerminal) -> Vec<(String, syn::Item)> {
        // Inspect each production RHS and find the superset of all possible
        // assignments which will be mapped to stuct fields.
        //
        // A special case is when each RHS has only one assignment without LHS
        // defined. In that case construct Enum instead of struct.
        let mut types: Vec<(String, syn::Item)> = vec![];
        let prods = nonterminal.productions(self.grammar);
        let type_name_ident = Ident::new(&nonterminal.name, Span::call_site());

        let ty = if self.grammar.is_enum(nonterminal) {
            // Variants will be named after RHS symbol names
            let variants: Vec<syn::Variant> = prods
                .iter()
                .map(|prod| {
                    let assign = &prod.rhs_assign()[0];
                    let type_name = self.grammar.symbol_name(assign.symbol);
                    if self.grammar.symbol_has_content(assign.symbol) {
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
            let fields = self.nonterminal_type_fields(nonterminal)
                .into_iter()
                .map(|f| f.to_syn_field(nonterminal));

            parse_quote! {
                #[derive(Debug, Clone)]
                pub struct #type_name_ident {
                    #(#fields),*
                }
            }
        };

        types.push((nonterminal.name.clone(), ty));
        types
    }

    fn nonterminal_actions(
        &self,
        nonterminal: &NonTerminal,
    ) -> Vec<(String, syn::Item)> {
        let mut actions: Vec<(String, syn::Item)> = vec![];
        let nt_fields = self.nonterminal_type_fields(nonterminal);
        let type_name = &nonterminal.name;
        let type_name_ident: syn::Type = syn::parse_str(&type_name).unwrap();
        for (idx, prod) in nonterminal.productions(self.grammar).iter().enumerate() {
            let fn_name = format!("{}_p{}", type_name.to_case(Case::Snake), idx);
            let fn_name_ident = Ident::new(&fn_name, Span::call_site());
            let prod_fields = self.production_type_fields(prod);

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

            let body_expr: syn::Expr = if self.grammar.is_enum(nonterminal) {
                // Enum variant value
                let assign = &prod.rhs_assign()[0];
                let symbol = assign.symbol;
                let symbol_ident =
                    Ident::new(&self.grammar.symbol_name(symbol), Span::call_site());
                let arg_type_name =
                    self.grammar.symbol_name(symbol).to_case(Case::Snake);
                let arg_name = assign.name.as_ref().unwrap_or(&arg_type_name);
                let arg_ident = Ident::new(arg_name, Span::call_site());
                let variant: syn::Variant = if self.grammar.is_term(symbol)
                    && !self.grammar.symbol_to_term(symbol).has_content
                {
                    // Variant without content
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
}

impl<'a> RuleActionsGenerator<'a> {

    pub fn new(grammar: &'a Grammar) -> Box<dyn ActionsGenerator + 'a> {
        Box::new(Self { grammar })
    }

    fn nonterminal_type_fields(
        &self,
        nonterminal: &NonTerminal,
    ) -> Vec<NTTypeField> {
        let mut fields: Vec<NTTypeField> = vec![];
        for prod in nonterminal.productions(self.grammar) {
            for prod_field in self.production_type_fields(prod) {
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
        &self,
        prod: &Production,
    ) -> Vec<NTTypeField> {
        let nt_name = &self.grammar.nonterminals[prod.nonterminal].name;
        let mut names = vec![];
        let mut fields = vec![];
        for assign in prod.rhs_with_content(self.grammar) {
            // If assignment name is not given use referenced NonTerminal name.
            let type_name = self.grammar.symbol_name(assign.symbol);
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
}

