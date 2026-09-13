use std::io;

use bitflags::bitflags;
use byteorder::{LE, ReadBytesExt, WriteBytesExt};

use nx_bone::{Quaternion, Translation};
use nx_common::{Game, Reader, Writer};
use nx_stdkey::StdKey;

use crate::WriteContext;
use crate::boned;

pub const HEADER_VERSION: u32 = 1;

const Q_SCALE: f32 = 16384.0;
const T_SCALE: f32 = 32.0;

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    struct HeaderFlags: u32 {
        // XXX: are these really all valid? or thaw-specific?
        const LONG_FRAME_TIMES = 1 << 8;
        const CUSTOM_LOOKUP_BUFFER = 1 << 13;
        const ONLY_SINGLE_BYTE_QUATS = 1 << 15;
        const USE_COMPRESS_TABLE_N8 = 1 << 16;
        const KEEP_MISSING_BONE_POS = 1 << 17;
        const PARTIAL_ANIM = 1 << 19;
        const CUTSCENE_DATA = 1 << 20;
        const CUSTOM_KEYS_AT_60FPS = 1 << 21;
        const HIRES_FRAME_POINTERS = 1 << 22;
        const USE_COMPRESS_TABLE = 1 << 23;
        const OBJECT_ANIM_DATA = 1 << 24;
        const PRE_ROTATED_ROOT = 1 << 25;
        const COMPRESSED_TIME = 1 << 26;
        const CAMERA_DATA = 1 << 27;
        const PLATFORM = 1 << 28;
        const UNCOMPRESSED = 1 << 29;
        const INTERMEDIATE = 1 << 30;
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct Header {
    version: u32,
    flags: HeaderFlags,
    duration: f32,
    num_bones: u32,
    num_q_keys: u32,
    num_t_keys: u32,
    num_custom_anim_keys: u32,
    q_alloc_size: u32,
    t_alloc_size: u32,
    q_block_sizes: Vec<u16>,
    t_block_sizes: Vec<u16>,
}

impl Header {
    fn read(reader: &mut impl Reader) -> io::Result<Self> {
        let version = reader.read_u32::<LE>()?;
        let flags = HeaderFlags::from_bits_retain(reader.read_u32::<LE>()?);
        log::info!("flags={:?}", flags);
        let duration = reader.read_f32::<LE>()?;

        let num_bones = reader.read_u32::<LE>()?;
        let num_q_keys = reader.read_u32::<LE>()?;
        let num_t_keys = reader.read_u32::<LE>()?;
        let num_custom_anim_keys = reader.read_u32::<LE>()?;
        let q_alloc_size = reader.read_u32::<LE>()?;
        let t_alloc_size = reader.read_u32::<LE>()?;

        let mut quat_block_sizes = Vec::with_capacity(num_bones as usize);

        for _ in 0..num_bones {
            quat_block_sizes.push(reader.read_u16::<LE>()?);
        }

        let mut trans_block_sizes = Vec::with_capacity(num_bones as usize);

        for _ in 0..num_bones {
            trans_block_sizes.push(reader.read_u16::<LE>()?);
        }

        Ok(Self {
            version,
            flags,
            duration,
            num_bones,
            num_q_keys,
            num_t_keys,
            num_custom_anim_keys,
            q_alloc_size,
            t_alloc_size,
            q_block_sizes: quat_block_sizes,
            t_block_sizes: trans_block_sizes,
        })
    }

    fn write(&self, writer: &mut impl Writer) -> io::Result<()> {
        writer.write_u32::<LE>(self.version)?;
        writer.write_u32::<LE>(self.flags.bits())?;
        writer.write_f32::<LE>(self.duration)?;
        writer.write_u32::<LE>(self.num_bones)?;
        writer.write_u32::<LE>(self.num_q_keys)?;
        writer.write_u32::<LE>(self.num_t_keys)?;
        writer.write_u32::<LE>(self.num_custom_anim_keys)?;
        writer.write_u32::<LE>(self.q_alloc_size)?;
        writer.write_u32::<LE>(self.t_alloc_size)?;

        for quat_block_size in self.q_block_sizes.iter() {
            writer.write_u16::<LE>(*quat_block_size)?;
        }

        for trans_block_size in self.t_block_sizes.iter() {
            writer.write_u16::<LE>(*trans_block_size)?;
        }

        Ok(())
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    struct QFlags: u16 {
        const W_SIGN_BIT = 1 << 15;
        const SINGLE_BYTE_VALUE = 1 << 14;
        const SINGLE_BYTE_X = 1 << 13;
        const SINGLE_BYTE_Y = 1 << 12;
        const SINGLE_BYTE_Z = 1 << 11;
        const UNK2 = 1 << 10;
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum QCoordKind {
    I8,
    I16,
    ShiftedI8,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum QCoord {
    U8(u8),      // one-byte unsigned value
    I16(i16),    // two-byte signed value
    Shifted(u8), // one-byte value that gets shifted left 8 bits and interpreted as i16
}

impl QCoord {
    fn read(reader: &mut impl Reader, kind: QCoordKind) -> io::Result<Self> {
        match kind {
            QCoordKind::I8 => Ok(Self::U8(reader.read_u8()?)),
            QCoordKind::I16 => Ok(Self::I16(reader.read_i16::<LE>()?)),
            QCoordKind::ShiftedI8 => Ok(Self::Shifted(reader.read_u8()?)),
        }
    }

    fn write(&self, writer: &mut impl Writer) -> io::Result<()> {
        match self {
            Self::U8(value) => Ok(writer.write_u8(*value)?),
            Self::I16(value) => Ok(writer.write_i16::<LE>(*value)?),
            Self::Shifted(value) => Ok(writer.write_u8(*value)?),
        }
    }

    fn decompress(&self) -> Self {
        match self {
            Self::U8(value) => Self::I16(*value as i16),
            Self::I16(value) => Self::I16(*value),
            Self::Shifted(value) => Self::I16(((*value as u16) << 8) as i16),
        }
    }

    fn coordinate(&self) -> f32 {
        match self {
            Self::U8(value) => *value as f32,
            Self::I16(value) => *value as f32,
            Self::Shifted(value) => (((*value as u16) << 8) as i16) as f32,
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum QFrameData {
    Key(u8),
    Inline { x: QCoord, y: QCoord, z: QCoord },
}

impl QFrameData {
    fn read(
        reader: &mut impl Reader,
        kinds: Option<(QCoordKind, QCoordKind, QCoordKind)>,
    ) -> io::Result<Self> {
        match kinds {
            None => Ok(Self::Key(reader.read_u8()?)),
            Some((x_size, y_size, z_size)) => Ok(Self::Inline {
                x: QCoord::read(reader, x_size)?,
                y: QCoord::read(reader, y_size)?,
                z: QCoord::read(reader, z_size)?,
            }),
        }
    }

    fn write(&self, writer: &mut impl Writer) -> io::Result<()> {
        match self {
            QFrameData::Key(index) => {
                writer.write_u8(*index)?;
            }
            QFrameData::Inline { x, y, z } => {
                x.write(writer)?;
                y.write(writer)?;
                z.write(writer)?;
            }
        }

        Ok(())
    }

    fn decompress(&self, qkeys: &StdKey) -> Self {
        match self {
            Self::Key(index) => {
                let key = qkeys.get(*index);
                Self::Inline {
                    x: QCoord::I16(key.x as i16),
                    y: QCoord::I16(key.y as i16),
                    z: QCoord::I16(key.z as i16),
                }
            }
            Self::Inline { x, y, z } => Self::Inline {
                x: x.decompress(),
                y: y.decompress(),
                z: z.decompress(),
            },
        }
    }

    fn quaternion(&self, qkeys: &StdKey, flags: QFlags) -> Quaternion {
        match self {
            Self::Key(index) => {
                let key = qkeys.get(*index);
                Quaternion::from_xyz(
                    key.x as f32 / Q_SCALE,
                    key.y as f32 / Q_SCALE,
                    key.z as f32 / Q_SCALE,
                    flags.contains(QFlags::W_SIGN_BIT),
                )
            }
            Self::Inline { x, y, z } => Quaternion::from_xyz(
                x.coordinate() / Q_SCALE,
                y.coordinate() / Q_SCALE,
                z.coordinate() / Q_SCALE,
                flags.contains(QFlags::W_SIGN_BIT),
            ),
        }
    }

    fn from_quaternion(quaternion: &Quaternion) -> Self {
        Self::Inline {
            x: QCoord::I16((quaternion.x() * Q_SCALE) as i16),
            y: QCoord::I16((quaternion.y() * Q_SCALE) as i16),
            z: QCoord::I16((quaternion.z() * Q_SCALE) as i16),
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct QFrame {
    time: Option<u16>, // Only if header flag LONG_FRAME_TIMES is set
    flags: QFlags,
    data: QFrameData,
}

impl QFrame {
    fn read(reader: &mut impl Reader, header_flags: HeaderFlags) -> io::Result<Self> {
        let time = match header_flags.contains(HeaderFlags::LONG_FRAME_TIMES) {
            true => Some(reader.read_u16::<LE>()?),
            false => None,
        };

        let flags = QFlags::from_bits_retain(reader.read_u16::<LE>()?);

        let coordinate_kinds = {
            match flags.contains(QFlags::SINGLE_BYTE_VALUE) {
                // Single-byte flag is not set, so all are either u8 or u16
                false => match header_flags.contains(HeaderFlags::ONLY_SINGLE_BYTE_QUATS) {
                    false => Some((QCoordKind::I16, QCoordKind::I16, QCoordKind::I16)),
                    true => Some((
                        QCoordKind::ShiftedI8,
                        QCoordKind::ShiftedI8,
                        QCoordKind::ShiftedI8,
                    )),
                },
                // Single-byte flag is set, so check which coordinates are single-byte
                true => match (flags
                    & (QFlags::SINGLE_BYTE_X | QFlags::SINGLE_BYTE_Y | QFlags::SINGLE_BYTE_Z))
                    != QFlags::empty()
                {
                    // No individual coordinates flagged as single byte, so this is a key
                    false => None,
                    // At least one flagged as a single byte; check if global single-byte quat flag is set
                    true => match header_flags.contains(HeaderFlags::ONLY_SINGLE_BYTE_QUATS) {
                        // Global single-byte quat flag is set; all are single byte
                        true => Some((
                            QCoordKind::ShiftedI8,
                            QCoordKind::ShiftedI8,
                            QCoordKind::ShiftedI8,
                        )),
                        // Global single-byte quat flag is not set; check each individual coordinate's flag
                        false => Some((
                            match flags.contains(QFlags::SINGLE_BYTE_X) {
                                true => QCoordKind::I8,
                                false => QCoordKind::I16,
                            },
                            match flags.contains(QFlags::SINGLE_BYTE_Y) {
                                true => QCoordKind::I8,
                                false => QCoordKind::I16,
                            },
                            match flags.contains(QFlags::SINGLE_BYTE_Z) {
                                true => QCoordKind::I8,
                                false => QCoordKind::I16,
                            },
                        )),
                    },
                },
            }
        };

        let data = QFrameData::read(reader, coordinate_kinds)?;

        Ok(Self { time, flags, data })
    }

    fn write(&self, writer: &mut impl Writer) -> io::Result<()> {
        if let Some(time) = self.time {
            writer.write_u16::<LE>(time)?;
        }

        // TODO: set flags based on data kind
        writer.write_u16::<LE>(self.flags.bits())?;

        self.data.write(writer)?;

        Ok(())
    }

    fn decompress(&self, qkeys: &StdKey) -> Self {
        Self {
            time: self.time,
            flags: self.flags
                - (QFlags::SINGLE_BYTE_VALUE
                    | QFlags::SINGLE_BYTE_X
                    | QFlags::SINGLE_BYTE_Y
                    | QFlags::SINGLE_BYTE_Z),
            data: QFrameData::decompress(&self.data, qkeys),
        }
    }

    fn to_boned(&self, qkeys: &StdKey) -> boned::QFrame {
        boned::QFrame {
            time: match self.time {
                Some(time) => time,
                None => (self.flags - QFlags::all()).bits() as u16,
            },
            quaternion: self.data.quaternion(qkeys, self.flags),
        }
    }

    fn from_boned(frame: &boned::QFrame) -> Self {
        let sign_bit = if frame.quaternion.w() < 0.0 {
            QFlags::W_SIGN_BIT
        } else {
            QFlags::empty()
        };

        Self {
            // time: Some(frame.time),
            // flags: QFlags::empty(),
            time: None,
            flags: QFlags::from_bits_retain(frame.time) | sign_bit,
            data: QFrameData::from_quaternion(&frame.quaternion),
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct QBlock {
    frames: Vec<QFrame>,
}

impl QBlock {
    fn read(
        reader: &mut impl Reader,
        block_size: u16,
        header_flags: HeaderFlags,
    ) -> io::Result<Self> {
        let block_start = reader.stream_position()?;
        let block_end = block_start + u64::from(block_size);

        log::debug!("reading quat block start={} end={}", block_start, block_end);

        let mut frames = Vec::new();

        let mut stream_position;
        while {
            stream_position = reader.stream_position()?;
            stream_position < block_end
        } {
            let frame = QFrame::read(reader, header_flags)?;

            frames.push(frame);
        }

        debug_assert!(
            stream_position == block_end,
            "read more bytes for quat block than expected"
        );

        Ok(Self { frames })
    }

    fn write(&self, writer: &mut impl Writer) -> io::Result<()> {
        for frame in self.frames.iter() {
            frame.write(writer)?;
        }

        Ok(())
    }

    fn decompress(&self, qkeys: &StdKey) -> Self {
        Self {
            frames: self
                .frames
                .iter()
                .map(|frame| QFrame::decompress(frame, qkeys))
                .collect(),
        }
    }
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    struct TFlags: u8 {
        const FLAG_TIME = 1 << 6;
        const USE_LOOKUP = 1 << 7;
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum TFrameData {
    Key(u8),
    Inline { x: i16, y: i16, z: i16 },
}

impl TFrameData {
    fn read(reader: &mut impl Reader, flags: TFlags) -> io::Result<Self> {
        match flags.contains(TFlags::USE_LOOKUP) {
            true => Ok(Self::Key(reader.read_u8()?)),
            false => Ok(Self::Inline {
                x: reader.read_i16::<LE>()?,
                y: reader.read_i16::<LE>()?,
                z: reader.read_i16::<LE>()?,
            }),
        }
    }

    fn write(&self, writer: &mut impl Writer) -> io::Result<()> {
        match self {
            Self::Key(index) => {
                writer.write_u8(*index)?;
            }
            Self::Inline { x, y, z } => {
                writer.write_i16::<LE>(*x)?;
                writer.write_i16::<LE>(*y)?;
                writer.write_i16::<LE>(*z)?;
            }
        }

        Ok(())
    }

    fn decompress(&self, tkeys: &StdKey) -> Self {
        match self {
            Self::Key(index) => {
                let key = tkeys.get(*index);
                Self::Inline {
                    x: key.x,
                    y: key.y,
                    z: key.z,
                }
            }
            Self::Inline { x, y, z } => Self::Inline {
                x: *x,
                y: *y,
                z: *z,
            },
        }
    }

    fn translation(&self, tkeys: &StdKey) -> Translation {
        match self {
            Self::Key(index) => {
                let key = tkeys.get(*index);
                Translation {
                    x: key.x as f32 / T_SCALE,
                    y: key.y as f32 / T_SCALE,
                    z: key.z as f32 / T_SCALE,
                }
            }
            Self::Inline { x, y, z } => Translation {
                x: *x as f32 / T_SCALE,
                y: *y as f32 / T_SCALE,
                z: *z as f32 / T_SCALE,
            },
        }
    }

    fn from_translation(translation: &Translation) -> Self {
        Self::Inline {
            x: (translation.x * T_SCALE) as i16,
            y: (translation.y * T_SCALE) as i16,
            z: (translation.z * T_SCALE) as i16,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct TFrame {
    flags: TFlags, // Time is lower 6 bits if FLAG_TIME is set
    time: Option<u16>,
    data: TFrameData,
}

impl TFrame {
    fn read(reader: &mut impl Reader) -> io::Result<Self> {
        let flags = TFlags::from_bits_retain(reader.read_u8()?);

        let time = match flags.contains(TFlags::FLAG_TIME) {
            true => None,
            false => Some(reader.read_u16::<LE>()?),
        };

        let data = TFrameData::read(reader, flags)?;

        Ok(Self { flags, time, data })
    }

    fn write(&self, writer: &mut impl Writer) -> io::Result<()> {
        writer.write_u8(self.flags.bits())?;

        if let Some(time) = self.time {
            writer.write_u16::<LE>(time)?;
        }

        self.data.write(writer)?;

        Ok(())
    }

    fn decompress(&self, tkeys: &StdKey) -> Self {
        Self {
            time: self.time,
            flags: self.flags - TFlags::USE_LOOKUP,
            data: TFrameData::decompress(&self.data, tkeys),
        }
    }

    fn to_boned(&self, tkeys: &StdKey) -> boned::TFrame {
        boned::TFrame {
            time: match self.time {
                Some(time) => time,
                None => (self.flags - TFlags::all()).bits() as u16,
            },
            translation: self.data.translation(tkeys),
        }
    }

    fn from_boned(frame: &boned::TFrame) -> Self {
        Self {
            time: Some(frame.time),
            flags: TFlags::empty(),
            data: TFrameData::from_translation(&frame.translation),
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct TBlock {
    frames: Vec<TFrame>,
}

impl TBlock {
    fn read(reader: &mut impl Reader, block_size: u16) -> io::Result<Self> {
        let block_start = reader.stream_position()?;
        let block_end = block_start + u64::from(block_size);

        log::debug!(
            "reading trans block start={} end={}",
            block_start,
            block_end
        );

        let mut frames = Vec::new();

        let mut stream_position;
        while {
            stream_position = reader.stream_position()?;
            stream_position < block_end
        } {
            frames.push(TFrame::read(reader)?);
        }

        debug_assert!(
            stream_position == block_end,
            "read more bytes for trans block than expected"
        );

        Ok(Self { frames })
    }

    fn write(&self, writer: &mut impl Writer) -> io::Result<()> {
        for frame in self.frames.iter() {
            frame.write(writer)?;
        }

        Ok(())
    }

    fn decompress(&self, tkeys: &StdKey) -> Self {
        Self {
            frames: self
                .frames
                .iter()
                .map(|frame| TFrame::decompress(frame, tkeys))
                .collect(),
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Animation {
    header: Header,
    q_blocks: Vec<QBlock>,
    t_blocks: Vec<TBlock>,
}

impl Animation {
    pub fn read(reader: &mut impl Reader) -> io::Result<Self> {
        let header = Header::read(reader)?;

        let q_blocks_start = reader.stream_position()?;

        let mut q_blocks = Vec::new();
        for q_block_size in header.q_block_sizes.iter() {
            q_blocks.push(QBlock::read(reader, *q_block_size, header.flags)?);
        }

        let q_blocks_end = reader.stream_position()?;

        debug_assert!(
            (q_blocks_end - q_blocks_start) as u32 == header.q_alloc_size,
            "read more bytes for quat blocks than expected"
        );

        let t_blocks_start = reader.stream_position()?;

        let mut t_blocks = Vec::new();
        for t_block_size in header.t_block_sizes.iter() {
            t_blocks.push(TBlock::read(reader, *t_block_size)?);
        }

        let t_blocks_end = reader.stream_position()?;

        debug_assert!(
            (t_blocks_end - t_blocks_start) as u32 == header.t_alloc_size,
            "read more bytes for trans blocks than expected"
        );

        Ok(Self {
            header,
            q_blocks,
            t_blocks,
        })
    }

    pub fn write(&self, writer: &mut impl Writer, ctx: &mut WriteContext) -> io::Result<()> {
        let header_position = writer.stream_position()?;
        self.header.write(writer)?;

        let mut num_q_frames = 0;
        let q_blocks_start = writer.stream_position()?;

        // TODO: enforce write returns number of bytes written
        let mut q_block_sizes = Vec::new();
        for q_block in self.q_blocks.iter() {
            num_q_frames += q_block.frames.len();
            let q_block_start = writer.stream_position()?;
            q_block.write(writer)?;
            let q_block_end = writer.stream_position()?;
            q_block_sizes.push((q_block_end - q_block_start) as u16);
        }

        let q_blocks_end = writer.stream_position()?;

        let mut num_t_frames = 0;
        let t_blocks_start = writer.stream_position()?;

        let mut t_block_sizes = Vec::new();
        for t_block in self.t_blocks.iter() {
            num_t_frames += t_block.frames.len();
            let t_block_start = writer.stream_position()?;
            t_block.write(writer)?;
            let t_block_end = writer.stream_position()?;
            t_block_sizes.push((t_block_end - t_block_start) as u16);
        }

        let t_blocks_end = writer.stream_position()?;

        match ctx.game {
            // Pad with 0s until 4-byte aligned
            Game::THUG | Game::THUG2 => {
                let num_bytes = writer.stream_position()?;
                let unaligned_bytes = num_bytes % 4;
                if unaligned_bytes > 0 {
                    for _ in unaligned_bytes..4 {
                        writer.write_u8(0)?;
                    }
                }
            }
            // THPS4 remains unpadded
            Game::THPS4 => (),
        }

        // Recalculate header
        let header = Header {
            num_q_keys: num_q_frames as u32,
            num_t_keys: num_t_frames as u32,
            q_block_sizes,
            t_block_sizes,
            q_alloc_size: (q_blocks_end - q_blocks_start) as u32,
            t_alloc_size: (t_blocks_end - t_blocks_start) as u32,
            ..self.header
        };

        // let header = Header {
        //     q_block_sizes,
        //     t_block_sizes,
        //     q_alloc_size: (q_blocks_end - q_blocks_start) as u32,
        //     t_alloc_size: (t_blocks_end - t_blocks_start) as u32,
        //     ..self.header
        // };

        writer.seek(io::SeekFrom::Start(header_position))?;
        header.write(writer)?;

        Ok(())
    }

    pub fn decompress(&self, qkeys: &StdKey, tkeys: &StdKey) -> Self {
        Self {
            // TODO: invalidate header?
            header: Header {
                q_block_sizes: self.header.q_block_sizes.clone(),
                t_block_sizes: self.header.t_block_sizes.clone(),
                ..self.header
            },
            q_blocks: self
                .q_blocks
                .iter()
                .map(|block| QBlock::decompress(block, qkeys))
                .collect(),
            t_blocks: self
                .t_blocks
                .iter()
                .map(|block| TBlock::decompress(block, tkeys))
                .collect(),
        }
    }

    pub fn to_boned(&self, qkeys: &StdKey, tkeys: &StdKey) -> boned::Animation {
        boned::Animation {
            duration: self.header.duration,
            bones: (0..self.header.num_bones)
                .map(|bone_index| boned::AnimBone {
                    q_frames: self
                        .q_blocks
                        .get(bone_index as usize)
                        .map(|b| b.frames.iter().map(|f| f.to_boned(qkeys)).collect())
                        .unwrap_or(Vec::new()),
                    t_frames: self
                        .t_blocks
                        .get(bone_index as usize)
                        .map(|b| b.frames.iter().map(|f| f.to_boned(tkeys)).collect())
                        .unwrap_or(Vec::new()),
                })
                .collect(),
        }
    }

    pub fn from_boned(animation: &boned::Animation) -> Self {
        let q_blocks = animation
            .bones
            .iter()
            .map(|bone| QBlock {
                frames: bone
                    .q_frames
                    .iter()
                    .map(|frame| QFrame::from_boned(frame))
                    .collect(),
            })
            .collect();

        let t_blocks = animation
            .bones
            .iter()
            .map(|bone| TBlock {
                frames: bone
                    .t_frames
                    .iter()
                    .map(|frame| TFrame::from_boned(frame))
                    .collect(),
            })
            .collect();

        let num_bones = animation.bones.len();

        // XXX (ellie): Maybe add a `.recalculate_header()` method or similar?

        // Will be recalculated when write is called
        let header = Header {
            version: HEADER_VERSION,
            flags: HeaderFlags::PRE_ROTATED_ROOT
                | HeaderFlags::COMPRESSED_TIME
                | HeaderFlags::USE_COMPRESS_TABLE
                | HeaderFlags::KEEP_MISSING_BONE_POS,
            duration: animation.duration,
            num_bones: num_bones as u32,
            num_q_keys: 0,
            num_t_keys: 0,
            num_custom_anim_keys: 0,
            q_alloc_size: 0,
            t_alloc_size: 0,
            q_block_sizes: vec![0; num_bones],
            t_block_sizes: vec![0; num_bones],
        };

        Self {
            header,
            q_blocks,
            t_blocks,
        }
    }
}
