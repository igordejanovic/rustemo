use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse::Parser, parse_quote};

use crate::grammar::{
    types::{SymbolTypeKind, SymbolTypes, VariantKind, Variant, SymbolType},
    Grammar, NonTerminal,
};

use super::ActionsGenerator;

pub(crate) struct ProductionActionsGenerator {
    types: SymbolTypes,
}

impl ProductionActionsGenerator {
    pub fn new(grammar: &Grammar) -> Box<dyn ActionsGenerator> {
        Box::new(Self {
            types: SymbolTypes::new(grammar),
        })
    }

    fn get_action_args(&self, variant: &Variant) -> Vec<syn::FnArg> {
        let mut fn_args: Vec<syn::FnArg> = vec![];

        match &variant.kind {
            VariantKind::Plain => (), // No args for plain enum
            VariantKind::Struct(_, fields) => {
                for field in fields {
                    let f_name =
                        Ident::new(&field.name, Span::call_site());
                    let f_type =
                        Ident::new(&field.ty, Span::call_site());
                    fn_args.push(parse_quote! { #f_name: #f_type });
                }
            }
            VariantKind::Ref(ref_type) => {
                let ty = Ident::new(&ref_type, Span::call_site());
                let name = Ident::new(
                    &ref_type.to_case(Case::Snake),
                    Span::call_site(),
                );
                fn_args.push(parse_quote! { #name: #ty });
            }
        };
        fn_args
    }

    fn get_action_body(&self, ty: &SymbolType, variant: &Variant, variants_len: usize) -> syn::Expr {
        let ty_ident = Ident::new(&ty.name, Span::call_site());
        let variant_ident =
            Ident::new(&variant.name, Span::call_site());
        let expr: syn::Expr = match &variant.kind {
            VariantKind::Plain => {
                parse_quote! { #ty_ident::#variant_ident }
            }
            VariantKind::Struct(name, fields) => {
                let struct_ty = Ident::new(&name, Span::call_site());
                let fields: Vec<syn::FieldValue> = fields
                    .iter()
                    .map(|f| {
                        let field =
                            Ident::new(&f.name, Span::call_site());
                        if f.recursive {
                            parse_quote! { #field: Box::new(#field) }
                        } else {
                            parse_quote! { #field }
                        }
                    })
                    .collect();

                if variants_len == 1 {
                    // Promote to struct
                    parse_quote! {
                        #struct_ty {
                            #(#fields),*
                        }
                    }
                } else {
                    parse_quote! {
                        #ty_ident::#variant_ident(
                            #struct_ty {
                                #(#fields),*
                            }
                        )
                    }
                }
            }
            VariantKind::Ref(ref_type) => {
                let ref_type = Ident::new(
                    &ref_type.to_case(Case::Snake),
                    Span::call_site(),
                );
                parse_quote! {
                    #ty_ident::#variant_ident(#ref_type)
                }
            }
        };

        if ty.optional {
            parse_quote! { Some(#expr) }
        } else {
            expr
        }
    }
}

impl ActionsGenerator for ProductionActionsGenerator {
    fn nonterminal_types(
        &self,
        nonterminal: &NonTerminal,
    ) -> Vec<syn::Item> {
        let ty = self.types.get_type(&nonterminal.name);
        let type_ident = Ident::new(&nonterminal.name, Span::call_site());

        match &ty.kind {
            SymbolTypeKind::Enum(variants) => {
                // Derived struct types
                let mut types: Vec<syn::Item> =
                    variants.iter().filter_map(|v| {
                    match &v.kind {
                        VariantKind::Struct(type_name, fields) => {
                            let type_ident = Ident::new(&type_name, Span::call_site());
                            let fields: Vec<syn::Field> = fields.iter().map(|f| {
                                let field_name = Ident::new(&f.name, Span::call_site());
                                let field_type = Ident::new(&f.ty, Span::call_site());
                                syn::Field::parse_named
                                    .parse2(
                                        if f.recursive {
                                            // Handle direct recursion
                                            quote! { pub #field_name: Box<#field_type> }
                                        } else {
                                            quote! {pub #field_name: #field_type}
                                        }
                                    ).unwrap()
                            }).collect();
                            Some(parse_quote! {
                                #[derive(Debug, Clone)]
                                pub struct #type_ident {
                                    #(#fields),*
                                }
                            })
                        },
                        _ => None,
                    }
                }).collect();

                // Enum type
                let variants: Vec<syn::Variant> = variants
                    .iter()
                    .map(|v| {
                        let variant_ident =
                            Ident::new(&v.name, Span::call_site());
                        match &v.kind {
                            VariantKind::Plain => {
                                parse_quote! { #variant_ident }
                            }
                            VariantKind::Struct(type_name, _) => {
                                let type_ident = Ident::new(&type_name, Span::call_site());
                                parse_quote! { #variant_ident(#type_ident) }
                            }
                            VariantKind::Ref(ref_type) => {
                                let ref_type =
                                    Ident::new(&ref_type, Span::call_site());
                                parse_quote! { #variant_ident(#ref_type) }
                            }
                        }
                    })
                    .collect();

                types.push(
                    parse_quote! {
                        #[derive(Debug, Clone)]
                        pub enum #type_ident {
                            #(#variants),*
                        }
                    },
                );
                types
            }
            SymbolTypeKind::Terminal => unreachable!(),
        }
    }

    fn nonterminal_actions(
        &self,
        nonterminal: &NonTerminal,
    ) -> Vec<(String, syn::Item)> {
        let ty = self.types.get_type(&nonterminal.name);

        match &ty.kind {
            SymbolTypeKind::Enum(variants) => {
                variants.iter().map(|v| {
                    let action_name = v.name.to_case(Case::Snake);
                    let action = Ident::new(&action_name, Span::call_site());
                    let args = self.get_action_args(v);
                    let ret_type = Ident::new(&nonterminal.name,
                                                Span::call_site());
                    let body = self.get_action_body(ty, v, variants.len());

                    (action_name, parse_quote!{
                        pub fn #action(#(#args),*) -> #ret_type {
                            #body
                        }
                    })
                }).collect()
            },
            SymbolTypeKind::Terminal => unreachable!{},
        }
    }
}
