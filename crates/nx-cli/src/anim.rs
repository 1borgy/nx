use std::path::PathBuf;

use nx_anim::WriteContext;
use nx_common::{Game, Readable, Writable};

use crate::common;

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    RoundTrip {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,

        #[arg(long)]
        out_game: Game,
    },
    Decompress {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,

        #[arg(long)]
        out_game: Game,

        #[arg(short, long)]
        qkeys: PathBuf,

        #[arg(short, long)]
        tkeys: PathBuf,
    },
    Dump {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        qkeys: PathBuf,

        #[arg(short, long)]
        tkeys: PathBuf,
    },
    Convert {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,

        #[arg(short, long)]
        qkeys: PathBuf,

        #[arg(short, long)]
        tkeys: PathBuf,

        #[arg(long)]
        in_game: nx_common::Game,

        #[arg(long)]
        out_game: nx_common::Game,
    },
}

fn decompress_anim(
    input_path: PathBuf,
    output_path: PathBuf,
    out_game: Game,
    qkeys_path: PathBuf,
    tkeys_path: PathBuf,
) -> color_eyre::Result<()> {
    let qkeys = nx_stdkey::StdKey::read_file(&qkeys_path, &mut ())?;
    let tkeys = nx_stdkey::StdKey::read_file(&tkeys_path, &mut ())?;

    let boned = nx_anim::Animation::read_file(&input_path, &mut ())?.to_boned(&qkeys, &tkeys);
    nx_anim::Animation::from_boned(&boned, out_game)
        .write_file(&output_path, &mut nx_anim::WriteContext { game: out_game })?;

    Ok(())
}

fn dump_bones(
    input_path: PathBuf,
    qkeys_path: PathBuf,
    tkeys_path: PathBuf,
) -> color_eyre::Result<()> {
    let qkeys = nx_stdkey::StdKey::read_file(&qkeys_path, &mut ())?;
    let tkeys = nx_stdkey::StdKey::read_file(&tkeys_path, &mut ())?;

    let bones = nx_anim::Animation::read_file(&input_path, &mut ())?.to_boned(&qkeys, &tkeys);
    log::info!("bones={:#?}", bones);

    Ok(())
}

fn convert(
    input_path: PathBuf,
    output_path: PathBuf,
    qkeys_path: PathBuf,
    tkeys_path: PathBuf,
    in_game: nx_common::Game,
    out_game: nx_common::Game,
) -> color_eyre::Result<()> {
    let qkeys = nx_stdkey::StdKey::read_file(&qkeys_path, &mut ())?;
    let tkeys = nx_stdkey::StdKey::read_file(&tkeys_path, &mut ())?;

    let input = nx_anim::Animation::read_file(&input_path, &mut ())?.to_boned(&qkeys, &tkeys);
    let converted = nx_anim::convert(&input, in_game, out_game)?;
    nx_anim::Animation::from_boned(&converted, out_game)
        .write_file(&output_path, &mut WriteContext { game: out_game })?;

    Ok(())
}

pub fn main(command: Command) -> color_eyre::Result<()> {
    match command {
        Command::RoundTrip {
            input,
            output,
            out_game,
        } => common::round_trip::<nx_anim::Animation>(
            input,
            output,
            &mut (),
            &mut nx_anim::WriteContext { game: out_game },
        ),
        Command::Decompress {
            input,
            output,
            out_game,
            qkeys,
            tkeys,
        } => decompress_anim(input, output, out_game, qkeys, tkeys),
        Command::Dump {
            input,
            qkeys,
            tkeys,
        } => dump_bones(input, qkeys, tkeys),
        Command::Convert {
            input,
            output,
            qkeys,
            tkeys,
            in_game,
            out_game,
        } => convert(input, output, qkeys, tkeys, in_game, out_game),
    }
}
