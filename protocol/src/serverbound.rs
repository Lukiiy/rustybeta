use std::io::{Read, Result, Error, ErrorKind};

use super::PacketReader;

#[derive(Debug)]
pub struct Handshake {
    pub username: String
}

#[derive(Debug)]
pub struct Login {
    pub protocol_version: i32,
    pub username: String,
    pub world_seed: i64,
    pub dimension: i8
}

pub fn read_handshake<R: Read>(reader: &mut PacketReader<R>) -> Result<Handshake> {
    let packet_id = reader.read_u8()?;

    if packet_id != 0x02 {
        return Err(Error::new(ErrorKind::InvalidData, format!("Expected handshake, got packet id 0x{packet_id:02X}")));
    }

    Ok(Handshake {
        username: reader.read_string()?
    })
}

pub fn read_login<R: Read>(reader: &mut PacketReader<R>) -> Result<Login> {
    let packet_id = reader.read_u8()?;

    if packet_id != 0x01 {
        return Err(Error::new(ErrorKind::InvalidData, format!("Expected login (0x01), got 0x{packet_id:02X}")));
    }

    Ok(Login {
        protocol_version: reader.read_i32()?,
        username: reader.read_string()?,
        world_seed: reader.read_i64()?,
        dimension: reader.read_u8()? as i8
    })
}