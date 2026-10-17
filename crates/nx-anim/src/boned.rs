// Here, "boned" describes a simple format that can be used to convert between games.
// Anim files themselves store quaternions/translations scaled down, so we scale back up
// when converting to a "boned" animation.
use nx_bone::{Quaternion, Translation};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QFrame {
    pub time: u16,
    pub quaternion: Quaternion,
}

impl QFrame {
    pub fn new(time: u16, quaternion: Quaternion) -> Self {
        Self { time, quaternion }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TFrame {
    pub time: u16,
    pub translation: Translation,
}

impl TFrame {
    pub fn new(time: u16, translation: Translation) -> Self {
        Self { time, translation }
    }

    pub fn scale(&self, factor: f32) -> Self {
        Self {
            time: self.time,
            translation: self.translation.scale(factor),
        }
    }

    pub fn translate(&self, translation: Translation) -> Self {
        Self {
            time: self.time,
            translation: self.translation + translation,
        }
    }
}

/// Sample a rotation track at `time`, interpolating between the keys that bracket it and
/// clamping outside the track. `None` only for a track with no keys at all.
fn sample_quaternion(frames: &[QFrame], time: u16) -> Option<Quaternion> {
    let first = frames.first()?;

    if time <= first.time {
        return Some(first.quaternion);
    }

    for pair in frames.windows(2) {
        let (from, to) = (&pair[0], &pair[1]);

        if time <= to.time {
            let span = to.time.saturating_sub(from.time);
            let alpha = match span {
                0 => 0.0,
                span => (time - from.time) as f32 / span as f32,
            };

            return Some(from.quaternion.nlerp(&to.quaternion, alpha));
        }
    }

    frames.last().map(|frame| frame.quaternion)
}

/// Sample a translation track at `time`, interpolating between the keys that bracket it and
/// clamping outside the track. `None` only for a track with no keys at all.
fn sample_translation(frames: &[TFrame], time: u16) -> Option<Translation> {
    let first = frames.first()?;

    if time <= first.time {
        return Some(first.translation);
    }

    for pair in frames.windows(2) {
        let (from, to) = (&pair[0], &pair[1]);

        if time <= to.time {
            let span = to.time.saturating_sub(from.time);
            let alpha = match span {
                0 => 0.0,
                span => (time - from.time) as f32 / span as f32,
            };

            return Some(from.translation + (to.translation - from.translation).scale(alpha));
        }
    }

    frames.last().map(|frame| frame.translation)
}

/// Half a quantization step, so collapsing never discards motion the file could represent.
/// Quaternion components store at 1/16384, translations at 1/32.
const DEDUPE_Q_EPSILON: f32 = 0.00003;
const DEDUPE_T_EPSILON: f32 = 0.015;

/// Drop keys whose value matches both neighbours, to reduce file size.
fn collapse_runs<T: Clone>(frames: Vec<T>, equal: impl Fn(&T, &T) -> bool) -> Vec<T> {
    if frames.len() < 3 {
        return frames;
    }

    let mut kept = Vec::with_capacity(frames.len());

    for (index, frame) in frames.iter().enumerate() {
        let interior = index > 0 && index + 1 < frames.len();
        let redundant =
            interior && equal(frame, &frames[index - 1]) && equal(frame, &frames[index + 1]);

        if !redundant {
            kept.push(frame.clone());
        }
    }

    kept
}

fn q_frames_equal(a: &QFrame, b: &QFrame) -> bool {
    let (x, y) = (a.quaternion, b.quaternion);

    (x.x() - y.x()).abs() < DEDUPE_Q_EPSILON
        && (x.y() - y.y()).abs() < DEDUPE_Q_EPSILON
        && (x.z() - y.z()).abs() < DEDUPE_Q_EPSILON
        && (x.w() - y.w()).abs() < DEDUPE_Q_EPSILON
}

fn t_frames_equal(a: &TFrame, b: &TFrame) -> bool {
    let (x, y) = (a.translation, b.translation);

    (x.x - y.x).abs() < DEDUPE_T_EPSILON
        && (x.y - y.y).abs() < DEDUPE_T_EPSILON
        && (x.z - y.z).abs() < DEDUPE_T_EPSILON
}

/// One frame of the pose a tail is being settled onto.
#[derive(Debug, Clone, Copy)]
pub struct TargetKey {
    pub time: u16,
    pub quaternion: Quaternion,
    pub translation: Translation,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AnimBone {
    pub q_frames: Vec<QFrame>,
    pub t_frames: Vec<TFrame>,
}

impl AnimBone {
    pub fn empty() -> Self {
        Self {
            q_frames: Vec::new(),
            t_frames: Vec::new(),
        }
    }

    pub fn static_bone(bone: nx_skel::Bone) -> Self {
        Self::empty()
            .with_static_quaternion(bone.quaternion)
            .with_static_translation(bone.translation)
    }

    pub fn with_static_quaternion(self, quaternion: Quaternion) -> Self {
        Self {
            q_frames: vec![QFrame::new(0, quaternion)],
            ..self
        }
    }

    pub fn with_static_translation(self, translation: Translation) -> Self {
        Self {
            t_frames: vec![TFrame::new(0, translation)],
            ..self
        }
    }

    pub fn scale_translations(self, factor: f32) -> Self {
        Self {
            t_frames: self
                .t_frames
                .iter()
                .map(|frame| frame.scale(factor))
                .collect(),
            ..self
        }
    }

    pub fn translate(self, translation: Translation) -> Self {
        Self {
            t_frames: self
                .t_frames
                .iter()
                .map(|frame| frame.translate(translation))
                .collect(),
            ..self
        }
    }

    /// Composes the bone with its child bone.
    /// That is useful when a target skeleton has one bone, but the source has two.
    /// An example is THPS4's single stomach bone, where THUG has stomach lower and
    /// upper.
    pub fn compose_child(self, child: &AnimBone) -> Self {
        if child.q_frames.is_empty() {
            return self;
        }

        let mut times: Vec<u16> = self
            .q_frames
            .iter()
            .chain(child.q_frames.iter())
            .map(|frame| frame.time)
            .collect();
        times.sort_unstable();
        times.dedup();

        let q_frames = times
            .into_iter()
            .map(|time| {
                let parent =
                    sample_quaternion(&self.q_frames, time).unwrap_or_else(Quaternion::identity);
                let child =
                    sample_quaternion(&child.q_frames, time).unwrap_or_else(Quaternion::identity);

                QFrame::new(time, child * parent)
            })
            .collect();

        Self { q_frames, ..self }
    }

    /// Skeletons might be built differently. For example THPS4's neck
    /// is tilted, while others isn't. This makes sure to apply a possible
    /// difference by baking these corrective rotations into the animation frames.
    /// So f.e that heads up, still means heads up.
    pub fn rebase_rotations(self, src: nx_skel::Bone, dst: nx_skel::Bone) -> Self {
        let delta = dst.quaternion * src.quaternion.conjugate();

        Self {
            q_frames: self
                .q_frames
                .into_iter()
                .map(|frame| QFrame::new(frame.time, delta * frame.quaternion))
                .collect(),
            ..self
        }
    }

    /// The game cuts from a trick straight into the next animation with no crossfade, so if the
    /// two don't line up the skater visibly jumps. This bends the last stretch of the trick so it
    /// ends in exactly the pose the next animation starts in, leaving nothing to jump across.
    ///
    /// It does that by mixing the trick toward that target pose over the final frame, barely at
    /// the start of the stretch, fully by the last frame. The mix eases in and out instead of
    /// ramping evenly, so the movement neither kinks where it begins nor arrives abruptly.
    pub fn settle_to(self, targets: &[TargetKey], start: u16, end: u16) -> Self {
        if end <= start {
            return self;
        }

        let weight = |time: u16| -> f32 {
            if time <= start {
                return 0.0;
            }
            if time >= end {
                return 1.0;
            }

            let alpha = (time - start) as f32 / (end - start) as f32;

            // smoothstep
            alpha * alpha * (3.0 - 2.0 * alpha)
        };

        let pinned_q = sample_quaternion(&self.q_frames, start);
        let pinned_t = sample_translation(&self.t_frames, start);

        let mut q_frames: Vec<QFrame> = self
            .q_frames
            .iter()
            .filter(|frame| frame.time < start)
            .cloned()
            .collect();

        if let Some(pinned) = pinned_q {
            q_frames.push(QFrame::new(start, pinned));

            for target in targets {
                let own = sample_quaternion(&self.q_frames, target.time).unwrap_or(pinned);

                q_frames.push(QFrame::new(
                    target.time,
                    own.nlerp(&target.quaternion, weight(target.time)),
                ));
            }
        }

        let mut t_frames: Vec<TFrame> = self
            .t_frames
            .iter()
            .filter(|frame| frame.time < start)
            .cloned()
            .collect();

        if let Some(pinned) = pinned_t {
            t_frames.push(TFrame::new(start, pinned));

            for target in targets {
                let own = sample_translation(&self.t_frames, target.time).unwrap_or(pinned);
                let alpha = weight(target.time);

                t_frames.push(TFrame::new(
                    target.time,
                    own + (target.translation - own).scale(alpha),
                ));
            }
        }

        Self {
            q_frames: collapse_runs(q_frames, q_frames_equal),
            t_frames: collapse_runs(t_frames, t_frames_equal),
        }
    }

    pub fn baseline(self, src: nx_skel::Bone, dst: nx_skel::Bone) -> Self {
        // return self;
        // return self.translate(dst.translation - src.translation);

        // Split src/dst translation vectors into their magnitude and angle components
        let src_mag = src.translation.magnitude();
        let (src_tx, src_ty, src_tz) = src.translation.to_angles();

        let dst_mag = dst.translation.magnitude();
        let (dst_tx, dst_ty, dst_tz) = dst.translation.to_angles();

        Self {
            t_frames: self
                .t_frames
                .into_iter()
                .map(|frame| {
                    // Find angle diff between frame and default position
                    let (d_tx, d_ty, d_tz) = if src_mag > 0.01 {
                        let (new_src_tx, new_src_ty, new_src_tz) = frame.translation.to_angles();
                        (
                            new_src_tx - src_tx,
                            new_src_ty - src_ty,
                            new_src_tz - src_tz,
                        )
                    } else {
                        // Default to empty if magnitude is below epsilon (0.01)
                        (0.0, 0.0, 0.0)
                    };

                    // Add angle diff to dst baseline vector
                    let (new_dst_tx, new_dst_ty, new_dst_tz) =
                        (dst_tx + d_tx, dst_ty + d_ty, dst_tz + d_tz);

                    TFrame {
                        time: frame.time,
                        translation: Translation::from_angles(
                            new_dst_tx, new_dst_ty, new_dst_tz, dst_mag,
                        ),
                    }
                })
                .collect(),
            q_frames: self.q_frames,
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Animation {
    pub duration: f32,
    pub bones: Vec<AnimBone>,
}

impl Animation {
    pub fn bone(&self, index: usize) -> AnimBone {
        self.bones.get(index).cloned().unwrap_or(AnimBone::empty())
    }

    /// The whole skeleton's pose at `time`, one rotation and translation per bone.
    pub fn pose_at(&self, time: u16) -> Pose {
        Pose {
            bones: self
                .bones
                .iter()
                .map(|bone| {
                    (
                        sample_quaternion(&bone.q_frames, time)
                            .unwrap_or_else(Quaternion::identity),
                        sample_translation(&bone.t_frames, time)
                            .unwrap_or_else(Translation::identity),
                    )
                })
                .collect(),
        }
    }

    /// Frame number of the last key in the animation.
    pub fn last_key_time(&self) -> u16 {
        self.bones
            .iter()
            .flat_map(|bone| {
                bone.q_frames
                    .iter()
                    .map(|frame| frame.time)
                    .chain(bone.t_frames.iter().map(|frame| frame.time))
            })
            .max()
            .unwrap_or(0)
    }
}

/// One frame of a skeleton, used as the pose a converted animation has to land on.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pose {
    pub bones: Vec<(Quaternion, Translation)>,
}

impl Pose {
    pub fn bone(&self, index: usize) -> (Quaternion, Translation) {
        self.bones
            .get(index)
            .copied()
            .unwrap_or((Quaternion::identity(), Translation::identity()))
    }

    /// Mirror the pose the way the engine's stance flip does: swap each bone with its
    /// left/right counterpart, negate the quaternion's y and z, and negate the translation's
    /// x. Needed because FlipAfter toggles at the same instant the animation is cut, so a
    /// converted animation has to end on the *mirror* of what plays next.
    pub fn mirror(&self, skeleton: &nx_skel::boned::Skeleton) -> Pose {
        let partner = |index: usize| -> usize {
            skeleton
                .get_index(index)
                .and_then(|bone| bone.flip)
                .and_then(|name| {
                    skeleton
                        .bones
                        .iter()
                        .position(|other| other.name == name.get())
                })
                .unwrap_or(index)
        };

        Pose {
            bones: (0..self.bones.len())
                .map(|index| {
                    let (q, t) = self.bone(partner(index));

                    (
                        Quaternion::new(q.x(), -q.y(), -q.z(), q.w()),
                        Translation::new(-t.x, t.y, t.z),
                    )
                })
                .collect(),
        }
    }
}
