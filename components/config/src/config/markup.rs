use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ThemeCss {
    /// Which theme are we generating the CSS from
    pub theme: String,
    /// In which file are we going to output the CSS
    pub filename: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Markdown {
    /// Whether external links are to be opened in a new tab
    /// If this is true, a `rel="noopener"` will always automatically be added for security reasons
    pub external_links_target_blank: bool,
    /// Whether to set rel="nofollow" for all external links
    pub external_links_no_follow: bool,
    /// Whether to set rel="noreferrer" for all external links
    pub external_links_no_referrer: bool,
    /// Whether smart punctuation is enabled (changing quotes, dashes, dots etc in their typographic form)
    pub smart_punctuation: bool,
    /// Whether footnotes are rendered at the bottom in the style of GitHub.
    pub bottom_footnotes: bool,
    /// Add loading="lazy" decoding="async" to img tags. When turned on, the alt text must be plain text. Defaults to false
    pub lazy_async_image: bool,
}

impl Markdown {
    pub fn has_external_link_tweaks(&self) -> bool {
        self.external_links_target_blank
            || self.external_links_no_follow
            || self.external_links_no_referrer
    }

    pub fn construct_external_link_tag(&self, url: &str, title: &str) -> String {
        let mut rel_opts = Vec::new();
        let mut target = "".to_owned();
        let title = if title.is_empty() { "".to_owned() } else { format!("title=\"{}\" ", title) };

        if self.external_links_target_blank {
            // Security risk otherwise
            rel_opts.push("noopener");
            target = "target=\"_blank\" ".to_owned();
        }
        if self.external_links_no_follow {
            rel_opts.push("nofollow");
        }
        if self.external_links_no_referrer {
            rel_opts.push("noreferrer");
        }
        let rel = if rel_opts.is_empty() {
            "".to_owned()
        } else {
            format!("rel=\"{}\" ", rel_opts.join(" "))
        };

        format!("<a {}{}{}href=\"{}\">", rel, target, title, url)
    }
}

impl Default for Markdown {
    fn default() -> Markdown {
        Markdown {
            external_links_target_blank: false,
            external_links_no_follow: false,
            external_links_no_referrer: false,
            smart_punctuation: false,
            bottom_footnotes: false,
            lazy_async_image: false,
        }
    }
}
