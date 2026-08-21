use std::io::{Read, Result, Error, ErrorKind};

pub struct PacketReader<'a, R: Read> {
    reader: &'a mut R
}

impl<'a, R: Read> PacketReader<'a, R> {
    pub fn new(reader: &'a mut R) -> Self {
        Self { reader }
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];

        self.reader.read_exact(&mut buf)?;

        Ok(buf[0])
    }

    pub fn read_i16(&mut self) -> Result<i16> {
        let mut buf = [0u8; 2];

        self.reader.read_exact(&mut buf)?;

        Ok(i16::from_be_bytes(buf))
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        let mut buf = [0u8; 4];

        self.reader.read_exact(&mut buf)?;

        Ok(i32::from_be_bytes(buf))
    }

    pub fn read_i64(&mut self) -> Result<i64> {
        let mut buf = [0u8; 8];

        self.reader.read_exact(&mut buf)?;

        Ok(i64::from_be_bytes(buf))
    }

    pub fn read_f32(&mut self) -> Result<f32> {
        Ok(f32::from_bits(self.read_i32()? as u32))
    }

    pub fn read_f64(&mut self) -> Result<f64> {
        Ok(f64::from_bits(self.read_i64()? as u64))
    }

    pub fn read_string(&mut self) -> Result<String> {
        let length = self.read_i16()? as usize;
        let mut buf = vec![0u8; length * 2];

        self.reader.read_exact(&mut buf)?;

        let units: Vec<u16> = buf.chunks_exact(2).map(|c| u16::from_be_bytes([c[0], c[1]])).collect();

        String::from_utf16(&units).map_err(|_| Error::new(ErrorKind::InvalidData, "invalid UTF-16 string"))
    }

    pub fn read_bool(&mut self) -> Result<bool> {
        Ok(self.read_u8()? != 0)
    }
}