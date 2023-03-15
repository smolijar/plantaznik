use std::{
    collections::HashMap,
    fmt::{self, Debug},
    path::Path,
};

use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    encoder::{PlantumlEncodingError, SourceEncode},
    file_manipulator::{FileManipulatorError, ManipulateFile},
};

pub enum ReplacementError {
    EncodingError(PlantumlEncodingError),
    ReadError(FileManipulatorError),
}
// TODO: Maybe better way that this stupido delegate
impl fmt::Display for ReplacementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EncodingError(x) => std::fmt::Display::fmt(x, f),
            Self::ReadError(x) => std::fmt::Display::fmt(x, f),
        }
    }
}

#[derive(Debug)]
pub struct ReplaceLog {
    pub before: String,
    pub after: String,
}

pub struct ReplacementResults {
    pub file: String,
    // line + result
    pub lines: Vec<(usize, Result<ReplaceLog, ReplacementError>)>,
}
pub trait Inline {
    fn inline(&self, path: &Path) -> Result<ReplacementResults, FileManipulatorError>;
}
pub struct Inliner<E: SourceEncode, M: ManipulateFile> {
    encoder: E,
    file_manipulator: M,
}
impl<E: SourceEncode, M: ManipulateFile> Inliner<E, M> {
    pub fn new(encoder: E, file_manipulator: M) -> Inliner<E, M> {
        Inliner {
            encoder,
            file_manipulator,
        }
    }
    fn gen_img(&self, base: &Path, path: &str) -> Result<String, ReplacementError> {
        let src = &self
            .file_manipulator
            .load(&base.join(Path::new(path)))
            .map_err(|x| ReplacementError::ReadError(x))?;
        let img = self
            .encoder
            .src_to_img(&src)
            .map_err(|x| ReplacementError::EncodingError(x))?;
        Ok(img)
    }
    fn inline_source(
        &self,
        source: &str,
        path: &Path,
    ) -> Result<(String, ReplacementResults), FileManipulatorError> {
        // TODO: Map error
        let base = path.parent().expect("Cannot find parent folder");
        lazy_static! {
            static ref RE: Regex = Regex::new(r"<!--\s*plantaznik:(\S*)\s*-->").unwrap();
        }

        let lines = source.split('\n').enumerate().collect::<Vec<_>>();

        let matches = lines
            .iter()
            .filter_map(|(n, line)| match RE.captures(line) {
                Some(captures) => {
                    let path = captures.get(1).unwrap().as_str();
                    let img_snippet = self.gen_img(base, path);
                    Some(img_snippet).map(|i| (n + 1, i))
                }
                None => None,
            })
            .collect::<Vec<(_, Result<_, _>)>>();

        let matches_h = matches
            .iter()
            .filter_map(|(ln, r)| match r {
                Ok(r) => Some((ln, r)),
                _ => None,
            })
            .collect::<HashMap<&usize, &String>>();

        let mut contents = lines
            .iter()
            .map(|(n, line)| match matches_h.get(&n) {
                Some(replacement) => replacement,
                _ => *line,
            })
            .collect::<Vec<&str>>()
            .join("\n");

        if let Some(replacement) = matches_h.get(&lines.len()) {
            contents += "\n";
            contents += replacement
        }

        let lines = matches
            .into_iter()
            .map(|(ln, result)| {
                (
                    ln,
                    result.map(|after| ReplaceLog {
                        after: after.to_string(),
                        before: lines.get(ln).map(|t| t.1).unwrap_or("").to_string(),
                    }),
                )
            })
            .collect::<Vec<_>>();
        Ok((
            contents,
            ReplacementResults {
                file: path.display().to_string(),
                lines,
            },
        ))
    }
}
impl<E: SourceEncode, L: ManipulateFile> Inline for Inliner<E, L> {
    fn inline(&self, path: &Path) -> Result<ReplacementResults, FileManipulatorError> {
        let path = Path::new(path);
        let src = self.file_manipulator.load(path)?;
        let (contents, results) = self.inline_source(&src, path)?;
        self.file_manipulator.replace_contents(path, &contents)?;
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use crate::{encoder::PlantumlEncodingError, file_manipulator::FileManipulatorError};

    use super::*;

    #[derive(Default)]
    struct MockEncoder {}
    impl SourceEncode for MockEncoder {
        fn src_to_img(&self, source: &str) -> Result<String, PlantumlEncodingError> {
            Ok(format!("<{source}>"))
        }
    }

    #[derive(Default)]
    struct MockFileManipulator {}
    impl ManipulateFile for MockFileManipulator {
        fn load(&self, path: &Path) -> Result<std::string::String, FileManipulatorError> {
            Ok(format!("[{}]", path.display()))
        }
        fn replace_contents(
            &self,
            _path: &Path,
            _contents: &str,
        ) -> Result<(), FileManipulatorError> {
            Ok(())
        }
    }

    #[test]
    fn test_inline() {
        let inliner = Inliner {
            encoder: MockEncoder::default(),
            file_manipulator: MockFileManipulator::default(),
        };
        let path = Path::new("README.md");
        assert_eq!(
            inliner.inline_source("Hello\n<!-- plantaznik:./foo.plantuml -->\n![](FOO)\nworld\n<!-- plantaznik:./bar.plantuml -->\n![](BAR)\nbrrr!", path).unwrap().0,
            "Hello\n<!-- plantaznik:./foo.plantuml -->\n<[./foo.plantuml]>\nworld\n<!-- plantaznik:./bar.plantuml -->\n<[./bar.plantuml]>\nbrrr!"
        );
        assert_eq!(inliner.inline_source("", path).unwrap().0, "");
        assert_eq!(
            inliner
                .inline_source("<!-- plantaznik:./foo.plantuml -->", path)
                .unwrap()
                .0,
            "<!-- plantaznik:./foo.plantuml -->\n<[./foo.plantuml]>"
        );
    }
    #[test]
    fn test_inline_nested_relative() {
        #[derive(Default)]
        struct MockFileManipulator {}
        impl ManipulateFile for MockFileManipulator {
            fn load(&self, path: &Path) -> Result<std::string::String, FileManipulatorError> {
                Ok(if path.ends_with("README.md") {
                    "<!-- plantaznik:./e/f/g/foo.plantuml -->".to_string()
                } else {
                    format!("[{}]", path.to_string_lossy().to_string())
                })
            }
            fn replace_contents(
                &self,
                _path: &Path,
                contents: &str,
            ) -> Result<(), FileManipulatorError> {
                assert_eq!(
                    contents,
                    "<!-- plantaznik:./e/f/g/foo.plantuml -->\n<[a/b/c/d/./e/f/g/foo.plantuml]>"
                );
                Ok(())
            }
        }
        let inliner = Inliner {
            encoder: MockEncoder::default(),
            file_manipulator: MockFileManipulator::default(),
        };
        inliner.inline(Path::new("a/b/c/d/README.md")).unwrap();
    }
}
