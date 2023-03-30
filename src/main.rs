mod cli;
mod encoder;
mod file_manipulator;
mod inliner;
mod plantaznik;
use std::process;

use clap::Parser;
use cli::Args;
use file_manipulator::FileMode;
use plantaznik::PlantaznikResult;

fn main() {
    let args = Args::parse();

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .format_timestamp(None)
        .format_target(false)
        .init();
    log::debug!("Running with: {args:?}");

    let mode = if args.dry_run || args.check_only {
        FileMode::ReadOnly
    } else {
        FileMode::ReadWrite
    };
    let result = plantaznik::run(&args.glob, mode);
    let status_code = match (result, args.check_only) {
        (PlantaznikResult::Pristine, _) | (PlantaznikResult::Ok, false) => 0,
        _ => 1,
    };
    log::debug!("Exitting with status code: {status_code}");
    process::exit(status_code)
}
