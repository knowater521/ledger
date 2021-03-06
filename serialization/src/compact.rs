use basictype::hash::{Hash256, Hash32, Hash96};
use reader::Deserializable;
use reader::Error;
use reader::Reader;
use std::io;
use stream::Serializable;
use stream::Stream;

pub struct Compact(u64);

impl Compact {
    pub fn len(&self) -> u64 {
        self.0
    }
}

impl From<u8> for Compact {
    fn from(i: u8) -> Self {
        Compact(i as u64)
    }
}

impl From<u16> for Compact {
    fn from(i: u16) -> Self {
        Compact(i as u64)
    }
}

impl From<u32> for Compact {
    fn from(i: u32) -> Self {
        Compact(i as u64)
    }
}

impl From<u64> for Compact {
    fn from(i: u64) -> Self {
        Compact(i)
    }
}

impl From<usize> for Compact {
    fn from(i: usize) -> Self {
        Compact(i as u64)
    }
}

impl From<Compact> for usize {
    fn from(c: Compact) -> Self {
        c.0 as usize
    }
}

impl Serializable for Compact {
    fn serialize(&self, s: &mut Stream) {
        match self.0 {
            0...0xfc => {
                s.write(&(self.0 as u8));
            }
            0xfd...0xffff => {
                s.write(&0xfd);
                s.write(&(self.0 as u16));
            }
            0xffff...0xffff_ffff => {
                s.write(&0xfe);
                s.write(&(self.0 as u32));
            }
            _ => {
                s.write(&0xff);
                s.write(&(self.0 as u64));
            }
        }
    }

    fn serialized_size(&self) -> usize {
        match self.0 {
            0...0xfc => 1,
            0xfd...0xffff => 3,
            0xffff...0xffff_ffff => 5,
            _ => 9
        }
    }
}

impl Deserializable for Compact {
    fn deserialize<T>(reader: &mut Reader<T>) -> Result<Self, Error> where Self: Sized, T: io::Read {
        let compact = match reader.read::<u8>()? {
            i @ 0...0xfc => i.into(),
            0xfd => reader.read::<u16>()?.into(),
            0xfe => reader.read::<u32>()?.into(),
            _ => reader.read::<u64>()?.into(),
        };

        Ok(compact)
    }
}