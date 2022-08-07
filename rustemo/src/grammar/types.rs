//! Inferring types from rustemo grammars.
//! This is a base support for auto AST inferrence.

use rustemo_rt::index::SymbolIndex;

use super::Grammar;

/// Returns a vector of all types inferred from the provided grammar.
pub(crate) fn symbol_types(grammar: &Grammar) -> Vec<SymbolType> {
    // 1. NT has empty production if type is optional
    // 2. By default, NT maps to enum type. Each production is a variant, except EMPTY.
    // 3. Enum variants are deduced by the following rules:
    //    - No content references => plain variant witout inner content
    //    - A single content. ref => variant with a referred NT type as its content
    //    - Multiple content. refs => Variant with a new struct type where fields types
    //      are types of the referred symbols.
    // 4. If NT enum has only one variant:
    //    - If a plain variant => NT type is completely removed?
    //    - If a ref. to another symbol => type alias to contained symbol type
    //    - If variant contains a new struct => NT type becomes inner struct type
    // 5. Each terminal produces `Terminal` kind which maps to String by default
    // 6. If enum ref is NT or any inner struct Field is NT -> Recursive

    let mut types = vec![];
    for terminal in grammar.terminals() {
        types.push(SymbolType {
            name: terminal.name.clone(),
            symbol: grammar.term_to_symbol_index(terminal.idx),
            kind: SymbolTypeKind::Terminal,
            optional: false,
        })
    }

    for nonterminal in grammar.nonterminals() {

    }
    types
}

pub(crate) struct SymbolType {
    name: String,
    symbol: SymbolIndex,
    kind: SymbolTypeKind,
    optional: bool,
}

pub(crate) enum SymbolTypeKind {
    Enum(Vec<Variant>),
    Struct(Vec<Field>),
    Terminal,
    Ref(Box<SymbolType>),
}

pub(crate) struct Variant {
    name: String,
    ty: String,
    recursive: bool,
}
pub(crate) type Field = Variant;
