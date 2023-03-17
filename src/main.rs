use glob::glob;

use crate::inliner::Inline;
use log::{debug, error, info, warn};

mod cli;
mod encoder;
mod file_manipulator;
mod inliner;

fn main() {
    let args = cli::run();

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .format_timestamp(None)
        .format_target(false)
        .init();

    let args = cli::run();
    let file_manipulator = file_manipulator::FileManipulator::default();
    let encoder = encoder::SourceEncoder::default();
    let inliner = inliner::Inliner::new(encoder, file_manipulator);

    for entry in glob(&args.glob).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let mut ok = 0;
                match inliner.inline(&path) {
                    Ok(results) => {
                        for (ln, r) in results.lines.iter () {
                            let replacement_display =
                                format!("Replacement {}:{}", path.display(), ln);
                            match r {
                                Ok(replacement) => {
                                    ok += 1;
                                    if replacement.before == replacement.after {
                                        debug!("{replacement_display} (no change)");
                                    } else {
                                        debug!("{replacement_display}");
                                        debug!("- {}", replacement.before);
                                        debug!("+ {}", replacement.after);
                                    }
                                }
                                Err(e) => {
                                    warn!("{replacement_display}: {e}")
                                }
                            }
                        }
                        info!(
                            "File {} processed ({}/{} successful replacements)",
                            path.display(),
                            ok,
                            results.lines.len()
                        );
                    }
                    Err(err) => {
                        error!("Error processing file {}: {}", path.display(), err)
                    }
                }
            }
            Err(e) => {
                error!("{:?}", e)
            }
        }
    }
}
