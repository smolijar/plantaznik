use std::{fs, io, path::Path};

use std::{error::Error, fmt};

#[derive(Debug)]
pub enum FileManipulatorErrorAccess {
    Read,
    Write,
}
#[derive(Debug)]
pub struct FileManipulatorError {
    cause: io::Error,
    path: String,
    access: FileManipulatorErrorAccess,
}
impl Error for FileManipulatorError {}
impl FileManipulatorError {
    pub fn new(cause: io::Error, path: &Path, access: FileManipulatorErrorAccess) -> Self {
        FileManipulatorError {
            cause,
            path: path.display().to_string(),
            access,
        }
    }
}
impl fmt::Display for FileManipulatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error accessing file: {:?} {} (caused by: {})",
            self.access, self.path, self.cause
        )
    }
}

pub trait ManipulateFile {
    fn load(&self, path: &Path) -> Result<String, FileManipulatorError>;
    fn replace_contents(&self, path: &Path, contents: &str) -> Result<(), FileManipulatorError>;
}

#[derive(Default)]
pub struct FileManipulator {}
impl ManipulateFile for FileManipulator {
    fn load(&self, path: &Path) -> Result<String, FileManipulatorError> {
        fs::read_to_string(path)
            .map_err(|e| FileManipulatorError::new(e, path, FileManipulatorErrorAccess::Read))
    }
    fn replace_contents(&self, path: &Path, contents: &str) -> Result<(), FileManipulatorError> {
        fs::write(path, contents)
            .map_err(|e| FileManipulatorError::new(e, path, FileManipulatorErrorAccess::Write))
    }
}
