#![allow(unused_imports)]
#[cfg(test)]
mod ambiguity;
#[cfg(test)]
mod layout;
#[cfg(test)]
mod partial;
#[cfg(test)]
mod pass_context;
#[cfg(test)]
mod rule_patterns;
#[cfg(test)]
mod sugar;

// For output_cmp macro
#[cfg(test)]
pub use rustemo::utils::string_difference;
