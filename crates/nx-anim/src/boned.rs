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

    pub fn baseline(self, src: nx_skel::Bone, dst: nx_skel::Bone) -> Self {
        // return self;
        // return self.translate(dst.translation - src.translation);

        // Split src/dst translation vectors into their magnitude and angle components
        let src_mag = src.translation.magnitude();
        let (src_tx, src_ty, src_tz) = src.translation.to_angles();

        let dst_mag = dst.translation.magnitude();
        let (dst_tx, dst_ty, dst_tz) = dst.translation.to_angles();

        let (src_rx, src_ry, src_rz) = src.quaternion.to_angles();
        let (dst_rx, dst_ry, dst_rz) = dst.quaternion.to_angles();

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
            // q_frames: self
            //     .q_frames
            //     .into_iter()
            //     .map(|frame| {
            //         let _ = 0;
            //         let (new_src_rx, new_src_ry, new_src_rz) = frame.quaternion.to_angles();
            //         let (d_rx, d_ry, d_rz) = (
            //             new_src_rx - src_rx,
            //             new_src_ry - src_ry,
            //             new_src_rz - src_rz,
            //         );
            //         let (new_dst_rx, new_dst_ry, new_dst_rz) =
            //             (dst_rx + d_rx, dst_ry + d_ry, dst_rz + d_rz);
            //         QFrame {
            //             time: frame.time,
            //             quaternion: Quaternion::from_angles(new_dst_rx, new_dst_ry, new_dst_rz),
            //         }
            //     })
            //     .collect(),
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
}
