use lazy_regex::{regex, Lazy, Regex};

pub static DIGITS_DOT: &Lazy<Regex> = regex!("\\d|\\.");

pub static SPACES_TABS: &Lazy<Regex> = regex!("\\s|\\t");
