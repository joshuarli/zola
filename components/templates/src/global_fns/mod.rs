#[macro_use]
mod macros;

mod content;
mod files;
mod helpers;
mod i18n;

pub use self::content::{GetPage, GetSection, GetTaxonomy, GetTaxonomyTerm, GetTaxonomyUrl};
pub use self::files::{GetHash, GetUrl};
pub use self::i18n::Trans;
