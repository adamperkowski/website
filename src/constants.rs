#[cfg(debug_assertions)]
pub(crate) const HOST: &str = "http://localhost:8000";
#[cfg(not(debug_assertions))]
pub(crate) const HOST: &str = "https://adamperkowski.dev";

pub(crate) const LEGAL_URL: &str =
    "https://github.com/adamperkowski/website/blob/old/templates/pages/legal.tera";
