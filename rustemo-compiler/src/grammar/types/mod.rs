//! Inferring types from rustemo grammars.
//! This is a base support for auto AST inference.

use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
};

use crate::index::{SymbolIndex, SymbolVec};
use convert_case::{Boundary, Case, Casing};

use crate::lang::rustemo_actions::Name;

use super::{Grammar, NonTerminal, Production};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub(crate) struct SymbolTypes {
    symbol_types: SymbolVec<SymbolType>,
}

pub(crate) fn to_snake_case<S: AsRef<str>>(s: S) -> String {
    s.as_ref()
        .with_boundaries(&[Boundary::LowerUpper])
        .to_case(Case::Snake)
}

pub(crate) fn to_pascal_case<S: AsRef<str>>(s: S) -> String {
    s.as_ref().to_case(Case::Pascal)
}

pub(crate) fn has_empty_type_name<S: AsRef<str>>(type_name: S) -> String {
    to_pascal_case(format!("{}NoO", type_name.as_ref()))
}

pub(crate) fn choice_name(prod: &Production, ref_type: Option<&String>) -> String {
    if let Some(ref kind) = prod.kind {
        kind.clone()
    } else if let Some(ref_type) = ref_type {
        ref_type.to_owned()
    } else if prod.rhs.is_empty() {
        String::from("Empty")
    } else {
        format!("C{}", prod.ntidx + 1)
    }
}

impl SymbolTypes {
    pub fn new(grammar: &Grammar) -> Self {
        Self {
            symbol_types: Self::symbol_types(grammar, grammar.symbol_name(grammar.start_index)),
        }
    }

    pub(crate) fn get_type(&self, idx: SymbolIndex) -> &SymbolType {
        &self.symbol_types[idx]
    }

    /// Returns a vector of all types inferred from the provided grammar.
    pub(crate) fn symbol_types(grammar: &Grammar, start_symbol: String) -> SymbolVec<SymbolType> {
        let mut types = SymbolVec::new();
        for terminal in &grammar.terminals {
            assert!(types.len() == terminal.idx.into());
            // Each terminal produces `Terminal` kind which maps to String by default
            types.push(SymbolType {
                name: terminal.name.clone(),
                kind: SymbolTypeKind::Terminal,
                choices: vec![],
                optional: false,
            });
        }

        // Each non-terminal produces Enum type
        for nonterminal in &grammar.nonterminals {
            let mut choices = vec![];
            let mut optional = false;

            for production in nonterminal.productions(grammar) {
                // Choices are deduced by the following rules:
                //
                // - No content references (e.g. just string matches) => plain
                //   choice without inner content
                // - A single content. ref and no assig LHS => choice with a
                //   referred NT type as its content
                // - Multiple content. refs => Choice with a new struct type
                //   where fields types are types of the referred symbols.
                let rhs = production.rhs_with_content(grammar);
                choices.push(match rhs.len() {
                    // Empty production
                    0 if production.rhs.is_empty() => {
                        optional = true;
                        Choice {
                            name: choice_name(production, None),
                            kind: ChoiceKind::Empty,
                        }
                    }
                    // A single non-content reference
                    0 if production.rhs.len() == 1 => {
                        let ref_type = grammar.symbol_name(production.rhs_symbols()[0]);
                        Choice {
                            name: choice_name(production, Some(&ref_type)),
                            kind: ChoiceKind::Plain,
                        }
                    }
                    // Multiple non-content references
                    0 => Choice {
                        name: choice_name(production, None),
                        kind: ChoiceKind::Plain,
                    },
                    // A single non-terminal reference
                    1 if rhs[0].name.is_none() => {
                        let ref_type = grammar.symbol_name(rhs[0].symbol);
                        Choice {
                            name: choice_name(production, Some(&ref_type)),
                            kind: ChoiceKind::Ref {
                                ref_type,
                                recursive: Cell::new(false),
                            },
                        }
                    }
                    _ => {
                        let mut fields = vec![];
                        let type_names =
                            grammar.symbol_names(rhs.iter().map(|a| a.symbol).collect::<Vec<_>>());
                        for assign in &rhs {
                            let ref_type = grammar.symbol_name(assign.symbol);
                            let name = assign.name.clone().unwrap_or(Name::new(
                                format!(
                                    "{}{}",
                                    to_snake_case(&ref_type),
                                    if type_names.iter().filter(|&ty| *ty == ref_type).count() > 1
                                    {
                                        // Not a unique rule ref inside this choice
                                        format!("_{}", assign.idx + 1)
                                    } else {
                                        "".into()
                                    }
                                ),
                                None,
                            ));
                            fields.push(Field {
                                name: name.as_ref().clone(),
                                ref_type: ref_type.clone(),
                                recursive: Cell::new(false),
                            })
                        }

                        let choice_name = choice_name(production, None);
                        let struct_type = if production.kind.is_some() {
                            choice_name.clone()
                        } else {
                            format!("{}{}", &nonterminal.name, choice_name)
                        };

                        Choice {
                            name: choice_name,
                            kind: ChoiceKind::Struct {
                                type_name: struct_type,
                                fields,
                            },
                        }
                    }
                });
            }

            Choice::make_choices_name_unique(&mut choices);

            types.push(SymbolType {
                name: nonterminal.name.clone(),
                kind: Self::get_type_kind(nonterminal, &choices),
                choices,
                optional,
            });
        }
        Self::find_recursions(&mut types, start_symbol);
        types
    }

    /// Recognize different rule patters:
    /// ```
    /// A: B | EMPTY ---> A is Option<B>
    /// A: A B | B; or A: A B | B | EMPTY; ---> A is Vec<B>
    /// A: <Whatever> ... | EMPTY; ---> A optional Enum
    /// ```
    fn get_type_kind(nt: &NonTerminal, choices: &Vec<Choice>) -> SymbolTypeKind {
        let type_name = &nt.name;
        struct Match {
            no_match: bool,

            empty: bool,
            single: Option<String>,
            recurse: Option<String>,
        }

        let mut m = Match {
            no_match: false,
            empty: false,
            single: None,
            recurse: None,
        };

        // For zero/one-or-more, and optional pattern recognition
        for choice in choices {
            match &choice.kind {
                ChoiceKind::Empty => m.empty = true,
                ChoiceKind::Struct { fields, .. } => match &fields[..] {
                    [a] => {
                        if m.single.is_none() {
                            m.single = Some(a.ref_type.clone())
                        } else {
                            m.no_match = true
                        }
                    }
                    [a, b] => {
                        if m.recurse.is_none() {
                            if a.ref_type == *type_name && b.ref_type != *type_name {
                                m.recurse = Some(b.ref_type.clone())
                            } else if b.ref_type == *type_name && a.ref_type != *type_name {
                                m.recurse = Some(a.ref_type.clone())
                            } else {
                                m.no_match = true
                            }
                        } else {
                            m.no_match = true
                        }
                    }
                    _ => m.no_match = true,
                },
                ChoiceKind::Ref { ref_type, .. } => m.single = Some(ref_type.clone()),
                ChoiceKind::Plain => m.no_match = true,
            }
        }

        let choices_noe = choices
            .iter()
            .filter(|c| !matches! {c.kind, ChoiceKind::Empty})
            .collect::<Vec<_>>();

        match m {
            // A: A B | B | EMPTY; or
            // A: A B | B;
            Match {
                single: Some(single),
                recurse: Some(recurse),
                no_match: false,
                ..
            } if single == recurse
                && matches! { nt.annotation, Some(ref action) if action == "vec" } =>
            {
                SymbolTypeKind::Vec {
                    ref_type: single,
                    recursive: Cell::new(false),
                }
            }
            Match { empty, .. } => {
                if choices_noe.len() == 1 && !matches! {choices_noe[0].kind, ChoiceKind::Plain} {
                    // Promote
                    match &choices_noe[0].kind {
                        ChoiceKind::Ref { ref_type, .. } => SymbolTypeKind::Ref {
                            ref_type: ref_type.to_string(),
                            recursive: Cell::new(false),
                        },
                        ChoiceKind::Struct { .. } => SymbolTypeKind::Struct {
                            type_name: if empty {
                                has_empty_type_name(type_name)
                            } else {
                                type_name.clone()
                            },
                        },
                        ChoiceKind::Plain | ChoiceKind::Empty => unreachable!(),
                    }
                } else {
                    SymbolTypeKind::Enum {
                        type_name: if empty {
                            has_empty_type_name(type_name)
                        } else {
                            type_name.clone()
                        },
                    }
                }
            }
        }
    }

    /// Flags recursive types by performing a DFS over the types reference graph.
    fn find_recursions(symbol_types: &mut SymbolVec<SymbolType>, start_symbol: String) {
        let types: HashMap<String, &SymbolType> =
            symbol_types.iter().map(|t| (t.name.clone(), t)).collect();
        fn dfs(
            ty: &SymbolType,
            visiting: &mut HashSet<String>,
            types: &HashMap<String, &SymbolType>,
        ) {
            match &ty.kind {
                SymbolTypeKind::Ref {
                    ref recursive,
                    ref_type,
                }
                | SymbolTypeKind::Vec {
                    ref recursive,
                    ref_type,
                } => {
                    if !recursive.get() {
                        if visiting.contains(ref_type) {
                            recursive.set(true);
                            for choice in &ty.choices {
                                if let ChoiceKind::Ref { ref recursive, .. } = choice.kind {
                                    recursive.set(true);
                                }
                            }
                        } else {
                            visiting.insert(ref_type.clone());
                            dfs(types.get(ref_type).unwrap(), visiting, types);
                            visiting.remove(ref_type);
                        }
                    }
                }
                SymbolTypeKind::Struct { .. } | SymbolTypeKind::Enum { .. } => {
                    for choice in &ty.choices {
                        match &choice.kind {
                            ChoiceKind::Ref {
                                ref_type,
                                ref recursive,
                            } => {
                                if !recursive.get() {
                                    if visiting.contains(ref_type) {
                                        recursive.set(true);
                                    } else {
                                        visiting.insert(ref_type.clone());
                                        dfs(types.get(ref_type).unwrap(), visiting, types);
                                        visiting.remove(ref_type);
                                    }
                                }
                            }
                            ChoiceKind::Struct { ref fields, .. } => {
                                for field in fields {
                                    if !field.recursive.get() {
                                        if visiting.contains(&field.ref_type) {
                                            field.recursive.set(true);
                                        } else {
                                            visiting.insert(field.ref_type.clone());
                                            dfs(
                                                types.get(&field.ref_type).unwrap(),
                                                visiting,
                                                types,
                                            );
                                            visiting.remove(&field.ref_type);
                                        }
                                    }
                                }
                            }
                            ChoiceKind::Empty | ChoiceKind::Plain => (),
                        }
                    }
                }
                SymbolTypeKind::Terminal => (),
            }
        }
        log!("Start symbol: {start_symbol:#?}");
        log!("Symbol types: {symbol_types:#?}");

        dfs(
            types.get(&start_symbol).unwrap(),
            &mut HashSet::new(),
            &types,
        );
    }
}

#[derive(Debug)]
pub(crate) struct SymbolType {
    pub name: String,
    pub kind: SymbolTypeKind,
    pub choices: Vec<Choice>,
    pub optional: bool,
}

/// Type kinds derived from grammar rules. Used to auto-generate AST types.
#[derive(Debug)]
pub(crate) enum SymbolTypeKind {
    /// Ref will be a type alias in the generated code.
    ///
    /// Produced by a single choice with plain ref. as in `B: A;`
    ///
    /// Non-content terminal refs are ignored. E.g. `B: 'a' A 'b';` is
    /// equivalent to the previous example.
    ///
    /// Can be optional: `B: A | EMPTY;`
    Ref {
        ref_type: String,
        recursive: Cell<bool>,
    },

    /// Zero or more, one or more patterns. E.g `A: A B | B;`
    Vec {
        ref_type: String,
        recursive: Cell<bool>,
    },

    /// Just a single choice as in `B: A C;`
    /// choices must be a single element of a Struct kind and
    /// optionally element of an Empty kind.
    /// Can be optional as in `B: A C | EMPTY;`
    Struct {
        type_name: String,
    },

    /// The default. All other non-empty rules. Can be optional if
    /// ```
    /// <Whatever>... | EMPTY
    /// ```
    ///
    Enum {
        type_name: String,
    },

    Terminal,
}

#[derive(Debug)]
pub(crate) struct Choice {
    pub name: String,
    pub kind: ChoiceKind,
}

impl Choice {
    /// Ensure that Choice names are unique
    fn make_choices_name_unique(choices: &mut [Choice]) {
        // Choices of a production must have unique names.
        // We need to update same names to include ord indexes.
        let mut name_counts: HashMap<String, usize> = HashMap::new();
        for c in choices.iter() {
            name_counts
                .entry(c.name.clone())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        // Update names to make them unique
        name_counts
            .iter()
            .filter(|&(_, count)| *count > 1)
            .for_each(|(name, _)| {
                choices
                    .iter_mut()
                    .filter(|c| c.name == *name)
                    .enumerate()
                    .for_each(|(idx, c)| {
                        c.name.push_str(&(idx + 1).to_string());
                    });
            });
    }
}

#[derive(Debug)]
pub(crate) enum ChoiceKind {
    /// EMPTY
    Empty,

    /// Just non-content refs. e.g. string match terminals.
    Plain,

    /// Just a single content ref. E.g. `B: A;`
    /// but not `B: a=A;` <- This will be struct.
    Ref {
        ref_type: String,
        recursive: Cell<bool>,
    },

    /// Multiple content refs or named assignments.
    Struct {
        type_name: String,
        fields: Vec<Field>,
    },
}

#[derive(Debug)]
pub(crate) struct Field {
    pub name: String,

    /// Referenced type name.
    pub ref_type: String,

    /// Used to break recursive type references.
    pub recursive: Cell<bool>,
}
