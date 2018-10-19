use byteorder::ReadBytesExt;
use std::io;
use std::marker;


pub struct Reader<T> {
    inner: T
}

pub enum Error {
    ReadDataErr
}

pub trait Deserializable {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read;
}


impl<R> Reader<R> where R: io::Read {
    pub fn from_buffer(buffer: R) -> Self {
        Reader {
            inner: buffer
        }
    }

    pub fn read<T>(&mut self) -> Result<T, Error> where T: Deserializable {
        T::deserialize(self)
    }


}


impl Deserializable for u8 {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read {
        Ok(reader.read_u8().unwrap())
    }
}

pub fn deserialize<R, T>(buffer: R) -> Result<T, Error> where R: io::Read, T: Deserializable {
    let mut reader = Reader::from_buffer(buffer);
    reader.read::<T>()
}


// 为了得到 ReadBytesExt 的能力
impl<T> io::Read for Reader<T> where T: io::Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        io::Read::read(&mut self.inner, buf)
    }
}

impl<'a> Reader<&'a [u8]> {
    fn from_bytes(bytes: &'a [u8]) -> Self {
        Reader {
            inner: bytes
        }
    }
}