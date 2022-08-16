//! Inferring types from rustemo grammars.
//! This is a base support for auto AST inference.

use convert_case::{Boundary, Case, Casing};

use super::{Grammar, NonTerminal, Production};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub(crate) struct SymbolTypes {
    symbol_types: Vec<SymbolType>,
}

pub(crate) fn to_snake_case<S: AsRef<str>>(s: S) -> String {
    s.as_ref()
        .with_boundaries(&[Boundary::LowerUpper])
        .to_case(Case::Snake)
}

pub(crate) fn to_pascal_case<S: AsRef<str>>(s: S) -> String {
    s.as_ref().to_case(Case::Pascal)
}

pub(crate) fn choice_name(prod: &Production) -> String {
    if let Some(ref kind) = prod.kind {
        kind.clone()
    } else if prod.rhs.len() == 0 {
        String::from("Empty")
    } else {
        format!("C{}", prod.ntidx + 1)
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
            })
        }

        // Each non-terminal produces Enum type
        for nonterminal in &grammar.nonterminals {
            let mut choices = vec![];

            for production in nonterminal.productions(grammar) {
                let choice_name = choice_name(production);

                // Choices are deduced by the following rules:
                // - No content references => plain choice without inner content
                // - A single content. ref and no assig LHS => choice with
                //   a referred NT type as its content
                // - Multiple content. refs => Choice with a new struct type
                //   where fields types are types of the referred symbols.
                let rhs = production.rhs_with_content(grammar);
                choices.push(match rhs.iter().count() {
                    0 if production.rhs.len() == 0 => Choice {
                        name: choice_name,
                        kind: ChoiceKind::Empty,
                    },
                    0 => Choice {
                        name: choice_name,
                        kind: ChoiceKind::Plain,
                    },
                    1 if rhs[0].name.is_none() => {
                        let ref_type = grammar.symbol_name(rhs[0].symbol);
                        Choice {
                            name: choice_name,
                            kind: ChoiceKind::Ref(ref_type),
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
                                    // Not a unique rule ref inside this choice
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

                        let struct_type = if production.kind.is_some() {
                            choice_name.clone()
                        } else {
                            format!("{}{}", &nonterminal.name, choice_name)
                        };

                        Choice {
                            name: choice_name.clone(),
                            kind: ChoiceKind::Struct(struct_type, fields),
                        }
                    }
                });
            }

            types.push(SymbolType {
                name: nonterminal.name.clone(),
                kind: Self::get_type_kind(&nonterminal, choices),
            });
        }
        types
    }

    /// Recognize different rule patters:
    /// A: B | EMPTY ---> A is Option<B>
    /// A: A B | B; or A: A B | B | EMPTY; ---> A is Vec<B>
    /// A: <Whatever> ... | EMPTY; ---> A optional Enum
    fn get_type_kind(
        nt: &NonTerminal,
        choices: Vec<Choice>,
    ) -> SymbolTypeKind {
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

        // For regex-like op. patter recognition
        for choice in &choices {
            match &choice.kind {
                ChoiceKind::Empty => m.empty = true,
                ChoiceKind::Struct(_, fields) => match &fields[..] {
                    [a] => {
                        if m.single.is_none() {
                            m.single = Some(a.ty.clone())
                        } else {
                            m.no_match = true
                        }
                    }
                    [a, b] => {
                        if m.recurse.is_none() {
                            if a.ty == *type_name && b.ty != *type_name {
                                m.recurse = Some(b.ty.clone())
                            } else if b.ty == *type_name && a.ty != *type_name {
                                m.recurse = Some(a.ty.clone())
                            } else {
                                m.no_match = true
                            }
                        } else {
                            m.no_match = true
                        }
                    }
                    _ => m.no_match = true,
                },
                ChoiceKind::Ref(ref_type) => m.single = Some(ref_type.clone()),
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
                empty,
                ..
            } if single == recurse
                && matches! { nt.action, Some(ref action) if action == "vec" } =>
            {
                SymbolTypeKind::Vec {
                    name: single,
                    choices,
                    optional: empty,
                }
            }
            Match { empty, .. } => {
                if choices_noe.len() == 1
                    && !matches! {choices_noe[0].kind, ChoiceKind::Plain}
                {
                    // Promote
                    match &choices_noe[0].kind {
                        ChoiceKind::Ref(ref_type) => SymbolTypeKind::Ref {
                            name: ref_type.to_string(),
                            choices,
                            optional: empty,
                        },
                        ChoiceKind::Struct(_, _) => SymbolTypeKind::Struct {
                            name: type_name.clone(),
                            choices,
                            optional: empty,
                        },
                        ChoiceKind::Plain | ChoiceKind::Empty => unreachable!(),
                    }
                } else {
                    SymbolTypeKind::Enum {
                        name: type_name.clone(),
                        choices,
                        optional: empty,
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct SymbolType {
    pub name: String,
    pub kind: SymbolTypeKind,
}

#[derive(Debug)]
pub(crate) enum SymbolTypeKind {
    /// Just a single choice with plain ref. as in "B: A;"
    /// This will be type alias.
    /// Can be optional: B: A | EMPTY;
    Ref {
        name: String,
        choices: Vec<Choice>,
        optional: bool,
    },

    /// Zero or more, one or more patterns
    Vec {
        name: String,
        choices: Vec<Choice>,
        optional: bool,
    },

    /// Just a single choice as in "B: A C;"
    /// choices must be a single element of Struct kind and
    /// optionally element of Empty kind.
    /// Can be optional as in "B: A C | EMPTY;"
    Struct {
        name: String,
        choices: Vec<Choice>,
        optional: bool,
    },

    /// All other non-empty rules. Can be optional if
    /// <Whatever>... | EMPTY
    Enum {
        name: String,
        choices: Vec<Choice>,
        optional: bool,
    },

    Terminal,
}

#[derive(Debug)]
pub(crate) struct Choice {
    pub name: String,
    pub kind: ChoiceKind,
}

#[derive(Debug)]
pub(crate) enum ChoiceKind {
    /// EMPTY
    Empty,

    /// Just non-content refs. e.g. string match terminals.
    Plain,

    /// Just a single content ref. E.g. B: A;
    /// but not B: a=A; <- This will be struct.
    Ref(String),

    /// Multiple content refs or named assignments.
    Struct(String, Vec<Field>),
}

#[derive(Debug)]
pub(crate) struct Field {
    pub name: String,

    /// Referenced type name.
    pub ty: String,

    /// Used to break recursive type references. Currently only direct recursion
    /// is detected but in the future versions indirect will be detected too.
    pub recursive: bool,
}
