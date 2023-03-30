use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// README files glob pattern to process
    #[arg()]
    pub glob: String,

    /// Dry run (skips file writes)
    #[arg(long, short = 'd', default_value_t = false)]
    pub dry_run: bool,

    /// Check only (skips file writes and error if updates would take place)
    #[arg(long, short = 'c', default_value_t = false)]
    pub check_only: bool,

    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}
