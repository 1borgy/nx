use std::path::Path;

use nx_common::Readable;
use nx_skel::Skeleton;
use nx_stdkey::StdKey;

use crate::boned::Animation;
use crate::mapping::exits::{self, ExitSpec};
use crate::mapping::thug_to_thps4::THPS4_SKEL_BYTES;

// Settling a converted animation onto what is showing on the far side of the cut.
/// 0 (dummy_scale_zz) already matches the target, so settling it would be a no-op. 47-49
/// (burnq_board and the two wheels) must be left alone.
fn is_settled(bone: usize) -> bool {
    !matches!(bone, 0 | 47 | 48 | 49)
}

/// Where the target animations live and the keys needed to read them.
pub struct SettleContext {
    pub target_dir: std::path::PathBuf,
    pub qkeys: StdKey,
    pub tkeys: StdKey,
    pub skeleton: nx_skel::boned::Skeleton,
}

impl SettleContext {
    pub fn new(
        target_dir: impl AsRef<Path>,
        qkeys_path: impl AsRef<Path>,
        tkeys_path: impl AsRef<Path>,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            target_dir: target_dir.as_ref().to_owned(),
            qkeys: StdKey::read_file(qkeys_path, &mut ())?,
            tkeys: StdKey::read_file(tkeys_path, &mut ())?,
            skeleton: Skeleton::read(&mut std::io::Cursor::new(THPS4_SKEL_BYTES), &mut ())?
                .to_boned()?,
        })
    }
}

/// Ease the tail of `anim` onto what it gets cut into.
pub fn settle(
    anim: Animation,
    name: &str,
    spec: &ExitSpec,
    ctx: &SettleContext,
) -> Result<Animation, crate::Error> {
    let end = anim.last_key_time();
    let start = end.saturating_sub(spec.blend_frames());

    let targets = exits::target_keys(
        name,
        spec,
        start,
        end,
        &ctx.target_dir,
        &ctx.qkeys,
        &ctx.tkeys,
        &ctx.skeleton,
    )?;

    log::info!(
        "settling {} onto {:?} over {} frames ({}..{}), {} target keys",
        name,
        spec.exit,
        spec.blend_frames(),
        start,
        end,
        targets.first().map(|keys| keys.len()).unwrap_or(0),
    );

    Ok(Animation {
        duration: anim.duration,
        bones: anim
            .bones
            .into_iter()
            .enumerate()
            .map(|(index, bone)| match is_settled(index) {
                true => match targets.get(index) {
                    Some(keys) => bone.settle_to(keys, start, end),
                    None => bone,
                },
                false => bone,
            })
            .collect(),
    })
}
