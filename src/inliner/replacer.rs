use lazy_static::lazy_static;
use regex::Regex;

pub struct Replacer {
    pub encoded_source: String,
}

impl Replacer {
    pub fn new(encoded_source: String) -> Self {
        Replacer { encoded_source }
    }
    pub fn replace_line(&self, _line: &str) -> String {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(.*)").unwrap();
        }
        format!(
            "![](https://www.plantuml.com/plantuml/svg/{})",
            self.encoded_source
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_line_simple() {
        let replacer = Replacer::new("abc".to_string());
        assert_eq!(
            replacer.replace_line(""),
            "![](https://www.plantuml.com/plantuml/svg/abc)"
        )
    }
}
