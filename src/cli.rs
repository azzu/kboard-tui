use crate::config::CliConfig;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "kboard-tui")]
pub struct Cli {
    #[structopt(flatten)]
    pub config: CliConfig,
}

pub fn parse() -> Cli {
    Cli::from_args()
}