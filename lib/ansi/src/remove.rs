use ansi_regex::ansi_regex;
use std::borrow::Cow;

pub fn remove(s: &'_ str) -> Cow<'_, str> {
    ansi_regex().replace_all(s, "")
}
