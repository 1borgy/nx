use std::io;

use byteorder::{LE, ReadBytesExt};
use nx_common::{Readable, Reader, Writable, Writer};
use thiserror::Error;

mod boned;
mod thps4;
mod thug2;

pub use boned::Bone;

pub const THAW_HEADER_VERSION: u16 = 1;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(io::ErrorKind),

    #[error("unknown ske version: {0}")]
    UnknownVersion(u16),

    #[error("THPS4 skeleton file does not contain quaternions/translations")]
    Thps4NoQT,
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value.kind())
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Skeleton {
    Thps4(thps4::Skeleton),
    Thug2(thug2::Skeleton),
}

impl Readable for Skeleton {
    type Error = Error;
    type ReadContext = ();

    fn read(reader: &mut impl Reader, _: &mut ()) -> Result<Self, Error> {
        let version = reader.read_u16::<LE>()?;
        reader.seek(io::SeekFrom::Start(0))?;

        match version {
            thug2::HEADER_VERSION => Ok(thug2::Skeleton::read(reader).map(Skeleton::Thug2)?),
            THAW_HEADER_VERSION => Err(Error::UnknownVersion(version)),
            _ => Ok(thps4::Skeleton::read(reader).map(Skeleton::Thps4)?),
        }
    }
}

impl Writable for Skeleton {
    type Error = Error;
    type WriteContext = ();

    fn write(&self, writer: &mut impl Writer, _: &mut ()) -> Result<(), Error> {
        match self {
            Self::Thug2(skeleton) => Ok(skeleton.write(writer)?),
            Self::Thps4(skeleton) => Ok(skeleton.write(writer)?),
        }
    }
}

impl Skeleton {
    pub fn to_boned(&self) -> Result<boned::Skeleton, Error> {
        match self {
            Self::Thug2(skeleton) => Ok(skeleton.to_boned()),
            Self::Thps4(_) => Err(Error::Thps4NoQT),
        }
    }
}
