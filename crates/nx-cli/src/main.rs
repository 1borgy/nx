use std::fmt::Debug;

use clap::Parser;
use nx_bone::{Quaternion, Translation};

mod anim;
mod common;
mod crc;
mod qb;
mod skel;
mod stdkey;

#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    Skel {
        #[clap(subcommand)]
        command: skel::Command,
    },
    Anim {
        #[clap(subcommand)]
        command: anim::Command,
    },
    Crc {
        #[clap(subcommand)]
        command: crc::Command,
    },
    StdKey {
        #[clap(subcommand)]
        command: stdkey::Command,
    },
    Qb {
        #[clap(subcommand)]
        command: qb::Command,
    },
    Test,
}

fn main() -> color_eyre::Result<()> {
    let App { command } = App::parse();

    env_logger::init();

    match command {
        Command::Skel { command } => skel::main(command),
        Command::Anim { command } => anim::main(command),
        Command::StdKey { command } => stdkey::main(command),
        Command::Qb { command } => qb::main(command),
        Command::Test => {
            nx_anim::mapping::thug_to_thps4_test()?;
            // println!("trans={:?} rot={:?} rotated={:?}", trans, rot, rotated);
            Ok(())
        }
        Command::Crc { command } => crc::main(command),
    }
}
