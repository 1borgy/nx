use std::path::PathBuf;

use crate::common;

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    Dump {
        #[arg(short, long)]
        input: PathBuf,
    },
    RoundTrip {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,
    },
}

pub fn main(command: Command) -> color_eyre::Result<()> {
    match command {
        Command::RoundTrip { input, output } => {
            common::round_trip::<nx_stdkey::StdKey>(input, output, &mut (), &mut ())
        }
        Command::Dump { input } => common::dump::<nx_stdkey::StdKey>(input, &mut ()),
    }
}
