//! `query` subcommand

mod eth;

use abscissa_core::{clap::Parser, Command, Runnable};

/// Query state on either ethereum or cosmos chains

#[derive(Command, Debug, Parser, Runnable)]
pub enum QueryCmd {
    #[clap(subcommand)]
    Eth(eth::Eth),
    // Example `--foobar` (with short `-f` argument)
    // #[options(short = "f", help = "foobar path"]
    // foobar: Option<PathBuf>

    // Example `--baz` argument with no short version
    // #[options(no_short, help = "baz path")]
    // baz: Option<PathBuf>

    // "free" arguments don't have an associated flag
    // #[options(free)]
    // free_args: Vec<String>,
}
