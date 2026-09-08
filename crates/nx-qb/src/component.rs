use std::{
    fmt::{self, Debug},
    io::Write,
};

use byteorder::{LE, ReadBytesExt, WriteBytesExt};
use nx_common::Reader;

use crate::Error;
use crate::id::Id;
use crate::structure::Structure;

const CHECKSUM_LOOKUP_MASK_8: u8 = 1 << 7;
const CHECKSUM_LOOKUP_MASK_16: u8 = 1 << 6;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Kind {
    None,
    Integer,
    Float,
    String,
    LocalString,
    Pair,
    Vector,
    QScript,
    CFunction,
    MemberFunction,
    Structure,
    StructurePointer,
    Array,
    Name,
    I8,
    I16,
    U8,
    U16,
    ZeroInt,
    ZeroFloat,
}

impl TryFrom<u8> for Kind {
    type Error = Error;

    fn try_from(v: u8) -> Result<Self, Error> {
        match v {
            0 => Ok(Kind::None),
            1 => Ok(Kind::Integer),
            2 => Ok(Kind::Float),
            3 => Ok(Kind::String),
            4 => Ok(Kind::LocalString),
            5 => Ok(Kind::Pair),
            6 => Ok(Kind::Vector),
            7 => Ok(Kind::QScript),
            8 => Ok(Kind::CFunction),
            9 => Ok(Kind::MemberFunction),
            0xA => Ok(Kind::Structure),
            0xB => Ok(Kind::StructurePointer),
            0xC => Ok(Kind::Array),
            0xD => Ok(Kind::Name),
            0xE => Ok(Kind::I8),
            0xF => Ok(Kind::I16),
            0x10 => Ok(Kind::U8),
            0x11 => Ok(Kind::U16),
            0x12 => Ok(Kind::ZeroInt),
            0x13 => Ok(Kind::ZeroFloat),
            _ => Err(Error::InvalidComponentType(v)),
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Value {
    None,
    U8(u8),
    U16(u16),
    I8(i8),
    I16(i16),
    I32(i32),
    F32(f32),
    ZeroInt,
    ZeroFloat,
    String(Vec<u8>),
    Pair(f32, f32),
    Vector(f32, f32, f32),
    Structure(Box<Structure>),
    Array(Kind, Vec<Value>),
    Name(u32),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Value {
    pub fn read(reader: &mut impl Reader, kind: Kind) -> Result<Value, Error> {
        Ok(match kind {
            Kind::None => Value::None,
            Kind::Integer => Value::I32(reader.read_i32::<LE>()?),
            Kind::Float => Value::F32(reader.read_f32::<LE>()?),
            Kind::String | Kind::LocalString => {
                let mut bytes = vec![];

                while {
                    // do:
                    // Read character
                    let byte = reader.read_u8()?;

                    // while:
                    if byte != 0 {
                        bytes.push(byte);
                        true
                    } else {
                        false
                    }
                } {}

                Value::String(bytes)
            }
            // https://doc.rust-lang.org/reference/expressions.html#evaluation-order-of-operands
            Kind::Pair => Value::Pair(reader.read_f32::<LE>()?, reader.read_f32::<LE>()?),
            Kind::Vector => Value::Vector(
                reader.read_f32::<LE>()?,
                reader.read_f32::<LE>()?,
                reader.read_f32::<LE>()?,
            ),
            Kind::Structure => Value::Structure(Box::new(Structure::read(reader)?)),
            Kind::Array => {
                // TODO this is code reuse w/ Symbol::deserialize
                let type_byte = reader.read_u8()?;
                let kind = Kind::try_from(type_byte)?;
                let len = reader.read_u16::<LE>()?;

                let mut elements = vec![];
                for _ in 0..len {
                    elements.push(Value::read(reader, kind)?)
                }

                Value::Array(kind, elements)
            }
            Kind::Name => Value::Name(reader.read_u32::<LE>()?),
            Kind::I8 => Value::I8(reader.read_i8()?),
            Kind::I16 => Value::I16(reader.read_i16::<LE>()?),
            Kind::U8 => Value::U8(reader.read_u8()?),
            Kind::U16 => Value::U16(reader.read_u16::<LE>()?),
            Kind::ZeroInt => Value::ZeroInt,
            Kind::ZeroFloat => Value::ZeroFloat,
            _ => {
                return Err(Error::NotImplemented(format!(
                    "deserializing symbol type {:?}",
                    kind
                )));
            }
        })
    }
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        match self {
            Value::U8(val) => writer.write_u8(*val)?,
            Value::U16(val) => writer.write_u16::<LE>(*val)?,
            Value::I8(val) => writer.write_i8(*val)?,
            Value::I16(val) => writer.write_i16::<LE>(*val)?,
            Value::I32(val) => writer.write_i32::<LE>(*val)?,
            Value::F32(val) => writer.write_f32::<LE>(*val)?,
            Value::String(val) => {
                writer.write_all(val)?;
                // null terminator
                writer.write_u8(0)?;
            }
            Value::Pair(a, b) => {
                writer.write_f32::<LE>(*a)?;
                writer.write_f32::<LE>(*b)?;
            }
            Value::Vector(a, b, c) => {
                writer.write_f32::<LE>(*a)?;
                writer.write_f32::<LE>(*b)?;
                writer.write_f32::<LE>(*c)?;
            }
            Value::Structure(structure) => structure.write(writer)?,
            Value::Array(kind, values) => {
                writer.write_u8(*kind as u8)?;
                writer.write_u16::<LE>(values.len() as u16)?;
                for value in values.iter() {
                    value.write(writer)?;
                }
            }
            Value::Name(val) => writer.write_u32::<LE>(*val)?,
            // no value is written for these types
            Value::None | Value::ZeroFloat | Value::ZeroInt => (),
        }
        Ok(())
    }

    pub fn try_as_structure(&self) -> Result<&Structure, Error> {
        match self {
            Value::Structure(value) => Ok(value),
            value => Err(Error::ExpectedValueType(
                "Structure".to_string(),
                value.clone(),
            )),
        }
    }

    pub fn try_as_structure_mut(&mut self) -> Result<&mut Structure, Error> {
        match self {
            Value::Structure(value) => Ok(value),
            value => Err(Error::ExpectedValueType(
                "Structure".to_string(),
                value.clone(),
            )),
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Component {
    pub kind: Kind,
    pub id: Id,
    pub value: Value,
}

impl Component {
    pub fn none() -> Self {
        Self {
            kind: Kind::None,
            id: Id::None,
            value: Value::None,
        }
    }

    pub fn structure(id: Id, structure: Box<Structure>) -> Self {
        Self {
            kind: Kind::Structure,
            id,
            value: Value::Structure(structure),
        }
    }

    pub fn read(reader: &mut impl Reader) -> Result<Self, Error> {
        let type_byte = reader.read_u8()?;

        let use_lookup_8 = (type_byte & CHECKSUM_LOOKUP_MASK_8) != 0;
        let use_lookup_16 = (type_byte & CHECKSUM_LOOKUP_MASK_16) != 0;

        // Error if both checksum bits are set
        (!(use_lookup_8 && use_lookup_16))
            .then_some(())
            .ok_or(Error::BothChecksumBits(type_byte))?;

        // 8-bit / 16-bit mask in bits 6/7
        let type_byte_masked = type_byte & !(CHECKSUM_LOOKUP_MASK_8 | CHECKSUM_LOOKUP_MASK_16);

        let kind = Kind::try_from(type_byte_masked)?;

        let id = Id::read(reader, kind, use_lookup_8, use_lookup_16)?;

        let value = Value::read(reader, kind)?;
        let symbol = Component { kind, id, value };

        Ok(symbol)
    }

    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        let type_byte = self.kind as u8;

        let type_byte_with_checksum_bits = match self.id {
            Id::Compress16(_) => type_byte | CHECKSUM_LOOKUP_MASK_16,
            Id::Compress8(_) => type_byte | CHECKSUM_LOOKUP_MASK_8,
            _ => type_byte,
        };

        writer.write_u8(type_byte_with_checksum_bits)?;
        self.id.write(writer)?;
        self.value.write(writer)?;

        Ok(())
    }
}
