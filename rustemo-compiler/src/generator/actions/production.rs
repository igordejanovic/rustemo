use quote::{format_ident, quote};
use syn::{parse::Parser, parse_quote};

use crate::{
    generator::action_name,
    grammar::{
        types::{to_snake_case, Choice, ChoiceKind, SymbolType, SymbolTypeKind, SymbolTypes},
        Grammar, NonTerminal,
    },
    settings::Settings,
};

use super::ActionsGenerator;

macro_rules! name_valloc {
    ($base_str:expr, $s:ident) => {
        if $s.builder_loc_info {
            format!("{}Base", $base_str)
        } else {
            $base_str.to_string()
        }
    };
}

pub(crate) struct ProductionActionsGenerator<'t> {
    types: &'t SymbolTypes,
    term_len: usize,
}

impl<'t> ProductionActionsGenerator<'t> {
    // TODO: Rework this to be aligned with conventions
    #[allow(clippy::new_ret_no_self)]
    pub fn new(grammar: &Grammar, types: &'t SymbolTypes) -> Box<dyn ActionsGenerator + 't> {
        Box::new(Self {
            types,
            term_len: grammar.terminals.len(),
        })
    }

    fn get_action_args(&self, ty: &SymbolType, choice: &Choice) -> Vec<syn::FnArg> {
        let mut fn_args: Vec<syn::FnArg> = vec![];

        match &choice.kind {
            ChoiceKind::Plain => (), // No args for plain enum
            ChoiceKind::Struct { fields, .. } => {
                for field in fields {
                    let f_name = format_ident!("{}", field.name);
                    let f_type = format_ident!("{}", field.ref_type);

                    // If this type is Vec and ref type is recursion make it
                    // mutable to support *, +...
                    if matches! { ty.kind, SymbolTypeKind::Vec{ .. } } && ty.name == field.ref_type
                    {
                        fn_args.push(parse_quote! { mut #f_name: #f_type });
                    } else {
                        fn_args.push(parse_quote! { #f_name: #f_type });
                    }
                }
            }
            ChoiceKind::Ref { ref_type, .. } => {
                let ty = format_ident!("{ref_type}");
                let name = format_ident!("{}", to_snake_case(ref_type));
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
        settings: &Settings,
    ) -> syn::Expr {
        let target_type = format_ident!("{target_type}");
        let target_type_base = format_ident!("{target_type}Base");
        let choice_ident = format_ident!("{}", choice.name);
        let expr: syn::Expr = match &choice.kind {
            ChoiceKind::Plain => {
                parse_quote! { #target_type::#choice_ident }
            }
            ChoiceKind::Struct { type_name, fields } => {
                let struct_ty = format_ident!("{type_name}");
                let struct_ty_base = format_ident!("{type_name}Base");
                let fields: Vec<syn::FieldValue> = fields
                    .iter()
                    .map(|f| {
                        let field = format_ident!("{}", f.name);
                        if f.recursive.get() {
                            parse_quote! { #field: Box::new(#field) }
                        } else {
                            parse_quote! { #field }
                        }
                    })
                    .collect();

                if matches!(ty.kind, SymbolTypeKind::Enum { .. }) {
                    if settings.builder_loc_info {
                        parse_quote! {
                            #target_type::#choice_ident(
                                #struct_ty::new(#struct_ty_base {
                                    #(#fields),*
                                }, Some(_ctx.span()))
                            )
                        }
                    } else {
                        parse_quote! {
                            #target_type::#choice_ident(
                                #struct_ty {
                                    #(#fields),*
                                }
                            )
                        }
                    }
                } else if settings.builder_loc_info {
                    parse_quote! {
                        #target_type::new(#target_type_base {
                            #(#fields),*
                        }, Some(_ctx.span()))
                    }
                } else {
                    parse_quote! {
                        #target_type {
                            #(#fields),*
                        }
                    }
                }
            }
            ChoiceKind::Ref {
                ref_type,
                recursive,
            } => {
                let ref_type_var_ident = format_ident!("{}", to_snake_case(ref_type));
                let mut ref_type_var: syn::Expr = parse_quote! { #ref_type_var_ident };

                if recursive.get() {
                    ref_type_var = parse_quote! { Box::new(#ref_type_var) }
                }
                if matches!(&ty.kind, SymbolTypeKind::Ref { .. }) {
                    parse_quote! {
                        #ref_type_var
                    }
                } else {
                    parse_quote! {
                        #target_type::#choice_ident(#ref_type_var)
                    }
                }
            }
            ChoiceKind::Empty => parse_quote! { None },
        };

        if ty.optional && !matches!(choice.kind, ChoiceKind::Empty) {
            parse_quote! { Some(#expr) }
        } else {
            expr
        }
    }
}

impl ActionsGenerator for ProductionActionsGenerator<'_> {
    fn nonterminal_types(&self, nonterminal: &NonTerminal, settings: &Settings) -> Vec<syn::Item> {
        let ty = self
            .types
            .get_type(nonterminal.idx.symbol_index(self.term_len));
        let type_ident = format_ident!("{}", ty.name);

        let get_choice_type =
            |choice: &Choice, type_name: Option<&str>| -> Option<Vec<syn::Item>> {
                match &choice.kind {
                    ChoiceKind::Struct {
                        type_name: struct_type,
                        fields,
                    } => {
                        let type_name = if let Some(type_name) = type_name {
                            type_name.to_string()
                        } else {
                            struct_type.into()
                        };
                        let type_ident = format_ident!("{}", name_valloc!(type_name, settings));

                        let fields: Vec<syn::Field> = fields
                            .iter()
                            .map(|f| {
                                let field_name = format_ident!("{}", f.name);
                                let field_type = format_ident!("{}", f.ref_type);
                                syn::Field::parse_named
                                    .parse2(if f.recursive.get() {
                                        // Handle direct recursion
                                        quote! { pub #field_name: Box<#field_type> }
                                    } else {
                                        quote! {pub #field_name: #field_type}
                                    })
                                    .unwrap()
                            })
                            .collect();

                        let mut types = vec![];
                        types.push(parse_quote! {
                            #[derive(Debug, Clone)]
                            pub struct #type_ident {
                                #(#fields),*
                            }
                        });
                        if settings.builder_loc_info {
                            let type_ident_loc = format_ident!("{type_name}");
                            types.push(parse_quote! {
                                pub type #type_ident_loc = ValSpan<#type_ident>;
                            });
                        }
                        Some(types)
                    }
                    _ => None,
                }
            };

        let get_choice_types = |choices: &[Choice], type_name: Option<&str>| -> Vec<syn::Item> {
            choices
                .iter()
                .filter_map(|choice| get_choice_type(choice, type_name))
                .flatten()
                .collect()
        };

        let get_variants = |choices: &[Choice]| -> Vec<syn::Variant> {
            choices
                .iter()
                .filter_map(|v| {
                    let variant_ident = format_ident!("{}", v.name);
                    match &v.kind {
                        ChoiceKind::Plain => Some(parse_quote! { #variant_ident }),
                        ChoiceKind::Struct { type_name, .. } => {
                            let type_ident = format_ident!("{type_name}");
                            Some(parse_quote! { #variant_ident(#type_ident) })
                        }
                        ChoiceKind::Ref {
                            ref_type,
                            recursive,
                        } => {
                            let ref_type = format_ident!("{ref_type}");
                            let ref_type: syn::Type = if recursive.get() {
                                parse_quote! { Box<#ref_type> }
                            } else {
                                parse_quote! { #ref_type }
                            };
                            Some(parse_quote! { #variant_ident(#ref_type) })
                        }
                        ChoiceKind::Empty => None,
                    }
                })
                .collect()
        };

        match &ty.kind {
            SymbolTypeKind::Enum {
                type_name: enum_type,
            } => {
                let mut types = get_choice_types(&ty.choices, None);
                let variants = get_variants(&ty.choices);
                let enum_type = format_ident!("{enum_type}");

                if ty.optional {
                    types.push(parse_quote! {pub type #type_ident = Option<#enum_type>;});
                }
                types.push(parse_quote! {
                    #[derive(Debug, Clone)]
                    pub enum #enum_type {
                        #(#variants),*
                    }
                });
                types
            }
            SymbolTypeKind::Struct {
                type_name: struct_type,
            } => {
                let mut types = get_choice_types(&ty.choices, Some(struct_type));
                let struct_type = format_ident!("{struct_type}");
                if ty.optional {
                    types.push(parse_quote! {pub type #type_ident = Option<#struct_type>;});
                }
                types
            }
            SymbolTypeKind::Ref {
                ref_type,
                recursive,
            } => {
                let ref_type = format_ident!("{ref_type}");
                let ref_type: syn::Type = if recursive.get() {
                    parse_quote! { Box<#ref_type> }
                } else {
                    parse_quote! { #ref_type }
                };
                if ty.optional {
                    vec![parse_quote! { pub type #type_ident = Option<#ref_type>; }]
                } else {
                    vec![parse_quote! { pub type #type_ident = #ref_type; }]
                }
            }
            SymbolTypeKind::Vec {
                ref_type,
                recursive,
            } => {
                let ref_type = format_ident!("{ref_type}");
                if recursive.get() {
                    vec![parse_quote! { pub type #type_ident = Vec<Box<#ref_type>>; }]
                } else {
                    vec![parse_quote! { pub type #type_ident = Vec<#ref_type>; }]
                }
            }
            SymbolTypeKind::Terminal => unreachable!(),
        }
    }

    fn nonterminal_actions(
        &self,
        nonterminal: &NonTerminal,
        settings: &Settings,
    ) -> Vec<(String, syn::Item)> {
        let ty = self
            .types
            .get_type(nonterminal.idx.symbol_index(self.term_len));
        let ret_type = format_ident!("{}", nonterminal.name);

        match &ty.kind {
            SymbolTypeKind::Enum {
                type_name: target_type,
            }
            | SymbolTypeKind::Struct {
                type_name: target_type,
            }
            | SymbolTypeKind::Ref {
                ref_type: target_type,
                ..
            } => ty
                .choices
                .iter()
                .map(|choice| {
                    let action_name = action_name(nonterminal, choice);
                    let action = format_ident!("{action_name}");
                    let args = self.get_action_args(ty, choice);
                    let body = self.get_action_body(ty, target_type, choice, settings);
                    (
                        action_name,
                        parse_quote! {
                            pub fn #action(_ctx: &Ctx, #(#args),*) -> #ret_type {
                                #body
                            }
                        },
                    )
                })
                .collect(),
            SymbolTypeKind::Vec { recursive, .. } => ty
                .choices
                .iter()
                .map(|choice| {
                    let action_name = action_name(nonterminal, choice);
                    let action = format_ident!("{action_name}");
                    let args = self.get_action_args(ty, choice);

                    let mut body: Vec<syn::Expr> = vec![];

                    match &choice.kind {
                        ChoiceKind::Empty => body.push(parse_quote! { vec![] }),
                        ChoiceKind::Struct { fields, .. } => {
                            match &fields[..] {
                                [a, b] => {
                                    let mut a_i = format_ident!("{}", a.name);
                                    let mut b_i = format_ident!("{}", b.name);
                                    // Find which one is a vector
                                    if b.ref_type == nonterminal.name {
                                        (a_i, b_i) = (b_i, a_i)
                                    }
                                    body.push(if recursive.get() {
                                        parse_quote! { #a_i.push(Box::new(#b_i)) }
                                    } else {
                                        parse_quote! { #a_i.push(#b_i) }
                                    });
                                    body.push(parse_quote! { #a_i });
                                }
                                [a] => {
                                    let a_i = format_ident!("{}", to_snake_case(&a.name));
                                    if recursive.get() {
                                        body.push(parse_quote! { vec![Box::new(#a_i)] });
                                    } else {
                                        body.push(parse_quote! { vec![#a_i] });
                                    }
                                }
                                _ => unreachable!(),
                            }
                        }
                        ChoiceKind::Ref { ref_type, .. } => {
                            let i = format_ident!("{}", to_snake_case(ref_type));
                            if recursive.get() {
                                log!("{:?}", i);
                                body.push(parse_quote! { vec![Box::new(#i)] });
                            } else {
                                body.push(parse_quote! { vec![#i] });
                            }
                        }
                        ChoiceKind::Plain => unreachable!(),
                    };

                    (
                        action_name,
                        parse_quote! {
                            pub fn #action(_ctx: &Ctx, #(#args),*) -> #ret_type {
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
