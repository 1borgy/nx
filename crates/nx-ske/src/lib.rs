use std::{fmt::Display, io};

use byteorder::{LE, ReadBytesExt};
use nx_common::{Readable, Writable};
use thiserror::Error;

mod common;
mod thug2;

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(io::ErrorKind),

    #[error("unimplemented ske version: {0}")]
    UnsupportedVersion(Version),

    #[error("unknown ske version: {0}")]
    UnknownVersion(u16),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value.kind())
    }
}

#[derive(Debug)]
pub enum Version {
    THPS4,
    THUG2,
    THAW,
}

impl TryFrom<u16> for Version {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Version::THUG2),
            1 => Ok(Version::THAW),
            _ => Ok(Version::THPS4),
            // _ => Err(UnknownVersion(value)),
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Version::THPS4 => "thps4",
                Version::THUG2 => "thug2",
                Version::THAW => "thaw",
            }
        )
    }
}

pub enum Skeleton {
    Thug2(thug2::Skeleton),
}

impl Skeleton {
    pub fn read(reader: &mut impl Readable) -> Result<Self, Error> {
        let version = Version::try_from(reader.read_u16::<LE>()?)?;
        reader.seek(io::SeekFrom::Start(0))?;

        match version {
            Version::THPS4 => Err(Error::UnsupportedVersion(version)),
            Version::THUG2 => Ok(thug2::Skeleton::read(reader).map(Skeleton::Thug2)?),
            Version::THAW => Err(Error::UnsupportedVersion(version)),
        }
    }
}
