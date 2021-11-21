use super::types::{Imports, ProductionRules, TerminalRules, PGFile};

#[derive(Debug)]
pub (in crate::lang) struct Grammar {
    imports: Option<Imports>,
    rules: Option<ProductionRules>,
    terminals: Option<TerminalRules>,
    // nonterminals: Vec<NonTerminalRules>,
    // symbol_by_name: HashMap<String, &'a Symbol<'a>>,
    // first_set: HashMap<NonTerminal<'a>, HashSet<&'a Terminal>>,
    // start_symbol: Option<&'a NonTerminal<'a>>,
}

impl Grammar {
    pub fn from_pgfile(pgfile: PGFile) -> Self {
        // 1. TODO: Terminal/non-terminal indexes, symbol index? Maybe symbol
        // index could be <max terminal index> + non-term index. Thus I can have
        // symbol index of each term/non-term when needed. Do I need symbol
        // index? Yes from RHS of productions. SymbolIndex can be a new type to
        // allow conversion to TerminalIndex/NonTerminalIndex.
        //
        // 2. TODO: Extract productions from rules. Production should have LHS
        // which is a SymbolIndex. Meta should be Production Meta from the
        // parse-tree with rule meta applied (meta inheritance). RHS should be
        // Production assignments from the parse tree.
        //
        // 3. TODO: Desugaring. Related to the previous. Desugar repetitions and
        // groups.

        Grammar {
            imports: pgfile.imports,
            rules: pgfile.rules,
            terminals: pgfile.terminals
        }
    }
}

// impl<'a> Grammar<'a> {
//     fn new(
//         productions: Vec<Production>,
//         terminals: Vec<Terminal>,
//         nonterminals: Vec<NonTerminal<'a>>,
//         start_symbol: Option<&'a NonTerminal<'a>>,
//     ) -> Grammar<'a> {
//         return Grammar {
//         //    productions,
//             terminals,
//             nonterminals,
//             first_set: HashMap::new(),
//             start_symbol,
//         };
//     }

//     fn add_terminal(&'a mut self, fqn: &str) -> &mut Self {
//         let t = Terminal {
//             name: fqn.split('.').last().unwrap().to_string(),
//             fqn: fqn.to_string(),
//             ..Terminal::default()
//         };
//         self.terminals.push(t);
//         self.symbol_by_name[fqn] = &Symbol::Terminal(t);
//         self
//     }

//     fn add_nonterminal(&'a mut self, fqn: String) -> &mut Self {
//         self.nonterminals.push(NonTerminal {
//             name: fqn.split('.').last().unwrap().to_string(),
//             fqn,
//             ..NonTerminal::default()
//         });
//         self
//     }

//     fn add_production(&'a mut self, nonterm_fqn: &str, rhs_names: &[&str]) -> &mut Self {
//         let rhs: Vec<&Symbol> = Vec::new();
//         for symbol_ref in rhs_names {
//             rhs.push(self.symbol_by_name(symbol_ref))
//         }
//        self
//     }

//     fn symbol_by_name(&self, name: &str) -> &Symbol {
//         self.symbol_by_name[name]
//     }

//     /// Calculate and update grammar first sets.
//     ///
//     /// The Dragon book, p 245.
//     ///
//     /// Define $FIRST(\alpha)$ where $\alpha$ is any string of grammar
//     /// symbols, to be the set of terminals that begin strings derived from
//     /// $\alpha$. If $\alpha \overset{*}{\Rightarrow} \epsilon$ then
//     /// $\epsilon$ is also in $FIRST(\alpha)$.
//     fn first(&mut self) -> () {
//         // 1. Initialize firsts set for every terminal to a set with
//         //    the terminal being its sole member.
//         // 2. Initialize firsts set for every non-terminal to an empty set.
//         // 3.
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::grammar::Grammar;

//     #[test]
//     fn test_create_grammar() {
//         let grammar = Grammar::new(vec![], vec![], vec![], None);
//     }

//     #[test]
//     fn test_grammar_first_sets() {
//         let grammar = Grammar::new(vec![Production::new()], vec![], vec![], None);

//         grammar.first_set();
//     }

//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
