//! Inferring types from rustemo grammars.
//! This is a base support for auto AST inference.

use convert_case::{Case, Casing};
use rustemo_rt::index::SymbolIndex;

use super::Grammar;

#[cfg(test)]
mod tests;

/// Returns a vector of all types inferred from the provided grammar.
pub(crate) fn symbol_types(grammar: &Grammar) -> Vec<SymbolType> {
    let mut types = vec![];
    for terminal in &grammar.terminals {
        // Each terminal produces `Terminal` kind which maps to String by default
        types.push(SymbolType {
            name: terminal.name.clone(),
            symbol: grammar.term_to_symbol_index(terminal.idx),
            kind: SymbolTypeKind::Terminal,
            optional: false,
        })
    }

    for nonterminal in &grammar.nonterminals {
        let mut variants = vec![];
        let symbol = grammar.nonterm_to_symbol_index(nonterminal.idx);

        // By default, NT maps to enum type. Each production is a variant,
        // except EMPTY.
        for production in &nonterminal.productions(grammar) {
            if production.rhs.len() == 0 {
                // Empty production
                continue;
            }
            let variant_name = format!(
                "{}_{}",
                nonterminal.name,
                production
                    .kind
                    .as_ref()
                    .map_or((production.ntidx + 1).to_string(), |k| k.clone())
            );

            // Enum variants are deduced by the following rules:
            // - No content references => plain variant without inner content
            // - A single content. ref and no assig LHS => variant with
            //   a referred NT type as its content
            // - Multiple content. refs => Variant with a new struct type
            //   where fields types are types of the referred symbols.
            let rhs = production.rhs_with_content(grammar);
            variants.push(match rhs.iter().count() {
                0 => Variant {
                    name: variant_name,
                    ty: None,
                },
                1 if rhs[0].name.is_none() => {
                    let ref_type = grammar.symbol_name(rhs[0].symbol);
                    if ref_type == nonterminal.name {
                        panic!("Infinite recursion on symbol '{}'", ref_type);
                    }
                    Variant {
                        name: variant_name,
                        ty: Some(ref_type.clone()),
                    }
                }
                _ => {
                    let mut type_fields = vec![];
                    for assign in rhs {
                        let ref_type = grammar.symbol_name(assign.symbol);
                        let name = assign
                            .name
                            .unwrap_or(ref_type.to_case(Case::Snake));
                        type_fields.push(Field {
                            name,
                            ty: ref_type.clone(),
                            recursive: ref_type == nonterminal.name,
                        })
                    }
                    types.push(SymbolType {
                        name: variant_name.clone(),
                        symbol,
                        kind: SymbolTypeKind::Struct(type_fields),
                        optional: false,
                    });
                    Variant {
                        name: variant_name.clone(),
                        ty: Some(variant_name),
                    }
                }
            });
        }

        // If NT has empty production type is optional
        let type_optional = nonterminal
            .productions(grammar)
            .iter()
            .find(|p| p.rhs.len() == 0)
            .is_some();

        if variants.len() == 1 {
            let variant = &variants[0];
            if variant.ty.is_none() {
                // No content in this NT. Do not create type.
                continue;
            } else {
                // Variant is referencing other type
                types.push(SymbolType {
                    name: nonterminal.name.clone(),
                    symbol,
                    kind: SymbolTypeKind::Ref(variant.name.clone()),
                    optional: type_optional,
                });
            }
        } else {
            // Variant is a proper enum
            types.push(SymbolType {
                name: nonterminal.name.clone(),
                symbol,
                kind: SymbolTypeKind::Enum(variants),
                optional: type_optional,
            });
        }
    }
    types
}

#[derive(Debug)]
pub(crate) struct SymbolType {
    name: String,
    symbol: SymbolIndex,
    kind: SymbolTypeKind,
    optional: bool,
}

#[derive(Debug)]
pub(crate) enum SymbolTypeKind {
    Enum(Vec<Variant>),
    Struct(Vec<Field>),
    Terminal,
    Ref(String),
}

#[derive(Debug)]
pub(crate) struct Variant {
    name: String,
    ty: Option<String>,
}

#[derive(Debug)]
pub(crate) struct Field {
    name: String,
    ty: String,
    recursive: bool,
}
