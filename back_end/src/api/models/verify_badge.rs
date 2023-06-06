use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub trait VerifyBadge {
    fn get_errors(&self) -> Option<HashMap<String, String>>;
}

pub fn validate_color(color: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"^#?([a-f0-9]{3}|[a-f0-9]{6})$"#).unwrap();
    }
    RE.is_match(color)
}
