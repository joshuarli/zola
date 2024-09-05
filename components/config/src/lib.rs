mod config;
mod theme;

use std::path::Path;

pub use crate::config::{
    languages::LanguageOptions,
    search::{IndexFormat, Search},
    slugify::Slugify,
    taxonomies::TaxonomyConfig,
    Config,
};
use errors::Result;

/// Get and parse the config.
/// If it doesn't succeed, exit
pub fn get_config(filename: &Path) -> Result<Config> {
    Config::from_file(filename)
}
