use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse::Parser, parse_quote};

use crate::grammar::{Grammar, NonTerminal, Production};

use super::ActionsGenerator;

pub(crate) struct ProductionActionsGenerator<'a> {
    grammar: &'a Grammar,
}

struct ProdField {
    name: String,
    ty: String,
    boxed: bool,
}

impl<'a> ProductionActionsGenerator<'a> {
    pub fn new(grammar: &'a Grammar) -> Box<dyn ActionsGenerator + 'a> {
        Box::new(Self { grammar })
    }

    fn prod_fields(&self, prod: &Production) -> Vec<ProdField> {
        let nonterminal = &self.grammar.nonterminals[prod.nonterminal];
        let variant_name = format!(
                "{}{}",
                nonterminal.name,
                prod.kind
                    .as_ref()
                    .unwrap_or(&format!("{}", prod.ntidx + 1))
            );
        let rhs = prod.rhs_with_content(self.grammar);
        match rhs.iter().count() {
            0 => vec![],
            1 => {
                let field_type_name =  self.grammar.symbol_name(rhs[0].symbol);
                vec![ProdField{ name: variant_name.to_case(Case::Snake),
                                ty: field_type_name.clone(),
                                boxed: field_type_name == nonterminal.name }]
            },
            _ => // More than one ref with content. Make a new type.
                prod.rhs_assign()
                    .into_iter()
                    .enumerate()
                    .filter( |(_, assign)|
                                    self.grammar.symbol_has_content(assign.symbol))
                    .map(|(idx, assign)| {
                        let field_type_name = self.grammar.symbol_name(assign.symbol);
                        let name = assign
                            .name
                            .unwrap_or(
                                format!("{}_{}",
                                        field_type_name.to_case(Case::Snake),
                                        idx + 1
                                )
                            );
                        ProdField { name,
                                    ty: field_type_name.clone(),
                                    boxed: field_type_name == nonterminal.name }
                    }).collect()
        }
    }

    fn variant_name(&self, nonterminal: &NonTerminal, prod: &Production) -> String {
        format!("{}{}",
                nonterminal.name,
                prod.kind
                .as_ref()
                .unwrap_or(&format!("{}", prod.ntidx + 1)))
    }
}

impl<'a> ActionsGenerator for ProductionActionsGenerator<'a> {
    fn nonterminal_types(
        &self,
        nonterminal: &NonTerminal,
    ) -> Vec<(String, syn::Item)> {
        // Each non-terminal is enum where variants are deduced from
        // the non-terminal productions.
        // If there is 0 content references variant is without content
        // If there is 1 content reference variant contains a value
        //    of the referenced symbol type.
        // If there is >1 content references a new struct will be created and
        // the variant contains a type of this struct.
        // The name of the variant will be either production kind if given or
        // <rule><prod idx>
        // The name of struct field will be either assignment name or
        // <ref_type_in_snakecase>_<position in production>

        let mut types: Vec<(String, syn::Item)> = vec![];
        let prods = nonterminal.productions(self.grammar);

        let variants: Vec<syn::Variant> = prods
            .iter()
            .map(|&prod| {
                let variant_name = self.variant_name(nonterminal, prod);
                let variant_ident = Ident::new(&variant_name, Span::call_site());
                let fields = self.prod_fields(prod);
                match fields.iter().count() {
                    0 => parse_quote! { #variant_ident },
                    1 => {
                        let variant_inner_type = Ident::new(
                            &fields[0].ty,
                            Span::call_site(),
                        );
                        parse_quote! { #variant_ident(#variant_inner_type) }
                    }
                    _ => {
                        // More than one ref with content. Make a new type.
                        let struct_fields: Vec<syn::Field> = fields
                            .into_iter()
                            .map(|f| {
                                let name_ident = Ident::new(&f.name, Span::call_site());
                                let field_type_ident = Ident::new(&f.ty, Span::call_site());

                                syn::Field::parse_named
                                    .parse2(if f.ty == nonterminal.name {
                                        // Handle direct recursion
                                        quote! { pub #name_ident: Box<#field_type_ident> }
                                    } else {
                                        quote! { pub #name_ident: #field_type_ident }
                                    })
                                    .unwrap()
                            }).collect();

                        types.push(
                            (variant_name.clone(),
                             parse_quote! {
                                 #[derive(Debug, Clone)]
                                 pub struct #variant_ident {
                                     #(#struct_fields),*
                                 }
                             }
                        ));

                        parse_quote! { #variant_ident(#variant_ident) }
                    }
                }
            })
            .collect::<Vec<_>>();

        let type_name_ident = Ident::new(&nonterminal.name, Span::call_site());
        types.push((
            nonterminal.name.clone(),
            parse_quote! {
                #[derive(Debug, Clone)]
                pub enum #type_name_ident {
                    #(#variants),*
                }
            },
        ));
        types
    }

    fn nonterminal_actions(
        &self,
        nonterminal: &NonTerminal,
    ) -> Vec<(String, syn::Item)> {
        let prods = nonterminal.productions(self.grammar);

        let actions: Vec<(String, syn::Item)> = prods
            .iter()
            .map(|&prod| {
                let action_name = format!(
                        "{}_{}",
                        nonterminal.name.to_case(Case::Snake),
                        if let Some(ref kind) = prod.kind {
                            kind.to_case(Case::Snake)
                        } else {
                            format!("{}", prod.ntidx + 1)
                        }
                    );
                let action_ident = Ident::new(&action_name, Span::call_site());
                let variant_name = self.variant_name(nonterminal, prod);
                let variant_ident = Ident::new(&variant_name, Span::call_site());
                let ret_type_ident = Ident::new(&nonterminal.name, Span::call_site());
                let mut fn_args: Vec<syn::FnArg> = vec![];
                let mut field_vals: Vec<syn::FieldValue> = vec![];
                let fields = self.prod_fields(prod);

                for field in &fields {
                    let ident = Ident::new(&field.name, Span::call_site());
                    let ty: syn::Type = syn::parse_str(&field.ty).unwrap();
                    fn_args.push(
                        parse_quote! { #ident: #ty }
                    );

                    if field.boxed {
                        field_vals.push(
                            parse_quote! { #ident: Box::new(#ident) }
                        );
                    } else {
                        field_vals.push(
                            parse_quote! { #ident }
                        );
                    }
                }

                let body_expr: syn::Expr = match fields.iter().count() {
                    0 => parse_quote! { #ret_type_ident::#variant_ident },
                    1 => {
                        let inner_var_ident = Ident::new(&fields[0].name,
                                                         Span::call_site());
                        parse_quote! {
                            #ret_type_ident::#variant_ident(#inner_var_ident)
                        }
                    },
                    _ => {
                        // More than one ref with content. Return stuct instance
                        parse_quote!{
                            #ret_type_ident::#variant_ident(
                                #variant_ident {
                                    #(#field_vals),*
                                }
                            )
                        }
                    }
                };

               (action_name,
                parse_quote! {
                    pub fn #action_ident(#(#fn_args),*) -> #ret_type_ident {
                        #body_expr
                    }
                })
            }).collect();

        actions
    }
}
