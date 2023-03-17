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

    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}
