use rustemo_rt::rustemo_parser;

rustemo_parser!(calculator01, "/src/ast_actions");
mod calculator01_actions;

rustemo_parser!(calculator02_ambig, "/src/ast_actions");
mod calculator02_ambig_actions;

rustemo_parser!(calculator03_ambig_prodkind, "/src/ast_actions");
mod calculator03_ambig_prodkind_actions;

rustemo_parser!(calculator04_ambig_lhs, "/src/ast_actions");
mod calculator04_ambig_lhs_actions;

#[cfg(test)]
mod tests;
