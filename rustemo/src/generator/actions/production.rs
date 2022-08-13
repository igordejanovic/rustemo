use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse::Parser, parse_quote};

use crate::grammar::{
    types::{
        to_snake_case, SymbolType, SymbolTypeKind, SymbolTypes, Variant,
        VariantKind,
    },
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

    fn get_action_args(
        &self,
        ty: &SymbolType,
        variant: &Variant,
    ) -> Vec<syn::FnArg> {
        let mut fn_args: Vec<syn::FnArg> = vec![];

        match &variant.kind {
            VariantKind::Plain => (), // No args for plain enum
            VariantKind::Struct(_, fields) => {
                for field in fields {
                    let f_name = Ident::new(&field.name, Span::call_site());
                    let f_type = Ident::new(&field.ty, Span::call_site());

                    // If this type is Vec and ref type is recursion make it
                    // mutable to support *, +...
                    if matches! { ty.kind, SymbolTypeKind::Vec(_, _) } {
                        if ty.name == field.ty {
                            fn_args.push(parse_quote! { mut #f_name: #f_type });
                        } else {
                            fn_args.push(parse_quote! { #f_name: #f_type });
                        }
                    } else {
                        fn_args.push(parse_quote! { #f_name: #f_type });
                    }
                }
            }
            VariantKind::Ref(ref_type) => {
                let ty = Ident::new(&ref_type, Span::call_site());
                let name =
                    Ident::new(&to_snake_case(ref_type), Span::call_site());
                fn_args.push(parse_quote! { #name: #ty });
            }
            VariantKind::Empty => (),
        };
        fn_args
    }

    fn get_action_body(
        &self,
        ty: &SymbolType,
        target_type: &String,
        variant: &Variant,
        variants_len: usize,
        optional: bool,
    ) -> syn::Expr {
        let target_type = Ident::new(target_type, Span::call_site());
        let variant_ident = Ident::new(&variant.name, Span::call_site());
        let expr: syn::Expr = match &variant.kind {
            VariantKind::Plain => {
                parse_quote! { #target_type::#variant_ident }
            }
            VariantKind::Struct(name, fields) => {
                let struct_ty = Ident::new(&name, Span::call_site());
                let fields: Vec<syn::FieldValue> = fields
                    .iter()
                    .map(|f| {
                        let field = Ident::new(&f.name, Span::call_site());
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
                        #target_type::#variant_ident(
                            #struct_ty {
                                #(#fields),*
                            }
                        )
                    }
                }
            }
            VariantKind::Ref(ref_type) => {
                let ref_type_var =
                    Ident::new(&to_snake_case(ref_type), Span::call_site());
                if matches! { &ty.kind,
                SymbolTypeKind::OptionEnum(_, _)
                | SymbolTypeKind::Enum(_, _) }
                {
                    parse_quote! {
                        #target_type::#variant_ident(#ref_type_var)
                    }
                } else {
                    parse_quote! {
                        #ref_type_var
                    }
                }
            }
            VariantKind::Empty => parse_quote! { None },
        };

        if optional && !matches! { variant.kind, VariantKind::Empty } {
            parse_quote! { Some(#expr) }
        } else {
            expr
        }
    }
}

impl ActionsGenerator for ProductionActionsGenerator {
    fn nonterminal_types(&self, nonterminal: &NonTerminal) -> Vec<syn::Item> {
        let ty = self.types.get_type(&nonterminal.name);
        let type_ident = Ident::new(&nonterminal.name, Span::call_site());

        fn get_variant_types(variants: &Vec<Variant>) -> Vec<syn::Item> {
            // Derived struct types
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
            }).collect()
        }

        fn get_variants(variants: &Vec<Variant>) -> Vec<syn::Variant> {
            variants
                .iter()
                .filter_map(|v| {
                    let variant_ident = Ident::new(&v.name, Span::call_site());
                    match &v.kind {
                        VariantKind::Plain => {
                            Some(parse_quote! { #variant_ident })
                        }
                        VariantKind::Struct(type_name, _) => {
                            let type_ident =
                                Ident::new(&type_name, Span::call_site());
                            Some(parse_quote! { #variant_ident(#type_ident) })
                        }
                        VariantKind::Ref(ref_type) => {
                            let ref_type =
                                Ident::new(&ref_type, Span::call_site());
                            Some(parse_quote! { #variant_ident(#ref_type) })
                        }
                        VariantKind::Empty => None,
                    }
                })
                .collect()
        }

        match &ty.kind {
            SymbolTypeKind::Enum(ref_type, variants) => {
                let mut types = get_variant_types(variants);
                let variants = get_variants(variants);
                let ref_type = Ident::new(&ref_type, Span::call_site());

                types.push(parse_quote! {
                    #[derive(Debug, Clone)]
                    pub enum #ref_type {
                        #(#variants),*
                    }
                });
                types
            }
            SymbolTypeKind::Terminal => unreachable!(),
            SymbolTypeKind::Option(ref_type, _) => {
                let ref_type = Ident::new(&ref_type, Span::call_site());
                vec![parse_quote! { pub type #type_ident = Option<#ref_type>; }]
            }
            SymbolTypeKind::Vec(ref_type, _) => {
                let ref_type = Ident::new(&ref_type, Span::call_site());
                vec![parse_quote! { pub type #type_ident = Vec<#ref_type>; }]
            }
            SymbolTypeKind::OptionEnum(enum_type_name, variants) => {
                let mut types = get_variant_types(variants);
                let variants = get_variants(variants);
                let enum_type = Ident::new(&enum_type_name, Span::call_site());

                types.push(
                    parse_quote! {pub type #type_ident = Option<#enum_type>;},
                );
                types.push(parse_quote! {
                    #[derive(Debug, Clone)]
                    pub enum #enum_type {
                        #(#variants),*
                    }
                });
                types
            }
        }
    }

    fn nonterminal_actions(
        &self,
        nonterminal: &NonTerminal,
    ) -> Vec<(String, syn::Item)> {
        let ty = self.types.get_type(&nonterminal.name);
        let ret_type = Ident::new(&nonterminal.name, Span::call_site());

        match &ty.kind {
            SymbolTypeKind::Enum(target_type, variants)
            | SymbolTypeKind::Option(target_type, variants)
            | SymbolTypeKind::OptionEnum(target_type, variants) => variants
                .iter()
                .map(|v| {
                    let action_name =
                        to_snake_case(format!("{}_{}", ty.name, v.name));
                    let action = Ident::new(&action_name, Span::call_site());
                    let args = self.get_action_args(ty, v);
                    let body = self.get_action_body(
                        ty,
                        target_type,
                        v,
                        variants.len(),
                        matches! {ty.kind,
                        SymbolTypeKind::Option(..) |
                        SymbolTypeKind::OptionEnum(..)},
                    );

                    (
                        action_name,
                        parse_quote! {
                            pub fn #action(#(#args),*) -> #ret_type {
                                #body
                            }
                        },
                    )
                })
                .collect(),
            SymbolTypeKind::Vec(_, variants) => variants
                .iter()
                .map(|v| {
                    let action_name =
                        to_snake_case(format!("{}_{}", ty.name, v.name));
                    let action = Ident::new(&action_name, Span::call_site());
                    let args = self.get_action_args(ty, v);

                    let mut body: Vec<syn::Expr> = vec![];

                    match &v.kind {
                        VariantKind::Empty => {
                            body.push(parse_quote! { vec![] })
                        }
                        VariantKind::Struct(_, fields) => {
                            match &fields[..] {
                                [a, b] => {
                                    let a_i =
                                        Ident::new(&a.name, Span::call_site());
                                    let b_i =
                                        Ident::new(&b.name, Span::call_site());
                                    // Find which one is a vector
                                    if a.ty == nonterminal.name {
                                        body.push(
                                            parse_quote! { #a_i.push(#b_i) },
                                        );
                                        body.push(parse_quote! { #a_i });
                                    } else {
                                        body.push(
                                            parse_quote! { #b_i.push(#a_i) },
                                        );
                                        body.push(parse_quote! { #b_i });
                                    }
                                }
                                [a] => {
                                    let a_i = Ident::new(
                                        &to_snake_case(&a.name),
                                        Span::call_site(),
                                    );
                                    body.push(parse_quote! { vec![#a_i] });
                                }
                                _ => unreachable!(),
                            }
                        }
                        VariantKind::Ref(ref_type) => {
                            let i = Ident::new(
                                &to_snake_case(ref_type),
                                Span::call_site(),
                            );
                            body.push(parse_quote! { vec![#i] });
                        }
                        VariantKind::Plain => unreachable!(),
                    };

                    (
                        action_name,
                        parse_quote! {
                            pub fn #action(#(#args),*) -> #ret_type {
                                #(#body);*
                            }
                        },
                    )
                })
                .collect(),
            SymbolTypeKind::Terminal => unreachable! {},
        }
    }
}
