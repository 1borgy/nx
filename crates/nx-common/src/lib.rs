use std::io::{self, Read, Seek, Write};

pub trait Reader: Read + Seek {}
pub trait Writer: Write + Seek {}

impl<T> Reader for T where T: Read + Seek {}
impl<T> Writer for T where T: Write + Seek {}

pub trait Readable
where
    Self: Sized,
{
    type Error;
    fn read(reader: &mut impl Reader) -> Result<Self, Self::Error>;
}

pub trait Writeable {
    type Error;
    fn read(&self, writer: &mut impl Writer) -> Result<(), Self::Error>;
}
