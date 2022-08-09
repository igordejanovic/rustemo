//! If in bootstrap mode we are loading _bootstrap modules if building build.rs
//! script. If we are building regular code we are common parser files. This is
//! done to enable developing rustemo parser by changing git versioned fiels and
//! seeing diffs.
//!
//! Bootstrap files are checked out from the main branch of the git repo by the
//! build script. Do not forget to remove them when finished with the change.
#[rustfmt::skip]
#[cfg(any(not(bootstrap), not(feature="bootstrap")))]
pub(crate) mod rustemo;
#[cfg(any(not(bootstrap), not(feature="bootstrap")))]
pub(crate) mod rustemo_actions;

#[rustfmt::skip]
#[cfg(all(bootstrap, feature="bootstrap"))]
pub(crate) mod rustemo_bootstrap;
#[cfg(all(bootstrap, feature="bootstrap"))]
pub(crate) mod rustemo_actions_bootstrap;

#[cfg(all(bootstrap, feature="bootstrap"))]
pub(crate) use rustemo_bootstrap as rustemo;
#[cfg(all(bootstrap, feature="bootstrap"))]
pub(crate) use rustemo_actions_bootstrap as rustemo_actions;

#[cfg(test)]
mod tests;
