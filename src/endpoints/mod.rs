pub mod api;
mod index;
mod login;

pub use index::index;
pub use login::login;

use lazy_static::lazy_static;
use option_inspect_none::OptionInspectNone;

const INDEX_HTML: &str = include_str!("index.html");

lazy_static! {
    static ref SPLIT_INDEX_HTML: (&'static str, &'static str) = {
        const CONTENT_STR: &str = "<content />";
        INDEX_HTML
            .split_once(CONTENT_STR)
            .inspect_none(|| log::error!("index.html does not contain '<content />'"))
            .unwrap_or_else(|| panic!("index.html does not contain '<content />'"))
    };
}

pub fn get_index_html_head() -> &'static str {
    SPLIT_INDEX_HTML.0
}

pub fn get_index_html_tail() -> &'static str {
    SPLIT_INDEX_HTML.1
}
