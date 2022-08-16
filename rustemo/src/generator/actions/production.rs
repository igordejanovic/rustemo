use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse::Parser, parse_quote};

use crate::grammar::{
    types::{
        to_snake_case, Choice, ChoiceKind, SymbolType, SymbolTypeKind,
        SymbolTypes,
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
        choice: &Choice,
    ) -> Vec<syn::FnArg> {
        let mut fn_args: Vec<syn::FnArg> = vec![];

        match &choice.kind {
            ChoiceKind::Plain => (), // No args for plain enum
            ChoiceKind::Struct(_, fields) => {
                for field in fields {
                    let f_name = Ident::new(&field.name, Span::call_site());
                    let f_type = Ident::new(&field.ty, Span::call_site());

                    // If this type is Vec and ref type is recursion make it
                    // mutable to support *, +...
                    if matches! { ty.kind, SymbolTypeKind::Vec{ .. } }
                        && ty.name == field.ty
                    {
                        fn_args.push(parse_quote! { mut #f_name: #f_type });
                    } else {
                        fn_args.push(parse_quote! { #f_name: #f_type });
                    }
                }
            }
            ChoiceKind::Ref(ref_type) => {
                let ty = Ident::new(&ref_type, Span::call_site());
                let name =
                    Ident::new(&to_snake_case(ref_type), Span::call_site());
                fn_args.push(parse_quote! { #name: #ty });
            }
            ChoiceKind::Empty => (),
        };
        fn_args
    }

    fn get_action_body(
        &self,
        ty: &SymbolType,
        target_type: &str,
        choice: &Choice,
    ) -> syn::Expr {
        let target_type = Ident::new(target_type, Span::call_site());
        let choice_ident = Ident::new(&choice.name, Span::call_site());
        let expr: syn::Expr = match &choice.kind {
            ChoiceKind::Plain => {
                parse_quote! { #target_type::#choice_ident }
            }
            ChoiceKind::Struct(type_name, fields) => {
                let struct_ty = Ident::new(&type_name, Span::call_site());
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

                if matches!(ty.kind, SymbolTypeKind::Enum {..}) {
                    parse_quote! {
                        #target_type::#choice_ident(
                            #struct_ty {
                                #(#fields),*
                            }
                        )
                    }
                } else {
                    parse_quote! {
                        #target_type {
                            #(#fields),*
                        }
                    }
                }
            }
            ChoiceKind::Ref(ref_type) => {
                let ref_type_var =
                    Ident::new(&to_snake_case(ref_type), Span::call_site());
                if matches!(&ty.kind, SymbolTypeKind::Ref{..}) {
                    parse_quote! {
                        #ref_type_var
                    }
                } else {
                    parse_quote! {
                        #target_type::#choice_ident(#ref_type_var)
                    }
                }
            }
            ChoiceKind::Empty => parse_quote! { None }
        };

        let optional = match ty.kind {
            SymbolTypeKind::Ref { optional: o, .. }
            | SymbolTypeKind::Vec { optional: o , .. }
            | SymbolTypeKind::Struct { optional: o , .. }
            | SymbolTypeKind::Enum { optional: o , .. } => {
                o
            },
            SymbolTypeKind::Terminal => unreachable!(),
        };
        if optional && !matches!(choice.kind, ChoiceKind::Empty){
            parse_quote! { Some(#expr) }
        } else {
            expr
        }
    }
}

impl ActionsGenerator for ProductionActionsGenerator {
    fn nonterminal_types(&self, nonterminal: &NonTerminal) -> Vec<syn::Item> {
        let ty = self.types.get_type(&nonterminal.name);
        let type_ident = Ident::new(&ty.name, Span::call_site());

        fn get_choice_type(
            choice: &Choice,
            type_name: Option<&String>,
        ) -> Option<syn::Item> {
            match &choice.kind {
                ChoiceKind::Struct(struct_type, fields) => {
                    let type_ident = if let Some(type_name) = type_name {
                        Ident::new(&type_name, Span::call_site())
                    } else {
                        Ident::new(&struct_type, Span::call_site())
                    };

                    let fields: Vec<syn::Field> = fields
                        .iter()
                        .map(|f| {
                            let field_name =
                                Ident::new(&f.name, Span::call_site());
                            let field_type =
                                Ident::new(&f.ty, Span::call_site());
                            syn::Field::parse_named
                                .parse2(if f.recursive {
                                    // Handle direct recursion
                                    quote! { pub #field_name: Box<#field_type> }
                                } else {
                                    quote! {pub #field_name: #field_type}
                                })
                                .unwrap()
                        })
                        .collect();
                    Some(parse_quote! {
                        #[derive(Debug, Clone)]
                        pub struct #type_ident {
                            #(#fields),*
                        }
                    })
                }
                _ => None,
            }
        }

        fn get_choice_types(
            choices: &Vec<Choice>,
            type_name: Option<&String>,
        ) -> Vec<syn::Item> {
            choices
                .iter()
                .filter_map(|choice| get_choice_type(choice, type_name))
                .collect()
        }

        fn get_variants(choices: &Vec<Choice>) -> Vec<syn::Variant> {
            choices
                .iter()
                .filter_map(|v| {
                    let variant_ident = Ident::new(&v.name, Span::call_site());
                    match &v.kind {
                        ChoiceKind::Plain => {
                            Some(parse_quote! { #variant_ident })
                        }
                        ChoiceKind::Struct(type_name, _) => {
                            let type_ident =
                                Ident::new(&type_name, Span::call_site());
                            Some(parse_quote! { #variant_ident(#type_ident) })
                        }
                        ChoiceKind::Ref(ref_type) => {
                            let ref_type =
                                Ident::new(&ref_type, Span::call_site());
                            Some(parse_quote! { #variant_ident(#ref_type) })
                        }
                        ChoiceKind::Empty => None,
                    }
                })
                .collect()
        }

        match &ty.kind {
            SymbolTypeKind::Enum {
                name: ref_type,
                choices,
                optional,
            } => {
                let mut types = get_choice_types(choices, None);
                let variants = get_variants(choices);
                let ref_type = Ident::new(&ref_type, Span::call_site());

                if *optional {
                    types.push(
                        parse_quote! {pub type #type_ident = Option<#ref_type>;},
                    );
                    types.push(parse_quote! {
                        #[derive(Debug, Clone)]
                        pub enum #ref_type {
                            #(#variants),*
                        }
                    });
                } else {
                    types.push(parse_quote! {
                        #[derive(Debug, Clone)]
                        pub enum #type_ident {
                            #(#variants),*
                        }
                    });
                }
                types
            }
            SymbolTypeKind::Struct {
                name: struct_type,
                choices,
                optional,
            } => {
                let mut types = get_choice_types(choices, Some(&struct_type));
                let struct_type = Ident::new(&struct_type, Span::call_site());
                if *optional {
                    types.push(
                        parse_quote! {pub type #type_ident = Option<#struct_type>;},
                    );
                }
                types
            }
            SymbolTypeKind::Ref {
                name: ref_type,
                optional,
                ..
            } => {
                let ref_type = Ident::new(&ref_type, Span::call_site());
                if *optional {
                    vec![
                        parse_quote! { pub type #type_ident = Option<#ref_type>; },
                    ]
                } else {
                    vec![parse_quote! { pub type #type_ident = #ref_type; }]
                }
            }
            SymbolTypeKind::Vec { name: ref_type, .. } => {
                let ref_type = Ident::new(&ref_type, Span::call_site());
                vec![parse_quote! { pub type #type_ident = Vec<#ref_type>; }]
            }
            SymbolTypeKind::Terminal => unreachable!(),
        }
    }

    fn nonterminal_actions(
        &self,
        nonterminal: &NonTerminal,
    ) -> Vec<(String, syn::Item)> {
        let ty = self.types.get_type(&nonterminal.name);
        let ret_type = Ident::new(&nonterminal.name, Span::call_site());

        match &ty.kind {
            SymbolTypeKind::Enum{ name: target_type, choices, .. }
            | SymbolTypeKind::Struct{ name: target_type, choices, ..}
            | SymbolTypeKind::Ref{ name: target_type, choices, ..} => choices
                .iter()
                .map(|v| {
                    let action_name =
                        to_snake_case(format!("{}_{}", ty.name, v.name));
                    let action = Ident::new(&action_name, Span::call_site());
                    let args = self.get_action_args(ty, v);
                    let body = self.get_action_body(
                        ty,
                        target_type,
                        v);

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
            SymbolTypeKind::Vec{choices, ..} => choices
                .iter()
                .map(|v| {
                    let action_name =
                        to_snake_case(format!("{}_{}", ty.name, v.name));
                    let action = Ident::new(&action_name, Span::call_site());
                    let args = self.get_action_args(ty, v);

                    let mut body: Vec<syn::Expr> = vec![];

                    match &v.kind {
                        ChoiceKind::Empty => body.push(parse_quote! { vec![] }),
                        ChoiceKind::Struct(_, fields) => {
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
                        ChoiceKind::Ref(ref_type) => {
                            let i = Ident::new(
                                &to_snake_case(ref_type),
                                Span::call_site(),
                            );
                            body.push(parse_quote! { vec![#i] });
                        }
                        ChoiceKind::Plain => unreachable!(),
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
