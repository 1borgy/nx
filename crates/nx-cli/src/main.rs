use std::{fs, io::BufReader, path::PathBuf};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    Ske {
        #[arg(short, long)]
        input: PathBuf,
    },
}

fn main() -> color_eyre::Result<()> {
    let App { command } = App::parse();

    env_logger::init();

    match command {
        Command::Ske { input } => {
            let input_file = fs::File::open(input)?;
            let mut input_reader = BufReader::new(input_file);
            nx_ske::Skeleton::read(&mut input_reader)?;
        }
    }

    Ok(())
}
