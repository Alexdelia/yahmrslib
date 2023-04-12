use ansi_regex::ansi_regex;
use std::borrow::Cow;

pub fn remove<'t>(s: &'t str) -> Cow<'t, str> {
    ansi_regex().replace_all(s, "")
}
