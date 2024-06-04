use rustemo::rustemo_mod;

rustemo_mod!(calculator01, "/src/ast_actions");
#[allow(dead_code)]
mod calculator01_actions;

rustemo_mod!(calculator02_ambig, "/src/ast_actions");
#[allow(dead_code)]
mod calculator02_ambig_actions;

rustemo_mod!(calculator03_ambig_prodkind, "/src/ast_actions");
#[allow(dead_code)]
mod calculator03_ambig_prodkind_actions;

rustemo_mod!(calculator04_ambig_lhs, "/src/ast_actions");
#[allow(dead_code)]
mod calculator04_ambig_lhs_actions;

#[cfg(test)]
mod tests;
