use std::{fs, path::PathBuf};

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

        #[clap(flatten)]
        settle: SettleArgs,
    },
    ConvertBulk {
        #[arg(short, long)]
        input_dir: PathBuf,

        #[arg(short, long)]
        output_dir: PathBuf,

        #[arg(short, long)]
        qkeys: PathBuf,

        #[arg(short, long)]
        tkeys: PathBuf,

        #[arg(long)]
        in_game: nx_common::Game,

        #[arg(long)]
        out_game: nx_common::Game,

        #[clap(flatten)]
        settle: SettleArgs,
    },
}

/// Settling a converted animation tail onto the pose it is cut into.
///
/// Optional: without `--exits` the conversion is unchanged, which is also what any animation
/// with no declared exit gets.
#[derive(Debug, Clone, clap::Args)]
pub struct SettleArgs {
    /// RON manifest of exits, keyed by the bare animation name. See nx_anim::mapping::exits.
    #[arg(long)]
    exits: Option<PathBuf>,

    /// Directory of THPS4 animations to take exit poses from, named as the game ships them
    /// (for example `Idleska.dat`).
    #[arg(long)]
    target_anims: Option<PathBuf>,

    /// Standard keys for reading those target animations: THPS4 ones, not the source game.
    #[arg(long)]
    target_qkeys: Option<PathBuf>,

    #[arg(long)]
    target_tkeys: Option<PathBuf>,

    /// Override every manifest entry blend length, for trying a different ease.
    #[arg(long)]
    blend_frames: Option<u16>,
}

struct Settle {
    exits: nx_anim::Exits,
    ctx: nx_anim::SettleContext,
    blend_frames: Option<u16>,
}

impl Settle {
    fn load(args: &SettleArgs) -> color_eyre::Result<Option<Self>> {
        let Some(exits_path) = &args.exits else {
            return Ok(None);
        };

        let missing = |what: &str| {
            color_eyre::eyre::eyre!("--exits also needs --target-{what} to read exit poses")
        };

        let anims = args.target_anims.as_ref().ok_or_else(|| missing("anims"))?;
        let qkeys = args.target_qkeys.as_ref().ok_or_else(|| missing("qkeys"))?;
        let tkeys = args.target_tkeys.as_ref().ok_or_else(|| missing("tkeys"))?;

        Ok(Some(Self {
            exits: nx_anim::mapping::exits::read_file(exits_path)?,
            ctx: nx_anim::SettleContext::new(anims, qkeys, tkeys)?,
            blend_frames: args.blend_frames,
        }))
    }

    /// The exit for `name`, with any --blend-frames override applied.
    fn spec(&self, name: &str) -> Option<nx_anim::ExitSpec> {
        self.exits.get(name).map(|spec| match self.blend_frames {
            Some(frames) => nx_anim::ExitSpec {
                blend_frames: Some(frames),
                ..spec.clone()
            },
            None => spec.clone(),
        })
    }
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

    nx_anim::Animation::read_file(&input_path, &mut ())?
        .decompress(&qkeys, &tkeys)
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
    settle_args: SettleArgs,
) -> color_eyre::Result<()> {
    let qkeys = nx_stdkey::StdKey::read_file(&qkeys_path, &mut ())?;
    let tkeys = nx_stdkey::StdKey::read_file(&tkeys_path, &mut ())?;
    let settle = Settle::load(&settle_args)?;

    let name = nx_anim::anim_name(&input_path).unwrap_or_default();
    let input = nx_anim::Animation::read_file(&input_path, &mut ())?.to_boned(&qkeys, &tkeys);
    let spec = settle.as_ref().and_then(|s| s.spec(&name));
    let converted = nx_anim::convert(
        &input,
        in_game,
        out_game,
        &name,
        spec.as_ref().zip(settle.as_ref().map(|s| &s.ctx)),
    )?;
    nx_anim::Animation::from_boned(&converted, out_game)
        .write_file(&output_path, &mut WriteContext { game: out_game })?;

    Ok(())
}

fn convert_bulk(
    input_dir: PathBuf,
    output_dir: PathBuf,
    qkeys_path: PathBuf,
    tkeys_path: PathBuf,
    in_game: nx_common::Game,
    out_game: nx_common::Game,
    settle_args: SettleArgs,
) -> color_eyre::Result<()> {
    let qkeys = nx_stdkey::StdKey::read_file(&qkeys_path, &mut ())?;
    let tkeys = nx_stdkey::StdKey::read_file(&tkeys_path, &mut ())?;
    let settle = Settle::load(&settle_args)?;

    fs::create_dir_all(&output_dir)?;

    for entry in input_dir.read_dir()? {
        match entry {
            Ok(entry) => {
                let output_path =
                    nx_anim::convert_path(&output_dir.join(entry.file_name()), in_game, out_game)?;
                println!(
                    "converting {} to {}",
                    entry.path().display(),
                    output_path.display()
                );
                let name = nx_anim::anim_name(&entry.path()).unwrap_or_default();
                let input =
                    nx_anim::Animation::read_file(entry.path(), &mut ())?.to_boned(&qkeys, &tkeys);
                let spec = settle.as_ref().and_then(|s| s.spec(&name));
                let converted = nx_anim::convert(
                    &input,
                    in_game,
                    out_game,
                    &name,
                    spec.as_ref().zip(settle.as_ref().map(|s| &s.ctx)),
                )?;
                nx_anim::Animation::from_boned(&converted, out_game)
                    .write_file(&output_path, &mut WriteContext { game: out_game })?;
            }
            Err(err) => {
                log::error!("error converting anim: {}", err)
            }
        }
    }

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
            settle,
        } => convert(input, output, qkeys, tkeys, in_game, out_game, settle),
        Command::ConvertBulk {
            input_dir,
            output_dir,
            qkeys,
            tkeys,
            in_game,
            out_game,
            settle,
        } => convert_bulk(
            input_dir, output_dir, qkeys, tkeys, in_game, out_game, settle,
        ),
    }
}
