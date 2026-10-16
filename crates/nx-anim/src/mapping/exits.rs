use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use nx_common::Readable;

use crate::boned::TargetKey;

/// How many frames the ease takes, when an entry does not say.
pub const DEFAULT_BLEND_FRAMES: u16 = 25;

/// Where a balance animation opens.
pub const BALANCE_START_FRAME: u16 = 33;

/// How to follow the animation being cut to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Follow {
    /// Settle onto the animation's motion, not just its opening pose.
    Motion,

    /// Settle onto one frozen frame.
    Pose,
}

/// The pose on the far side of the cut.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Exit {
    /// Cuts to IDLE.
    Ground,
    /// Cuts to one of the crouch idles or turns.
    Crouch,
    /// Cuts to a manual balance animation at `BALANCE_START_FRAME`.
    Manual,
    /// Cuts to a nose manual balance animation at `BALANCE_START_FRAME`.
    NoseManual,
    /// Anything not yet covered by a preset.
    Anim {
        anim: String,
        frame: u16,
        mirror: bool,
        #[serde(default)]
        motion: bool,
    },
}

struct Resolved<'a> {
    anim: &'a str,
    frame: u16,
    mirror: bool,
    follow: Follow,
}

impl Exit {
    fn resolve(&self) -> Resolved<'_> {
        let (anim, frame, mirror, follow) = match self {
            Self::Ground => ("Idle", 0, true, Follow::Motion),
            Self::Crouch => ("CrouchIdle", 0, true, Follow::Motion),
            Self::Manual => ("Manual_range", BALANCE_START_FRAME, true, Follow::Pose),
            Self::NoseManual => ("NoseManual_range", BALANCE_START_FRAME, true, Follow::Pose),
            Self::Anim {
                anim,
                frame,
                mirror,
                motion,
            } => (
                anim.as_str(),
                *frame,
                *mirror,
                match motion {
                    true => Follow::Motion,
                    false => Follow::Pose,
                },
            ),
        };

        Resolved {
            anim,
            frame,
            mirror,
            follow,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExitSpec {
    pub exit: Exit,

    /// Frames of ease. Short animations need a shorter tail: ten frames is a sixth of a second
    /// out of a 74-frame wallpush, but a third of a 30-frame flip.
    #[serde(default)]
    pub blend_frames: Option<u16>,
}

impl ExitSpec {
    pub fn blend_frames(&self) -> u16 {
        self.blend_frames.unwrap_or(DEFAULT_BLEND_FRAMES)
    }
}

/// Keyed by the animation's bare name, e.g. `Wallpush_Standing`.
pub type Exits = BTreeMap<String, ExitSpec>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error reading {0}: {1}")]
    Io(PathBuf, std::io::ErrorKind),

    #[error("could not parse exit manifest {0}: {1}")]
    Parse(PathBuf, String),

    #[error("exit for {0} names animation {1}, which is not in the target directory")]
    MissingTarget(String, String),
}

pub fn read_file(path: impl AsRef<Path>) -> Result<Exits, Error> {
    let path = path.as_ref();
    let text =
        std::fs::read_to_string(path).map_err(|err| Error::Io(path.to_owned(), err.kind()))?;

    ron::from_str(&text).map_err(|err| Error::Parse(path.to_owned(), err.to_string()))
}

/// The keys to settle a tail onto, one per bone.
pub fn target_keys(
    name: &str,
    spec: &ExitSpec,
    start: u16,
    end: u16,
    target_dir: impl AsRef<Path>,
    qkeys: &nx_stdkey::StdKey,
    tkeys: &nx_stdkey::StdKey,
    skeleton: &nx_skel::boned::Skeleton,
) -> Result<Vec<Vec<TargetKey>>, crate::Error> {
    let resolved = spec.exit.resolve();
    let path = target_dir
        .as_ref()
        .join(format!("{}ska.dat", resolved.anim));

    if !path.exists() {
        return Err(crate::Error::Exit(Error::MissingTarget(
            name.to_owned(),
            resolved.anim.to_owned(),
        )));
    }

    let target = crate::Animation::read_file(&path, &mut ())?.to_boned(qkeys, tkeys);
    let cycle = target.last_key_time();

    // Every frame of the window, and which frame of the target each one shows.
    let frames: Vec<(u16, u16)> = (start + 1..=end)
        .map(|time| {
            let target_frame = match resolved.follow {
                Follow::Pose => resolved.frame,
                Follow::Motion => match cycle {
                    0 => resolved.frame,
                    cycle => {
                        let back = end.saturating_sub(time) as i32;
                        (resolved.frame as i32 - back).rem_euclid(cycle as i32) as u16
                    }
                },
            };

            (time, target_frame)
        })
        .collect();

    let poses: Vec<_> = frames
        .iter()
        .map(|(_, target_frame)| {
            let pose = target.pose_at(*target_frame);

            match resolved.mirror {
                true => pose.mirror(skeleton),
                false => pose,
            }
        })
        .collect();

    let bones = skeleton.bones.len();

    Ok((0..bones)
        .map(|bone| {
            frames
                .iter()
                .zip(poses.iter())
                .map(|((time, _), pose)| {
                    let (quaternion, translation) = pose.bone(bone);

                    TargetKey {
                        time: *time,
                        quaternion,
                        translation,
                    }
                })
                .collect()
        })
        .collect())
}
