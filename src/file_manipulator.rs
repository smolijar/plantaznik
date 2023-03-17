use std::{fs, io, path::Path};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileManipulatorError {
    #[error("Error reading file {1:?} (caused by: {0})")]
    ReadError(io::Error, String),
    #[error("Error writing file {1:?} (caused by: {0})")]
    WriteError(io::Error, String),
}

pub trait ManipulateFile {
    fn load(&self, path: &Path) -> Result<String, FileManipulatorError>;
    fn replace_contents(&self, path: &Path, contents: &str) -> Result<(), FileManipulatorError>;
}

pub enum FileMode {
    ReadOnly,
    ReadWrite,
}
pub struct FileManipulator {
    mode: FileMode,
}
impl FileManipulator {
    pub fn new(mode: FileMode) -> FileManipulator {
        FileManipulator { mode }
    }
}
impl ManipulateFile for FileManipulator {
    fn load(&self, path: &Path) -> Result<String, FileManipulatorError> {
        fs::read_to_string(path)
            .map_err(|e| FileManipulatorError::ReadError(e, path.display().to_string()))
    }
    fn replace_contents(&self, path: &Path, contents: &str) -> Result<(), FileManipulatorError> {
        match self.mode {
            FileMode::ReadOnly => {
                log::trace!("Skipping file write due to readonly mode");
                Ok(())
            }
            FileMode::ReadWrite => {
                log::trace!("Writing file contents {}", path.display());
                fs::write(path, contents)
                    .map_err(|e| FileManipulatorError::WriteError(e, path.display().to_string()))
            }
        }
    }
}
