use nx_common::Vec4;
use std::{fmt, ops};

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

    pub fn identity() -> Self {
        Self::new(0., 0., 0., 1.)
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

impl From<Vec4> for Quaternion {
    fn from(value: Vec4) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        }
    }
}

impl fmt::Display for Quaternion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({:.4}, {:.4}, {:.4}, {:.4})",
            self.x, self.y, self.z, self.w
        )
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

    pub fn identity() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn angles(&self) -> (f32, f32, f32) {
        let mag = self.magnitude();
        (
            (self.x / mag).acos(),
            (self.y / mag).acos(),
            (self.z / mag).acos(),
        )
    }

    pub fn dot(&self, other: &Translation) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Translation) -> Translation {
        Translation {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.z * other.y - self.y * other.x,
        }
    }

    pub fn scale(&self, v: f32) -> Translation {
        Translation {
            x: self.x * v,
            y: self.y * v,
            z: self.z * v,
        }
    }

    pub fn rotate(&self, q: &Quaternion) -> Translation {
        let u = Translation {
            x: q.x,
            y: q.y,
            z: q.z,
        };
        let s = q.w();

        u.scale(2.0 * self.dot(&u))
            + self.scale(s.powi(2) - u.dot(&u))
            + u.cross(&self).scale(2.0 * s)
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

impl From<Vec4> for Translation {
    fn from(value: Vec4) -> Self {
        Self {
            x: value.x * value.w,
            y: value.y * value.w,
            z: value.z * value.w,
        }
    }
}

impl fmt::Display for Translation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.4}, {:.4}, {:.4})", self.x, self.y, self.z)
    }
}
