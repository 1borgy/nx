use std::{io, num::NonZeroU32};

use byteorder::{LE, ReadBytesExt, WriteBytesExt};
use itertools::izip;

use nx_common::{Reader, Vec4, Writer};

use crate::boned;

pub const HEADER_VERSION: u16 = 2;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bone {
    name: u32,
    parent: Option<NonZeroU32>,
    flip: Option<NonZeroU32>,
    quaternion: Vec4,
    translation: Vec4,
}

impl Bone {
    pub fn to_boned(&self) -> boned::Bone {
        boned::Bone {
            name: self.name,
            parent: self.parent,
            flip: self.flip,
            quaternion: self.quaternion.into(),
            translation: self.translation.into(),
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Skeleton {
    version: u16,
    unk1: u16,
    unk2: u32,
    num_bones: u32,
    bones: Vec<Bone>,
}

impl Skeleton {
    pub fn read(reader: &mut impl Reader) -> io::Result<Self> {
        let version = reader.read_u16::<LE>()?;
        let unk1 = reader.read_u16::<LE>()?;
        let unk2 = reader.read_u32::<LE>()?;
        let num_bones = reader.read_u32::<LE>()?;

        log::debug!("version={}", version);
        log::debug!("unk1={}", unk1);
        log::debug!("unk2={}", unk2);
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

        let mut quat_list = Vec::new();
        let mut trans_list = Vec::new();
        for i in 0..num_bones {
            let quat = Vec4::read(reader)?;
            log::debug!("quat_{}={:?}", i, &quat);
            quat_list.push(quat);
            let trans = Vec4::read(reader)?;
            log::debug!("trans_{}={:?}", i, &trans);
            trans_list.push(trans);
        }

        let mut bones = Vec::new();
        for (name, parent, flip, quat, trans) in
            izip!(name_list, parent_list, flip_list, quat_list, trans_list)
        {
            bones.push(Bone {
                name,
                parent: NonZeroU32::new(parent),
                flip: NonZeroU32::new(flip),
                quaternion: quat,
                translation: trans,
            })
        }

        log::debug!("bones={:?}", bones);

        Ok(Self {
            version,
            unk1,
            unk2,
            num_bones,
            bones,
        })
    }

    pub fn write(&self, writer: &mut impl Writer) -> io::Result<()> {
        writer.write_u16::<LE>(self.version)?;
        writer.write_u16::<LE>(self.unk1)?;
        writer.write_u32::<LE>(self.unk2)?;
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

        for bone in self.bones.iter() {
            bone.quaternion.write(writer)?;
            bone.translation.write(writer)?;
        }

        Ok(())
    }

    pub fn to_boned(&self) -> boned::Skeleton {
        boned::Skeleton::new(self.bones.iter().map(|bone| bone.to_boned()).collect())
    }
}
