// Here, "boned" describes a simple format that can be used to convert between games.
// Anim files themselves store quaternions/translations scaled down, so we scale back up
// when converting to a "boned" animation.

use std::ops;

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Quaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn from_xyz(x: f32, y: f32, z: f32, sign_bit: bool) -> Self {
        let sign = if sign_bit { -1 } else { 1 };
        let w = (1.0 - x.powi(2) - y.powi(2) - z.powi(2)).sqrt() * (sign as f32);
        Self::new(x, y, z, w)
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn w(&self) -> f32 {
        self.w
    }
}

impl ops::Mul for Quaternion {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            z: self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Translation {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Translation {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl ops::Add for Translation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for Translation {
    type Output = Translation;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

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
pub struct Bone {
    pub q_frames: Vec<QFrame>,
    pub t_frames: Vec<TFrame>,
}

impl Bone {
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
    pub bones: Vec<Bone>,
}

impl Animation {
    pub fn bone(&self, index: usize) -> Bone {
        self.bones.get(index).cloned().unwrap_or(Bone::empty())
    }
}
