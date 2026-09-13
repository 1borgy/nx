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
