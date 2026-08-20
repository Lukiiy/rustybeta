use std::io::{Write, Result, Error, ErrorKind};

pub struct PacketWriter<'a, W: Write> {
    writer: &'a mut W,
}

impl<'a, W: Write> PacketWriter<'a, W> {
    pub fn new(writer: &'a mut W) -> Self {
        Self { writer }
    }

    pub fn write_bytes(&mut self, data: &[u8]) -> Result<()> {
        self.writer.write_all(data)
    }

    pub fn write_u8(&mut self, v: u8) -> Result<()> {
        self.writer.write_all(&[v])
    }

    pub fn write_bool(&mut self, v: bool) -> Result<()> {
        self.write_u8(v as u8)
    }

    pub fn write_i16(&mut self, v: i16) -> Result<()> {
        self.writer.write_all(&v.to_be_bytes())
    }

    pub fn write_i32(&mut self, v: i32) -> Result<()> {
        self.writer.write_all(&v.to_be_bytes())
    }

    pub fn write_i64(&mut self, v: i64) -> Result<()> {
        self.writer.write_all(&v.to_be_bytes())
    }

    pub fn write_f32(&mut self, v: f32) -> Result<()> {
        self.write_i32(v.to_bits() as i32)
    }

    pub fn write_f64(&mut self, v: f64) -> Result<()> {
        self.write_i64(v.to_bits() as i64)
    }

    pub fn write_string(&mut self, v: &str) -> Result<()> {
        let utf16: Vec<u16> = v.encode_utf16().collect();

        if utf16.len() > i16::MAX as usize {
            return Err(Error::new(ErrorKind::InvalidInput, "string too long"));
        }

        self.write_i16(utf16.len() as i16)?;

        for unit in utf16 {
            self.writer.write_all(&unit.to_be_bytes())?;
        }

        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}