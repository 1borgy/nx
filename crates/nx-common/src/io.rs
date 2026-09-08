use std::{
    error::Error,
    fs,
    io::{self, Read, Seek, Write},
    path::Path,
};

pub trait Reader: Read + Seek {}
pub trait Writer: Write + Seek {}

impl<T> Reader for T where T: Read + Seek {}
impl<T> Writer for T where T: Write + Seek {}

pub trait Readable
where
    Self: Sized,
{
    type Error: Error + From<io::Error> + Send + Sync + 'static;
    type ReadContext: Send + Sync + Clone + 'static;

    fn read(reader: &mut impl Reader, ctx: &mut Self::ReadContext) -> Result<Self, Self::Error>;

    fn read_file(path: impl AsRef<Path>, ctx: &mut Self::ReadContext) -> Result<Self, Self::Error> {
        let reader = &mut io::BufReader::new(fs::File::open(path.as_ref())?);
        Ok(Self::read(reader, ctx)?)
    }
}

pub trait Writable
where
    Self: Sized,
{
    type Error: Error + From<io::Error> + Send + Sync + 'static;
    type WriteContext: Send + Sync + Clone + 'static;

    fn write(
        &self,
        writer: &mut impl Writer,
        ctx: &mut Self::WriteContext,
    ) -> Result<(), Self::Error>;

    fn write_file(
        &self,
        path: impl AsRef<Path>,
        ctx: &mut Self::WriteContext,
    ) -> Result<(), Self::Error> {
        let writer = &mut io::BufWriter::new(fs::File::create(path.as_ref())?);
        Ok(self.write(writer, ctx)?)
    }
}
