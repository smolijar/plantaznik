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

    warn!("[!]   warn");
    info!("[!]   info");
    debug!("[!]  debug");

    debug!("this is a debug {}", "message");
    error!("this is printed by default");

    error!("[!]  error");
    warn!("[!]   warn");
    info!("[!]   info");
    debug!("[!]  debug");

    let args = cli::run();
    let file_manipulator = file_manipulator::FileManipulator::default();
    let encoder = encoder::SourceEncoder::default();
    let inliner = inliner::Inliner::new(encoder, file_manipulator);

    for entry in glob(&args.glob).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                println!("{:?}", path.display());
                match inliner.inline(&path) {
                    Ok(results) => {
                        for (n, r) in results.iter().enumerate() {
                            match r {
                                Ok(replacement) => {
                                    debug!(
                                        "Replacement {}#{}, LN:{}",
                                        path.display(),
                                        n,
                                        replacement.ln
                                    );
                                    debug!("From: {}", replacement.before);
                                    debug!("To  : {}", replacement.after);
                                }
                                Err(e) => {
                                    error!("Error processing replacement {}: {}", path.display(), e)
                                }
                            }
                        }
                        info!("File {} processed successfully", path.display());
                    }
                    Err(err) => {
                        error!("Error processing file {}: {}", path.display(), err)
                    }
                }
            }
            Err(e) => {
                println!("{:?}", e)
            }
        }
    }
}
