//! Inferring types from rustemo grammars.
//! This is a base support for auto AST inference.

use convert_case::{Case, Casing, Boundary};

use super::{Grammar, Production};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub(crate) struct SymbolTypes {
    symbol_types: Vec<SymbolType>,
}

pub(crate) fn to_snake_case<S: AsRef<str>>(s: S) -> String {
    s.as_ref().with_boundaries(&[Boundary::LowerUpper]).to_case(Case::Snake)
}

pub(crate) fn to_pascal_case<S: AsRef<str>>(s: S) -> String {
    s.as_ref().to_case(Case::Pascal)
}

pub(crate) fn variant_name(prod: &Production) -> String {
    if let Some(ref kind) = prod.kind {
        kind.clone()
    } else if prod.rhs.len() == 0 {
        String::from("Empty")
    } else {
        format!("V{}", prod.ntidx + 1)
    }
}

impl SymbolTypes {
    pub fn new(grammar: &Grammar) -> Self {
        Self {
            symbol_types: Self::symbol_types(grammar),
        }
    }

    pub(crate) fn get_type(&self, ty: &str) -> &SymbolType {
        self.symbol_types.iter().find(|t| t.name == ty).unwrap()
    }

    /// Returns a vector of all types inferred from the provided grammar.
    pub(crate) fn symbol_types(grammar: &Grammar) -> Vec<SymbolType> {
        let mut types = vec![];
        for terminal in &grammar.terminals {
            // Each terminal produces `Terminal` kind which maps to String by default
            types.push(SymbolType {
                name: terminal.name.clone(),
                kind: SymbolTypeKind::Terminal,
                optional: false,
            })
        }

        // Each non-terminal produces Enum type
        for nonterminal in &grammar.nonterminals {
            let mut variants = vec![];

            for production in nonterminal.productions(grammar) {
                let variant_name = variant_name(production);

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
                        kind: VariantKind::Plain,
                    },
                    1 if rhs[0].name.is_none() => {
                        let ref_type = grammar.symbol_name(rhs[0].symbol);
                        Variant {
                            name: variant_name,
                            kind: VariantKind::Ref(ref_type),
                        }
                    }
                    _ => {
                        let mut fields = vec![];
                        for assign in &rhs {
                            let ref_type = grammar.symbol_name(assign.symbol);
                            let type_names = grammar.symbol_names(
                                rhs.iter()
                                    .map(|a| a.symbol)
                                    .collect::<Vec<_>>(),
                            );
                            let name = assign.name.clone().unwrap_or(format!(
                                "{}{}",
                                to_snake_case(&ref_type),
                                if type_names
                                    .iter()
                                    .filter(|&ty| *ty == ref_type)
                                    .count()
                                    > 1
                                {
                                    // Not a unique type
                                    format!("_{}", assign.idx + 1)
                                } else {
                                    "".into()
                                }
                            ));
                            fields.push(Field {
                                name: name.clone(),
                                ty: ref_type.clone(),
                                recursive: ref_type == nonterminal.name,
                            })
                        }

                        let struct_type =
                            format!("{}{}", &nonterminal.name, variant_name);
                        Variant {
                            name: variant_name.clone(),
                            kind: VariantKind::Struct(struct_type, fields),
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
            types.push(SymbolType {
                name: nonterminal.name.clone(),
                kind: SymbolTypeKind::Enum(variants),
                optional: type_optional,
            });
        }
        types
    }
}

#[derive(Debug)]
pub(crate) struct SymbolType {
    pub name: String,
    pub kind: SymbolTypeKind,
    pub optional: bool,
}

#[derive(Debug)]
pub(crate) enum SymbolTypeKind {
    Enum(Vec<Variant>),
    Terminal,
}

#[derive(Debug)]
pub(crate) struct Variant {
    pub name: String,
    pub kind: VariantKind,
}

#[derive(Debug)]
pub(crate) enum VariantKind {
    Plain,
    Struct(String, Vec<Field>),
    Ref(String),
}

#[derive(Debug)]
pub(crate) struct Field {
    pub name: String,
    pub ty: String,
    pub recursive: bool,
}
