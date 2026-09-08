use std::{fmt, io, ops};

use crate::{Reader, Writer};
use byteorder::{LE, ReadBytesExt, WriteBytesExt};

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn read(reader: &mut impl Reader) -> io::Result<Self> {
        let x = reader.read_f32::<LE>()?;
        let y = reader.read_f32::<LE>()?;
        let z = reader.read_f32::<LE>()?;
        let w = reader.read_f32::<LE>()?;
        Ok(Vec4 { x, y, z, w })
    }

    pub fn write(&self, writer: &mut impl Writer) -> io::Result<()> {
        writer.write_f32::<LE>(self.x)?;
        writer.write_f32::<LE>(self.y)?;
        writer.write_f32::<LE>(self.z)?;
        writer.write_f32::<LE>(self.w)?;
        Ok(())
    }
}

impl fmt::Display for Vec4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({:.4}, {:.4}, {:.4}, {:.4})",
            self.x, self.y, self.z, self.w
        )
    }
}

impl ops::Sub for Vec4 {
    type Output = Vec4;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}
