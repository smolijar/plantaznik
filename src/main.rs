mod cli;
mod encoder;
mod file_manipulator;
mod inliner;
mod plantaznik;
use clap::Parser;
use cli::Args;
use file_manipulator::FileMode;

fn main() {
    let args = Args::parse();

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .format_timestamp(None)
        .format_target(false)
        .init();

    let mode = if args.dry_run {
        FileMode::ReadOnly
    } else {
        FileMode::ReadWrite
    };
    plantaznik::run(&args.glob, mode)
}
