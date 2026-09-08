// standardkeyQ.bin / standardkeyT.bin

use std::io;

use byteorder::{LE, ReadBytesExt, WriteBytesExt};
use nx_common::{Readable, Reader, Writable, Writer};

pub const NUM_KEYS: usize = 256;

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Key {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub n: i16,
}

impl Key {
    pub fn read(reader: &mut impl Reader) -> io::Result<Self> {
        let x = reader.read_i16::<LE>()?;
        let y = reader.read_i16::<LE>()?;
        let z = reader.read_i16::<LE>()?;
        let n = reader.read_i16::<LE>()?;

        Ok(Self { x, y, z, n })
    }

    pub fn write(&self, writer: &mut impl Writer) -> io::Result<()> {
        writer.write_i16::<LE>(self.x)?;
        writer.write_i16::<LE>(self.y)?;
        writer.write_i16::<LE>(self.z)?;
        writer.write_i16::<LE>(self.n)?;

        Ok(())
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StdKey {
    keys: Vec<Key>,
}

impl Readable for StdKey {
    type Error = io::Error;
    type ReadContext = ();

    fn read(reader: &mut impl Reader, _: &mut ()) -> io::Result<Self> {
        let mut keys = Vec::new();

        for _ in 0..NUM_KEYS {
            keys.push(Key::read(reader)?);
        }

        Ok(Self { keys })
    }
}

impl Writable for StdKey {
    type Error = io::Error;
    type WriteContext = ();

    fn write(&self, writer: &mut impl Writer, _: &mut ()) -> io::Result<()> {
        for key in self.keys.iter() {
            key.write(writer)?;
        }

        Ok(())
    }
}

impl StdKey {
    pub fn get(&self, index: u8) -> Key {
        // XXX (ellie): invariant: NUM_KEYS=256 so as long as `index` is a u8 this will
        // never panic
        self.keys[index as usize]
    }
}
