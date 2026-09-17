use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

use nx_common::Readable;

use crate::common;

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    Dump {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,
    },
    RoundTrip {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,
    },
    Parse {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,
    },
    Decompile {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,
    },
    // NodeArray {
    //     #[arg(short, long)]
    //     input: PathBuf,
    // },
}

fn dump(input_path: impl AsRef<Path>, output_path: impl AsRef<Path>) -> color_eyre::Result<()> {
    let qb = nx_qb::Qb::read_file(input_path, &mut ())?;
    let contents = ron::ser::to_string_pretty(&qb, ron::ser::PrettyConfig::new())?;
    let output_writer = &mut io::BufWriter::new(fs::File::create(output_path)?);
    output_writer.write_all(contents.as_bytes())?;

    Ok(())
}

fn parse(input_path: impl AsRef<Path>, output_path: impl AsRef<Path>) -> color_eyre::Result<()> {
    let qb = nx_qb::Qb::read_file(input_path, &mut ())?;
    let node = nx_qb::parser::Node::try_from(&qb)?;
    let contents = ron::ser::to_string_pretty(&node, ron::ser::PrettyConfig::new())?;
    let output_writer = &mut io::BufWriter::new(fs::File::create(output_path)?);
    output_writer.write_all(contents.as_bytes())?;

    Ok(())
}

fn decompile(
    input_path: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
) -> color_eyre::Result<()> {
    let qb = nx_qb::Qb::read_file(input_path, &mut ())?;

    let contents = qb.to_string();
    let output_writer = &mut io::BufWriter::new(fs::File::create(output_path)?);
    output_writer.write_all(contents.as_bytes())?;

    Ok(())
}

// fn nodearray(input_path: impl AsRef<Path>) -> color_eyre::Result<()> {
//     let qb = nx_qb::Qb::read_file(input_path, &mut ())?;
//     let node = nx_qb::parser::Node::try_from(&qb)?;
//
//     let nodearray = nx_qb::qb::nodearray::read_nodearray(&node)?;
//
//     Ok(())
// }

pub fn main(command: Command) -> color_eyre::Result<()> {
    match command {
        Command::Dump { input, output } => dump(input, output),
        Command::RoundTrip { input, output } => {
            common::round_trip::<nx_qb::Qb>(input, output, &mut (), &mut ())
        }
        Command::Parse { input, output } => parse(input, output),
        Command::Decompile { input, output } => decompile(input, output),
        // Command::NodeArray { input } => nodearray(input),
    }
}
