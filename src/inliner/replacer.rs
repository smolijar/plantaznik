use lazy_static::lazy_static;
use regex::Regex;

pub struct Replacer {
    pub encoded_source: String,
}

impl Replacer {
    pub fn new(encoded_source: String) -> Self {
        Replacer { encoded_source }
    }
    pub fn replace_line(&self, line: &str) -> String {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?P<before>http?s://[a-zA-Z0-9\.]+/plantuml/(uml|svg|png)/)[?~A-Za-z0-9\-_]+"
            )
            .unwrap();
        }
        let after = RE.replace_all(line, "${before}".to_string() + &self.encoded_source);
        if after != line {
            return after.to_string();
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

    #[test]
    fn replace_line_preserve() {
        let replacer = Replacer::new("abc".to_string());
        assert_eq!(
            replacer.replace_line("Yoyoyo <img src=\"https://www.plantuml.com/plantuml/png/old_source\" /> ...and that's not all!"),
            "Yoyoyo <img src=\"https://www.plantuml.com/plantuml/png/abc\" /> ...and that's not all!"
        )
    }

    #[test]
    fn replace_line_preserve_mutliple_link() {
        let replacer = Replacer::new("abc".to_string());
        assert_eq!(
            // TODO: This is invalid
            replacer.replace_line("[![](https://www.plantuml.com/plantuml/svg/foo)](https://www.plantuml.com/plantuml/uml/foo)"),
            "[![](https://www.plantuml.com/plantuml/svg/abc)](https://www.plantuml.com/plantuml/uml/abc)"
        )
    }

    #[test]
    fn replace_line_preserve_other_links() {
        let replacer = Replacer::new("abc".to_string());
        assert_eq!(
            replacer.replace_line("![Diagram](https://www.plantuml.com/plantuml/uml/foo), created in [PlantUML](https://plantuml.com/)"),
            "![Diagram](https://www.plantuml.com/plantuml/uml/abc), created in [PlantUML](https://plantuml.com/)"
        )
    }
}
