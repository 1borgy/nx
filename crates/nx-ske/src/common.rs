use std::io;

use byteorder::{LE, ReadBytesExt};
use nx_common::{Readable, Writable};

use crate::Error;

#[derive(Debug)]
pub struct Vec4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vec4 {
    pub fn read(reader: &mut impl Readable) -> io::Result<Self> {
        let x = reader.read_f32::<LE>()?;
        let y = reader.read_f32::<LE>()?;
        let z = reader.read_f32::<LE>()?;
        let w = reader.read_f32::<LE>()?;
        Ok(Vec4 { x, y, z, w })
    }
}
