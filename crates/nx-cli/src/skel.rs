use std::path::{Path, PathBuf};

use nx_common::Readable;

use crate::common;

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    Dump {
        input: PathBuf,
    },
    RoundTrip {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: PathBuf,
    },
    Diff {
        #[arg(short, long)]
        src: PathBuf,

        #[arg(short, long)]
        dst: PathBuf,

        #[arg(long)]
        src_game: nx_common::Game,

        #[arg(long)]
        dst_game: nx_common::Game,
    },
}

fn diff(
    src_path: impl AsRef<Path>,
    dst_path: impl AsRef<Path>,
    src_game: nx_common::Game,
    dst_game: nx_common::Game,
) -> color_eyre::Result<()> {
    let src = nx_skel::Skeleton::read_file(src_path, &mut ())?.to_boned()?;
    let dst = nx_skel::Skeleton::read_file(dst_path, &mut ())?.to_boned()?;

    // log::info!("src={:?}", src);
    // log::info!("dst={:?}", dst);

    let mapping = match (src_game, dst_game) {
        (nx_common::Game::THUG, nx_common::Game::THPS4) => {
            nx_skel::mappings::thug1_to_thps4::MAPPING
        }
        _ => todo!(),
    };

    let thps4_stomach = dst.get_index(2).unwrap();
    log::info!(
        "thps4 stomach: {} | {}",
        thps4_stomach.quaternion,
        thps4_stomach.translation
    );
    let thps4_chest = dst.get_index(3).unwrap();
    log::info!(
        "thps4 chest: {} | {}",
        thps4_chest.quaternion,
        thps4_chest.translation
    );
    let thps4_neck = dst.get_index(4).unwrap();
    log::info!(
        "thps4 neck: {} | {}",
        thps4_neck.quaternion,
        thps4_neck.translation
    );

    // let thug_stomach_lower = src.get_index(2).unwrap();
    // let thug_stomach_upper = src.get_index(3).unwrap();
    // let thug_stomach_chest = src.get_index(4).unwrap();
    // let thug_neck = src.get_index(29).unwrap();

    for (dst_index, src_index) in mapping.iter() {
        let src_bone = match src.get_index(*src_index) {
            Some(bone) => bone,
            None => continue,
        };

        let dst_bone = match dst.get_index(*dst_index) {
            Some(bone) => bone,
            None => continue,
        };

        let quaternion_diff = src_bone.quaternion - dst_bone.quaternion;
        let translation_diff = src_bone.translation - dst_bone.translation;

        log::info!(
            "{}:{} || quat {} || trans {}",
            src_index,
            dst_index,
            quaternion_diff,
            translation_diff,
        );
    }

    Ok(())
}

pub fn main(command: Command) -> color_eyre::Result<()> {
    match command {
        Command::RoundTrip { input, output } => {
            common::round_trip::<nx_skel::Skeleton>(input, output, &mut (), &mut ())
        }
        Command::Dump { input } => common::dump::<nx_skel::Skeleton>(input, &mut ()),
        Command::Diff {
            src,
            dst,
            src_game,
            dst_game,
        } => diff(src, dst, src_game, dst_game),
    }
}
