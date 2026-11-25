use std::io::{Read, Result};

use crate::{PacketReader, codec::{Decode, Packet}};

#[derive(Debug, Clone)]
pub struct Handshake {
    pub username: String
}

impl Packet for Handshake {
    const ID: u8 = 0x02;
}

impl Decode for Handshake {
    fn decode<R: Read>(r: &mut PacketReader<R>) -> Result<Self> {
        Ok(Self { username: r.read_string()? })
    }
}


#[derive(Debug, Clone)]
pub struct Login {
    pub protocol: i32,
    pub username: String,
    pub seed: i64,
    pub dimension: i8
}

impl Packet for Login {
    const ID: u8 = 0x01;
}

impl Decode for Login {
    fn decode<R: Read>(r: &mut PacketReader<R>) -> Result<Self> {
        Ok(Self {
            protocol: r.read_i32()?,
            username: r.read_string()?,
            seed: r.read_i64()?,
            dimension: r.read_u8()? as i8
        })
    }
}

#[derive(Debug, Clone)] /// LEGACY ping for the server list
pub struct ServerListPing;

impl Packet for ServerListPing {
    const ID: u8 = 0xFE;
}

impl Decode for ServerListPing {
    fn decode<R: Read>(_: &mut PacketReader<R>) -> Result<Self> { Ok(Self) }
}