pub(crate) const GITHUB_SECRET_VAR: &str = "GITHUB_SECRET";

#[cfg(debug_assertions)]
pub(crate) const HOST: &str = "http://localhost:8000";
#[cfg(not(debug_assertions))]
pub(crate) const HOST: &str = "https://adamperkowski.dev";

pub(crate) const OLD_HOST: &str = "https://old.adamperkowski.dev";
