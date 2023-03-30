use glob::glob;

use crate::{
    encoder,
    file_manipulator::{self, FileMode},
    inliner::{self, Inline},
};
use log::{debug, error, info};

#[derive(Debug)]
pub enum PlantaznikResult {
    /// No errors and no changes
    Pristine,
    /// No errors, changes
    Ok,
    /// Errors occured, changes might have taken place
    Errors,
}

pub fn run(pattern: &str, mode: FileMode) -> PlantaznikResult {
    let file_manipulator = file_manipulator::FileManipulator::new(mode);
    let encoder = encoder::SourceEncoder::default();
    let inliner = inliner::Inliner::new(encoder, file_manipulator);

    let glob = glob(pattern);
    if glob.is_err() {
        error!("Failed to read glob pattern: {pattern}");
        return PlantaznikResult::Errors;
    }

    let mut changes = 0;
    let mut errors = 0;

    for entry in glob.unwrap() {
        match entry {
            Ok(path) => {
                let mut ok_current_file_replacements = 0;
                match inliner.inline(&path) {
                    Ok(results) => {
                        for (ln, r) in results.lines.iter() {
                            let replacement_display =
                                format!("Replacement {}:{}", path.display(), ln);
                            match r {
                                Ok(replacement) => {
                                    ok_current_file_replacements += 1;
                                    if replacement.before == replacement.after {
                                        debug!("{replacement_display} (no change)");
                                    } else {
                                        changes += 1;
                                        debug!("{replacement_display}");
                                        debug!("- {}", replacement.before);
                                        debug!("+ {}", replacement.after);
                                    }
                                }
                                Err(e) => {
                                    errors += 1;
                                    error!("{replacement_display}: {e}")
                                }
                            }
                        }
                        info!(
                            "File {} processed ({}/{} successful replacements)",
                            path.display(),
                            ok_current_file_replacements,
                            results.lines.len()
                        );
                    }
                    Err(err) => {
                        errors += 1;
                        error!("Error processing file {}: {}", path.display(), err)
                    }
                }
            }
            Err(e) => {
                errors += 1;
                error!("Error reading glob file: {e:?}")
            }
        }
    }

    match (changes, errors) {
        (0, 0) => PlantaznikResult::Pristine,
        (_, 0) => PlantaznikResult::Ok,
        _ => PlantaznikResult::Errors,
    }
}
