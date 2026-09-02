use std::io;

use byteorder::{LE, ReadBytesExt};

use nx_common::Readable;

use crate::common::Vec4;

pub struct Skeleton {
    version: u16,
    unk: u32,
    num_bones: u32,
}

impl Skeleton {
    pub fn new() -> Self {
        todo!();
    }

    pub fn read(reader: &mut impl Readable) -> io::Result<Self> {
        let version = reader.read_u16::<LE>()?;
        let _ = reader.read_u16::<LE>()?;
        let unk = reader.read_u32::<LE>()?;
        let num_bones = reader.read_u32::<LE>()?;

        log::debug!("version={}", version);
        log::debug!("unk={}", unk);
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

        let mut trans_list = Vec::new();

        for i in 0..num_bones {
            let vector = Vec4::read(reader)?;
            log::debug!("trans_{}={:?}", i, &vector);
            trans_list.push(vector);
        }

        Ok(Self {
            version,
            unk,
            num_bones,
        })
    }

    // pub fn write(&self, writer: &mut impl Writable) -> Result<(), Error> {
    //     Ok(())
    // }
}
