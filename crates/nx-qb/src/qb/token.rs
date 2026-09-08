use byteorder::{LE, ReadBytesExt, WriteBytesExt};
use encoding_rs::WINDOWS_1252;
use nx_common::Reader;

use crate::{Error, qb::symbols::SymbolTable};

#[derive(Debug, Clone)]
#[allow(unused)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Token {
    Terminator,                    // 0x0
    Newline,                       // 0x1
    NewlineDebug(i32),             // 0x2
    LeftBrace,                     // 0x3
    RightBrace,                    // 0x4
    LeftBracket,                   // 0x5
    RightBracket,                  // 0x6
    Equals,                        // 0x7
    Property,                      // 0x8
    Comma,                         // 0x9
    Subtract,                      // 0xa
    Add,                           // 0xb
    Divide,                        // 0xc
    Multiply,                      // 0xd
    LeftParen,                     // 0xe
    RightParen,                    // 0xf
    Unk0x10,                       // 0x10
    Unk0x11,                       // 0x11
    LessThan,                      // 0x12
    LessThanEq,                    // 0x13
    GreaterThan,                   // 0x14
    GreaterThanEq,                 // 0x15
    Symbol(u32),                   // 0x16
    Integer(i32),                  // 0x17
    Unk0x18,                       // 0x18
    Unk0x19,                       // 0x19
    Float(f32),                    // 0x1a
    String(Vec<u8>),               // 0x1b
    Param(Vec<u8>),                // 0x1c
    Unk0x1d,                       // 0x1d
    Vec3(f32, f32, f32),           // 0x1e
    Vec2(f32, f32),                // 0x1f
    Begin,                         // 0x20
    Repeat,                        // 0x21
    Break,                         // 0x22
    Script,                        // 0x23
    EndScript,                     // 0x24
    If,                            // 0x25
    Else,                          // 0x26
    ElseIf,                        // 0x27
    EndIf,                         // 0x28
    Return,                        // 0x29
    Unk0x2a,                       // 0x2a
    SymbolDef(u32, Vec<u8>),       // 0x2b
    GlobalAll,                     // 0x2c
    Global,                        // 0x2d
    Jump(i32),                     // 0x2e
    Random(u32, Vec<u32>),         // 0x2f
    RandomRange,                   // 0x30
    Unk0x31,                       // 0x31
    Or,                            // 0x32
    And,                           // 0x33
    Xor,                           // 0x34
    ShiftLeft,                     // 0x35
    ShiftRight,                    // 0x36
    Random2(u32, Vec<u32>),        // 0x37
    RandomRange2,                  // 0x38
    Not,                           // 0x39
    Unk0x3a,                       // 0x3a
    Unk0x3b,                       // 0x3b
    Switch,                        // 0x3c
    EndSwitch,                     // 0x3d
    Case,                          // 0x3e
    Default,                       // 0x3f
    RandomNoRepeat(u32, Vec<u32>), // 0x40
    RandomPermute(i32, Vec<i32>),  // 0x41
    Member,                        // 0x42
    CFunc,                         // 0x43
    MemberFunc,                    // 0x44
    Unk0x45,                       // 0x45
    Unk0x46,                       // 0x46
    If2(i16),                      // 0x47
    Else2(i16),                    // 0x48
    EndSwitch2(i16),               // 0x49
    Unk0x4a,                       // 0x4a
    Unk0x4b,                       // 0x4b
    Unk0x4c,                       // 0x4c
    Unk0x4d,                       // 0x4d
    Unk0x4e,                       // 0x4e
    Unk0x4f,                       // 0x4f
    Reserved,                      // 0xff
}

impl Token {
    fn read_varlen_string(reader: &mut impl Reader) -> Result<Vec<u8>, Error> {
        log::debug!(
            "reading null-terminated string at position {:#08x}",
            reader.stream_position()?
        );
        let mut bytes = vec![];

        while {
            // do: read byte
            let byte = reader.read_u8()?;

            // while: byte is not 0
            if byte != 0 {
                bytes.push(byte);
                true
            } else {
                false
            }
        } {}

        Ok(bytes)
    }

    fn read_fixed_string(reader: &mut impl Reader) -> Result<Vec<u8>, Error> {
        // len-1 to ignore null terminator
        let len = reader.read_i32::<LE>()? - 1;

        log::debug!(
            "reading fixed string at position {:#08x}, len={}",
            reader.stream_position()? - 4,
            len
        );

        let mut bytes = Vec::with_capacity(len as usize);

        for _ in 0..(len) {
            let byte = reader.read_u8()?;
            bytes.push(byte);
        }
        // Consume null terminator
        reader.read_u8()?;

        Ok(bytes)
    }

    fn read_random(reader: &mut impl Reader) -> Result<(u32, Vec<u32>), Error> {
        let count = reader.read_u32::<LE>()?;

        // TODO: add ReadContext for game
        // let mut weights = Vec::with_capacity(num_items as usize);
        // for _ in 0..num_items {
        //     weights.push(reader.read_u16::<LE>()?);
        // }

        let mut offsets = Vec::with_capacity(count as usize);
        for _ in 0..count {
            offsets.push(reader.read_u32::<LE>()?);
        }

        Ok((count, offsets))
    }

    pub fn read(reader: &mut impl Reader) -> Result<Self, Error> {
        match reader.read_u8()? {
            0x0 => Ok(Self::Terminator),
            0x1 => Ok(Self::Newline),
            0x2 => Ok(Self::NewlineDebug(reader.read_i32::<LE>()?)),
            0x3 => Ok(Self::LeftBrace),
            0x4 => Ok(Self::RightBrace),
            0x5 => Ok(Self::LeftBracket),
            0x6 => Ok(Self::RightBracket),
            0x7 => Ok(Self::Equals),
            0x8 => Ok(Self::Property),
            0x9 => Ok(Self::Comma),
            0xa => Ok(Self::Subtract),
            0xb => Ok(Self::Add),
            0xc => Ok(Self::Divide),
            0xd => Ok(Self::Multiply),
            0xe => Ok(Self::LeftParen),
            0xf => Ok(Self::RightParen),
            0x10 => Ok(Self::Unk0x10),
            0x11 => Ok(Self::Unk0x11),
            0x12 => Ok(Self::LessThan),
            0x13 => Ok(Self::LessThanEq),
            0x14 => Ok(Self::GreaterThan),
            0x15 => Ok(Self::GreaterThanEq),
            0x16 => Ok(Self::Symbol(reader.read_u32::<LE>()?)),
            0x17 => Ok(Self::Integer(reader.read_i32::<LE>()?)),
            0x18 => Ok(Self::Unk0x18),
            0x19 => Ok(Self::Unk0x19),
            0x1a => Ok(Self::Float(reader.read_f32::<LE>()?)),
            0x1b => {
                let bytes = Self::read_fixed_string(reader)?;
                Ok(Self::String(bytes))
            }
            0x1c => {
                let bytes = Self::read_fixed_string(reader)?;
                Ok(Self::Param(bytes))
            }
            0x1d => Ok(Self::Unk0x1d),
            0x1e => {
                let x = reader.read_f32::<LE>()?;
                let y = reader.read_f32::<LE>()?;
                let z = reader.read_f32::<LE>()?;
                Ok(Self::Vec3(x, y, z))
            }
            0x1f => {
                let x = reader.read_f32::<LE>()?;
                let y = reader.read_f32::<LE>()?;
                Ok(Self::Vec2(x, y))
            }
            0x20 => Ok(Self::Begin),
            0x21 => Ok(Self::Repeat),
            0x22 => Ok(Self::Break),
            0x23 => Ok(Self::Script),
            0x24 => Ok(Self::EndScript),
            0x25 => Ok(Self::If),
            0x26 => Ok(Self::Else),
            0x27 => Ok(Self::ElseIf),
            0x28 => Ok(Self::EndIf),
            0x29 => Ok(Self::Return),
            0x2a => Ok(Self::Unk0x2a),
            0x2b => {
                let checksum = reader.read_u32::<LE>()?;
                let bytes = Self::read_varlen_string(reader)?;
                Ok(Self::SymbolDef(checksum, bytes))
            }
            0x2c => Ok(Self::GlobalAll),
            0x2d => Ok(Self::Global),
            0x2e => Ok(Self::Jump(reader.read_i32::<LE>()?)),
            0x2f => {
                let (count, offsets) = Self::read_random(reader)?;
                Ok(Self::Random(count, offsets))
            }
            0x30 => Ok(Self::RandomRange),
            0x31 => Ok(Self::Unk0x31),
            0x32 => Ok(Self::Or),
            0x33 => Ok(Self::And),
            0x34 => Ok(Self::Xor),
            0x35 => Ok(Self::ShiftLeft),
            0x36 => Ok(Self::ShiftRight),
            0x37 => {
                let (count, offsets) = Self::read_random(reader)?;
                Ok(Self::Random2(count, offsets))
            }
            0x38 => Ok(Self::RandomRange2),
            0x39 => Ok(Self::Not),
            0x3a => Ok(Self::Unk0x3a),
            0x3b => Ok(Self::Unk0x3b),
            0x3c => Ok(Self::Switch),
            0x3d => Ok(Self::EndSwitch),
            0x3e => Ok(Self::Case),
            0x3f => Ok(Self::Default),
            0x40 => {
                let (count, offsets) = Self::read_random(reader)?;
                Ok(Self::RandomNoRepeat(count, offsets))
            }
            0x41 => {
                let count = reader.read_i32::<LE>()?;
                let mut values = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    values.push(reader.read_i32::<LE>()?);
                }

                Ok(Self::RandomPermute(count, values))
            }
            0x42 => Ok(Self::Member),
            0x43 => Ok(Self::CFunc),
            0x44 => Ok(Self::MemberFunc),
            0x45 => Ok(Self::Unk0x45),
            0x46 => Ok(Self::Unk0x46),
            0x47 => Ok(Self::If2(reader.read_i16::<LE>()?)),
            0x48 => Ok(Self::Else2(reader.read_i16::<LE>()?)),
            0x49 => Ok(Self::EndSwitch2(reader.read_i16::<LE>()?)),
            0x4a => Ok(Self::Unk0x4a),
            0x4b => Ok(Self::Unk0x4b),
            0x4c => Ok(Self::Unk0x4c),
            0x4d => Ok(Self::Unk0x4d),
            0x4e => Ok(Self::Unk0x4e),
            0x4f => Ok(Self::Unk0x4f),
            0xff => Ok(Self::Reserved),
            v => Err(Error::InvalidTokenType(v)),
        }
    }

    fn write_varlen_string(
        writer: &mut impl nx_common::Writer,
        bytes: &Vec<u8>,
    ) -> Result<(), Error> {
        for byte in bytes.iter() {
            writer.write_u8(*byte)?;
        }
        // Null terminator
        writer.write_u8(0)?;

        Ok(())
    }

    fn write_fixed_string(
        writer: &mut impl nx_common::Writer,
        bytes: &Vec<u8>,
    ) -> Result<(), Error> {
        let len = (bytes.len() + 1) as i32;
        writer.write_i32::<LE>(len)?;

        for byte in bytes.iter() {
            writer.write_u8(*byte)?;
        }
        // Null terminator
        writer.write_u8(0)?;

        Ok(())
    }

    fn write_random(
        writer: &mut impl nx_common::Writer,
        count: u32,
        offsets: &Vec<u32>,
    ) -> Result<(), Error> {
        writer.write_u32::<LE>(count)?;

        for offset in offsets.iter() {
            writer.write_u32::<LE>(*offset)?;
        }

        Ok(())
    }

    pub fn write(&self, writer: &mut impl nx_common::Writer) -> Result<(), Error> {
        match self {
            Self::Terminator => writer.write_u8(0x0)?,
            Self::Newline => writer.write_u8(0x1)?,
            Self::NewlineDebug(v) => {
                writer.write_u8(0x2)?;
                writer.write_i32::<LE>(*v)?;
            }
            Self::LeftBrace => writer.write_u8(0x3)?,
            Self::RightBrace => writer.write_u8(0x4)?,
            Self::LeftBracket => writer.write_u8(0x5)?,
            Self::RightBracket => writer.write_u8(0x6)?,
            Self::Equals => writer.write_u8(0x7)?,
            Self::Property => writer.write_u8(0x8)?,
            Self::Comma => writer.write_u8(0x9)?,
            Self::Subtract => writer.write_u8(0xa)?,
            Self::Add => writer.write_u8(0xb)?,
            Self::Divide => writer.write_u8(0xc)?,
            Self::Multiply => writer.write_u8(0xd)?,
            Self::LeftParen => writer.write_u8(0xe)?,
            Self::RightParen => writer.write_u8(0xf)?,
            Self::Unk0x10 => writer.write_u8(0x10)?,
            Self::Unk0x11 => writer.write_u8(0x11)?,
            Self::LessThan => writer.write_u8(0x12)?,
            Self::LessThanEq => writer.write_u8(0x13)?,
            Self::GreaterThan => writer.write_u8(0x14)?,
            Self::GreaterThanEq => writer.write_u8(0x15)?,
            Self::Symbol(v) => {
                writer.write_u8(0x16)?;
                writer.write_u32::<LE>(*v)?;
            }
            Self::Integer(v) => {
                writer.write_u8(0x17)?;
                writer.write_i32::<LE>(*v)?;
            }
            Self::Unk0x18 => writer.write_u8(0x18)?,
            Self::Unk0x19 => writer.write_u8(0x19)?,
            Self::Float(v) => {
                writer.write_u8(0x1a)?;
                writer.write_f32::<LE>(*v)?;
            }
            Self::String(bytes) => {
                writer.write_u8(0x1b)?;
                Self::write_fixed_string(writer, bytes)?;
            }
            Self::Param(bytes) => {
                writer.write_u8(0x1c)?;
                Self::write_fixed_string(writer, bytes)?;
            }
            Self::Unk0x1d => writer.write_u8(0x1d)?,
            Self::Vec3(x, y, z) => {
                writer.write_u8(0x1e)?;
                writer.write_f32::<LE>(*x)?;
                writer.write_f32::<LE>(*y)?;
                writer.write_f32::<LE>(*z)?;
            }
            Self::Vec2(x, y) => {
                writer.write_u8(0x1f)?;
                writer.write_f32::<LE>(*x)?;
                writer.write_f32::<LE>(*y)?;
            }
            Self::Begin => writer.write_u8(0x20)?,
            Self::Repeat => writer.write_u8(0x21)?,
            Self::Break => writer.write_u8(0x22)?,
            Self::Script => writer.write_u8(0x23)?,
            Self::EndScript => writer.write_u8(0x24)?,
            Self::If => writer.write_u8(0x25)?,
            Self::Else => writer.write_u8(0x26)?,
            Self::ElseIf => writer.write_u8(0x27)?,
            Self::EndIf => writer.write_u8(0x28)?,
            Self::Return => writer.write_u8(0x29)?,
            Self::Unk0x2a => writer.write_u8(0x2a)?,
            Self::SymbolDef(checksum, bytes) => {
                writer.write_u8(0x2b)?;
                writer.write_u32::<LE>(*checksum)?;
                Self::write_varlen_string(writer, bytes)?;
            }
            Self::GlobalAll => writer.write_u8(0x2c)?,
            Self::Global => writer.write_u8(0x2d)?,
            Self::Jump(v) => {
                writer.write_u8(0x2e)?;
                writer.write_i32::<LE>(*v)?;
            }
            Self::Random(count, offsets) => {
                writer.write_u8(0x2f)?;
                Self::write_random(writer, *count, offsets)?;
            }
            Self::RandomRange => writer.write_u8(0x30)?,
            Self::Unk0x31 => writer.write_u8(0x31)?,
            Self::Or => writer.write_u8(0x32)?,
            Self::And => writer.write_u8(0x33)?,
            Self::Xor => writer.write_u8(0x34)?,
            Self::ShiftLeft => writer.write_u8(0x35)?,
            Self::ShiftRight => writer.write_u8(0x36)?,
            Self::Random2(count, offsets) => {
                writer.write_u8(0x37)?;
                Self::write_random(writer, *count, offsets)?;
            }
            Self::RandomRange2 => writer.write_u8(0x38)?,
            Self::Not => writer.write_u8(0x39)?,
            Self::Unk0x3a => writer.write_u8(0x3a)?,
            Self::Unk0x3b => writer.write_u8(0x3b)?,
            Self::Switch => writer.write_u8(0x3c)?,
            Self::EndSwitch => writer.write_u8(0x3d)?,
            Self::Case => writer.write_u8(0x3e)?,
            Self::Default => writer.write_u8(0x3f)?,
            Self::RandomNoRepeat(count, offsets) => {
                writer.write_u8(0x40)?;
                Self::write_random(writer, *count, offsets)?;
            }
            Self::RandomPermute(count, items) => {
                writer.write_u8(0x41)?;
                writer.write_i32::<LE>(*count)?;
                for item in items.iter() {
                    writer.write_i32::<LE>(*item)?;
                }
            }
            Self::Member => writer.write_u8(0x42)?,
            Self::CFunc => writer.write_u8(0x43)?,
            Self::MemberFunc => writer.write_u8(0x44)?,
            Self::Unk0x45 => writer.write_u8(0x45)?,
            Self::Unk0x46 => writer.write_u8(0x46)?,
            Self::If2(v) => {
                writer.write_u8(0x47)?;
                writer.write_i16::<LE>(*v)?;
            }
            Self::Else2(v) => {
                writer.write_u8(0x48)?;
                writer.write_i16::<LE>(*v)?;
            }
            Self::EndSwitch2(v) => {
                writer.write_u8(0x49)?;
                writer.write_i16::<LE>(*v)?;
            }
            Self::Unk0x4a => writer.write_u8(0x4a)?,
            Self::Unk0x4b => writer.write_u8(0x4b)?,
            Self::Unk0x4c => writer.write_u8(0x4c)?,
            Self::Unk0x4d => writer.write_u8(0x4d)?,
            Self::Unk0x4e => writer.write_u8(0x4e)?,
            Self::Unk0x4f => writer.write_u8(0x4f)?,
            Self::Reserved => writer.write_u8(0xff)?,
        }
        Ok(())
    }

    pub fn to_string(&self, buffer: &mut String, symbol_table: &SymbolTable) {
        match self {
            Self::Newline => buffer.push('\n'),
            Self::NewlineDebug(_) => buffer.push('\n'),
            Self::LeftBrace => buffer.push('{'),
            Self::RightBrace => buffer.push('}'),
            Self::LeftBracket => buffer.push('['),
            Self::RightBracket => buffer.push(']'),
            Self::Equals => buffer.push('='),
            Self::Property => buffer.push('.'),
            Self::Comma => buffer.push(','),
            Self::Subtract => buffer.push('-'),
            Self::Add => buffer.push('+'),
            Self::Divide => buffer.push('/'),
            Self::Multiply => buffer.push('*'),
            Self::LeftParen => buffer.push('('),
            Self::RightParen => buffer.push(')'),
            Self::LessThan => buffer.push('<'),
            Self::LessThanEq => buffer.push_str("<="),
            Self::GreaterThan => buffer.push('>'),
            Self::GreaterThanEq => buffer.push_str(">="),
            Self::Symbol(checksum) => match symbol_table.get(*checksum) {
                Some(name) => buffer.push_str(name.as_str()),
                None => buffer.push_str(format!("{:#08x}", checksum).as_str()),
            },
            Self::Integer(v) => buffer.push_str(format!("{}", v).as_str()),
            Self::Float(v) => buffer.push_str(format!("{}", v).as_str()),
            Self::String(bytes) => {
                let (s, _, _) = WINDOWS_1252.decode(bytes);
                buffer.push_str(format!("\"{}\"", s).as_str())
            }
            Self::Param(bytes) => {
                let (s, _, _) = WINDOWS_1252.decode(bytes);
                buffer.push_str(format!("\"{}\"", s).as_str())
            }
            Self::Vec3(x, y, z) => buffer.push_str(format!("({}, {}, {})", x, y, z).as_str()),
            Self::Vec2(x, y) => buffer.push_str(format!("({}, {})", x, y).as_str()),
            Self::Begin => buffer.push_str("begin"),
            Self::Repeat => buffer.push_str("repeat"),
            Self::Break => buffer.push_str("break"),
            Self::Script => buffer.push_str("script"),
            Self::EndScript => buffer.push_str("endscript"),
            Self::If => buffer.push_str("if"),
            Self::Else => buffer.push_str("else"),
            Self::ElseIf => buffer.push_str("elseif"),
            Self::EndIf => buffer.push_str("endif"),
            Self::Return => buffer.push_str("return"),
            Self::GlobalAll => buffer.push_str("<...>"),
            Self::Jump(_) => buffer.push('@'),
            Self::Random(_, _) => buffer.push_str("SEOMTHIGN RANDOM IDK"),
            Self::RandomRange => buffer.push_str("SEOMTHIGN RANDOM IDK"),
            Self::Or => buffer.push_str("or"),
            Self::And => buffer.push_str("and"),
            Self::Xor => buffer.push_str("xor"),
            Self::ShiftLeft => buffer.push_str("shl"),
            Self::ShiftRight => buffer.push_str("shr"),
            Self::Random2(_, _) => todo!(),
            Self::RandomRange2 => todo!(),
            Self::Not => buffer.push_str("not"),
            Self::Switch => buffer.push_str("switch"),
            Self::EndSwitch => buffer.push_str("endswitch"),
            Self::Case => buffer.push_str("case"),
            Self::Default => buffer.push_str("default"),
            Self::RandomNoRepeat(_, _) => buffer.push_str("SEOMTHIGN RANDOM IDK"),
            Self::RandomPermute(_, _) => buffer.push_str("SEOMTHIGN RANDOM IDK"),
            Self::Member => buffer.push(':'),
            Self::If2(_) => buffer.push_str("if2"),
            Self::Else2(_) => buffer.push_str("else2"),
            Self::EndSwitch2(_) => buffer.push_str("endswitch2"),
            // Ignore
            Self::Terminator => {}
            Self::Global => {}
            Self::SymbolDef(_, _) => {}
            // Unimplemented
            Self::CFunc
            | Self::MemberFunc
            | Self::Unk0x10
            | Self::Unk0x11
            | Self::Unk0x18
            | Self::Unk0x19
            | Self::Unk0x1d
            | Self::Unk0x2a
            | Self::Unk0x31
            | Self::Unk0x3a
            | Self::Unk0x3b
            | Self::Unk0x45
            | Self::Unk0x46
            | Self::Unk0x4a
            | Self::Unk0x4b
            | Self::Unk0x4c
            | Self::Unk0x4d
            | Self::Unk0x4e
            | Self::Unk0x4f
            | Self::Reserved => {
                log::warn!("got unexpected token ({:?})", self);
            }
        }
        match self {
            Self::Newline
            | Self::NewlineDebug(_)
            | Self::Property
            | Self::Terminator
            | Self::Global
            | Self::SymbolDef(_, _)
            | Self::CFunc
            | Self::MemberFunc
            | Self::Unk0x10
            | Self::Unk0x11
            | Self::Unk0x18
            | Self::Unk0x19
            | Self::Unk0x1d
            | Self::Unk0x2a
            | Self::Unk0x31
            | Self::Unk0x3a
            | Self::Unk0x3b
            | Self::Unk0x45
            | Self::Unk0x46
            | Self::Unk0x4a
            | Self::Unk0x4b
            | Self::Unk0x4c
            | Self::Unk0x4d
            | Self::Unk0x4e
            | Self::Unk0x4f
            | Self::Reserved => {}
            _ => buffer.push(' '),
        }
    }
}
