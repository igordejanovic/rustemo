use rustemo_rt::rustemo_mod;

rustemo_mod!(calculator01, "/src/calc_actions");
mod calculator01_actions;

rustemo_mod!(calculator02_ambig, "/src/calc_actions");
mod calculator02_ambig_actions;

rustemo_mod!(calculator03_ambig_prodkind, "/src/calc_actions");
mod calculator03_ambig_prodkind_actions;

rustemo_mod!(calculator04_ambig_lhs, "/src/calc_actions");
mod calculator04_ambig_lhs_actions;

#[cfg(test)]
mod tests;
