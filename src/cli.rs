use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// README files pattern to process
    #[arg()]
    pub glob: String,

    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}

pub fn run() -> Args {
    Args::parse()
}
