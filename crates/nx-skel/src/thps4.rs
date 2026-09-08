use std::{io, num::NonZeroU32};

use byteorder::{LE, ReadBytesExt, WriteBytesExt};
use itertools::izip;

use nx_common::{Reader, Writer};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bone {
    name: u32,
    parent: Option<NonZeroU32>,
    flip: Option<NonZeroU32>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Skeleton {
    version: u16,
    unk: u16,
    num_bones: u32,
    bones: Vec<Bone>,
}

impl Skeleton {
    pub fn read(reader: &mut impl Reader) -> io::Result<Self> {
        let version = reader.read_u16::<LE>()?;
        let unk = reader.read_u16::<LE>()?;
        let num_bones = reader.read_u32::<LE>()?;

        log::debug!("version={}", version);
        log::debug!("num_bones={}", num_bones);

        let mut name_list = Vec::new();
        for i in 0..num_bones {
            let checksum = reader.read_u32::<LE>()?;
            log::debug!("bone_{}={:#08x}", i, checksum);
            name_list.push(checksum);
        }

        let mut parent_list = Vec::new();
        for i in 0..num_bones {
            let checksum = reader.read_u32::<LE>()?;
            log::debug!("parent_{}={:#08x}", i, checksum);
            parent_list.push(checksum);
        }

        let mut flip_list = Vec::new();
        for i in 0..num_bones {
            let checksum = reader.read_u32::<LE>()?;
            log::debug!("flip_{}={:#08x}", i, checksum);
            flip_list.push(checksum);
        }

        let mut bones = Vec::new();
        for (name, parent, flip) in izip!(name_list, parent_list, flip_list) {
            bones.push(Bone {
                name,
                parent: NonZeroU32::new(parent),
                flip: NonZeroU32::new(flip),
            })
        }

        log::debug!("bones={:?}", bones);

        Ok(Self {
            version,
            unk,
            num_bones,
            bones,
        })
    }

    pub fn write(&self, writer: &mut impl Writer) -> io::Result<()> {
        writer.write_u16::<LE>(self.version)?;
        writer.write_u16::<LE>(self.unk)?;
        writer.write_u32::<LE>(self.num_bones)?;

        for bone in self.bones.iter() {
            writer.write_u32::<LE>(bone.name)?;
        }

        for bone in self.bones.iter() {
            writer.write_u32::<LE>(bone.parent.map_or(0, |v| v.get()))?;
        }

        for bone in self.bones.iter() {
            writer.write_u32::<LE>(bone.flip.map_or(0, |v| v.get()))?;
        }

        Ok(())
    }
}
