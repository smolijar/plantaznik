use std::{collections::HashMap, fmt::Debug, path::Path};

use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

mod replacer;

use crate::{
    encoder::{PlantumlEncodingError, SourceEncode},
    file_manipulator::{FileManipulatorError, ManipulateFile},
};

use self::replacer::Replacer;

#[derive(Error, Debug)]
pub enum ReplacementError {
    #[error(transparent)]
    EncodingError(#[from] PlantumlEncodingError),
    #[error(transparent)]
    ReadError(#[from] FileManipulatorError),
}

#[derive(Error, Debug)]
pub enum InliningError {
    #[error("Cannot find parent folder {0}")]
    InvalidParentFolder(String),
    #[error(transparent)]
    ReadError(#[from] FileManipulatorError),
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
    fn inline(&self, path: &Path) -> Result<ReplacementResults, InliningError>;
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
    fn create_replacer(&self, base: &Path, path: &str) -> Result<Replacer, ReplacementError> {
        let src = &self
            .file_manipulator
            .load(&base.join(Path::new(path)))
            .map_err(ReplacementError::ReadError)?;
        let encoded = self
            .encoder
            .encode_source(src)
            .map_err(ReplacementError::EncodingError)?;

        Ok(Replacer::new(encoded))
    }
    fn inline_source(
        &self,
        source: &str,
        path: &Path,
    ) -> Result<(String, ReplacementResults), InliningError> {
        // TODO: Map error
        let base = match path.parent() {
            Some(base) => Ok(base),
            None => Err(InliningError::InvalidParentFolder(
                path.display().to_string(),
            )),
        }?;
        lazy_static! {
            static ref RE: Regex = Regex::new(r"<!--\s*plantaznik:(\S*)\s*-->").unwrap();
        }

        let lines = source.split('\n').enumerate().collect::<Vec<_>>();

        let mut ignore_block = false;
        let matches = lines
            .iter()
            .filter(|(_, line)| {
                if line.starts_with("```") {
                    ignore_block = !ignore_block;
                }
                !ignore_block
            })
            .filter_map(|(n, line)| match RE.captures(line) {
                Some(captures) => {
                    let path = captures.get(1).unwrap().as_str();
                    let img_snippet = self.create_replacer(base, path);
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
            .collect::<HashMap<&usize, &Replacer>>();

        let mut new_contents = lines
            .iter()
            .map(|(n, line)| match matches_h.get(&n) {
                Some(replacer) => replacer.replace_line(line),
                _ => line.to_string(),
            })
            .collect::<Vec<String>>();

        if let Some(replacer) = matches_h.get(&lines.len()) {
            new_contents.push(replacer.replace_line(""))
        }

        let lines = matches
            .into_iter()
            .map(|(ln, result)| {
                (
                    ln,
                    result.map(|_| {
                        let before = lines.get(ln).map(|t| t.1).unwrap_or("").to_string();
                        let after = new_contents.get(ln).unwrap_or(&"".to_string()).to_string();
                        ReplaceLog { after, before }
                    }),
                )
            })
            .collect::<Vec<_>>();
        Ok((
            new_contents.join("\n"),
            ReplacementResults {
                file: path.display().to_string(),
                lines,
            },
        ))
    }
}
impl<E: SourceEncode, L: ManipulateFile> Inline for Inliner<E, L> {
    fn inline(&self, path: &Path) -> Result<ReplacementResults, InliningError> {
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
        fn encode_source(&self, source: &str) -> Result<String, PlantumlEncodingError> {
            Ok(format!("{source}"))
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
            "Hello\n<!-- plantaznik:./foo.plantuml -->\n![](https://www.plantuml.com/plantuml/svg/[./foo.plantuml])\nworld\n<!-- plantaznik:./bar.plantuml -->\n![](https://www.plantuml.com/plantuml/svg/[./bar.plantuml])\nbrrr!"
        );
        assert_eq!(inliner.inline_source("", path).unwrap().0, "");
        assert_eq!(
            inliner
                .inline_source("<!-- plantaznik:./foo.plantuml -->", path)
                .unwrap()
                .0,
            "<!-- plantaznik:./foo.plantuml -->\n![](https://www.plantuml.com/plantuml/svg/[./foo.plantuml])"
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
                    format!("[{}]", path.to_string_lossy())
                })
            }
            fn replace_contents(
                &self,
                _path: &Path,
                contents: &str,
            ) -> Result<(), FileManipulatorError> {
                assert_eq!(
                    contents,
                    "<!-- plantaznik:./e/f/g/foo.plantuml -->\n![](https://www.plantuml.com/plantuml/svg/[a/b/c/d/./e/f/g/foo.plantuml])"
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
    #[test]
    fn test_inline_codeblocks() {
        let inliner = Inliner {
            encoder: MockEncoder::default(),
            file_manipulator: MockFileManipulator::default(),
        };
        let path = Path::new("README.md");
        let md_source = "# Foo\n```sh\n<!--plantaznik:./path-->\n```\n";
        assert_eq!(inliner.inline_source(md_source, path).unwrap().0, md_source);
    }
}
