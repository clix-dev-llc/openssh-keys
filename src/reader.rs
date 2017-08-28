//! reader
//!
//! this crate provides a struct for reading bytes in the OpenSSH public key format.

use errors::*;

use byteorder::{BigEndian, ByteOrder};

pub struct Reader<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> Reader<'a> {
    pub fn new(data: &[u8]) -> Reader {
        Reader {
            data: data,
            offset: 0,
        }
    }

    pub fn peek_int(&mut self) -> Result<u32> {
        let cur = &self.data[self.offset..];
        if cur.len() < 4 {
            return Err(ErrorKind::InvalidFormat.into());
        }
        Ok(BigEndian::read_u32(&cur[..4]))
    }

    pub fn read_int(&mut self) -> Result<u32> {
        let val = self.peek_int()?;
        self.offset += 4;
        Ok(val)
    }

    pub fn read_string(&mut self) -> Result<&'a str> {
        ::std::str::from_utf8(self.read_bytes()?)
            .chain_err(|| ErrorKind::InvalidFormat)
    }

    pub fn read_mpint(&mut self) -> Result<&'a [u8]> {
        // mpints might have an extra byte of zeros at the start
        let bytes = self.read_bytes()?;
        if bytes[0] == 0 {
            Ok(&bytes[1..])
        } else {
            Ok(bytes)
        }
    }

    pub fn read_bytes(&mut self) -> Result<&'a [u8]> {
        let cur = &self.data[self.offset..];
        let len = self.peek_int()? as usize;
        if cur.len() < len + 4 {
            return Err(ErrorKind::InvalidFormat.into());
        }
        self.offset += len + 4;
        Ok(&cur[4..len+4])
    }
}

