//! Special grammars from the literature on GLR parsing. Crafted specifically to
//! trip GLR parsers.

mod cyclic_1;
mod cyclic_2;
mod knuth_lr1;
mod nondeterministic_palindromes;

mod bounded_ambiguity;
mod bounded_direct_ambiguity;
mod farshi_g7;
mod farshi_g8;
mod highly_ambiguous;
mod reduce_enough_empty;
mod reduce_enough_many_empty;
mod right_nullable;
mod unbounded_ambiguity;
