use std::io;

use byteorder::{LE, ReadBytesExt};
use nx_common::{Readable, Reader, Writable, Writer};
use nx_stdkey::StdKey;
use thiserror::Error;

pub mod boned;
mod convert;
pub mod mapping;
mod thug2;

pub use convert::{convert, convert_path};

#[derive(Debug, Error)]
pub enum Error {
    #[error("io error: {0}")]
    Io(io::ErrorKind),

    #[error("unknown anim version: {0}")]
    UnknownVersion(u32),

    #[error("mapping not implemented: {0} to {1}")]
    MappingNotImplemented(nx_common::Game, nx_common::Game),

    #[error("unknown error while converting path")]
    FilenameError,

    #[error("skeleton error: {0}")]
    SkeletonError(#[from] nx_skel::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value.kind())
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Animation {
    Thug2(thug2::Animation),
}

impl Readable for Animation {
    type Error = Error;
    type ReadContext = ();

    fn read(reader: &mut impl Reader, _: &mut ()) -> Result<Self, Error> {
        let version = reader.read_u32::<LE>()?;
        reader.seek(io::SeekFrom::Start(0))?;

        match version {
            thug2::HEADER_VERSION => Ok(thug2::Animation::read(reader).map(Animation::Thug2)?),
            _ => Err(Error::UnknownVersion(version)),
        }
    }
}

#[derive(Clone)]
pub struct WriteContext {
    pub game: nx_common::Game,
}

impl Writable for Animation {
    type Error = io::Error;
    type WriteContext = WriteContext;

    fn write(&self, writer: &mut impl Writer, ctx: &mut WriteContext) -> io::Result<()> {
        match self {
            Self::Thug2(animation) => Ok(animation.write(writer, ctx)?),
        }
    }
}

impl Animation {
    pub fn decompress(&self, qkeys: &StdKey, tkeys: &StdKey) -> Animation {
        match self {
            Self::Thug2(animation) => Self::Thug2(animation.decompress(qkeys, tkeys)),
        }
    }

    pub fn to_boned(&self, qkeys: &StdKey, tkeys: &StdKey) -> boned::Animation {
        match self {
            Self::Thug2(animation) => animation.to_boned(qkeys, tkeys),
        }
    }

    pub fn from_boned(boned: &boned::Animation, game: nx_common::Game) -> Self {
        match game {
            nx_common::Game::THPS4 | nx_common::Game::THUG | nx_common::Game::THUG2 => {
                Self::Thug2(thug2::Animation::from_boned(boned))
            }
        }
    }
}
