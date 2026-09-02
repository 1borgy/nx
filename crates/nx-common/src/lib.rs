use std::io::{Read, Seek, Write};

pub trait Readable: Read + Seek {}
pub trait Writable: Write + Seek {}

impl<T> Readable for T where T: Read + Seek {}
impl<T> Writable for T where T: Write + Seek {}
