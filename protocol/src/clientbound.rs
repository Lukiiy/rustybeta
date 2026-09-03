use std::io::{Write, Result};

use super::PacketWriter;
use world::Chunk;
use world::{SIZE_X, SIZE_Y, SIZE_Z};
use world::Position;
use utils::math;

/// I still don't really know if this will be kept as a giant enum just for clientbound packets, but I'll stick to it for now. Why? They're simpler than serverbound ones and i also don't want to mess a lot more with memory or possible missmatching IDs and structs;
#[derive(Debug, Clone)]
pub enum ClientboundPacket {
    KeepAlive,
    Login {
        entity_id: i32,
        world_seed: i64,
        dimension: i8
    },
    Handshake,
    ChatMessage {
        message: String
    },
    SetSpawnPosition {
        x: i32,
        y: i32,
        z: i32
    },
    PlayerPositionLook {
        position: Position,
        stance: f64
    },
    SpawnPlayer {
        entity_id: i32,
        username: String,
        position: Position,
        current_item: i16
    },
    DestroyEntity {
        entity_id: i32
    },
    EntityTeleport {
        entity_id: i32,
        position: Position
    },
    PlayerCrouch {
        entity_id: i32,
        sneaking: bool
    },
    PreChunk {
        chunk_x: i32,
        chunk_z: i32,
        mode: bool
    },
    Chunk {
        chunk_x: i32,
        chunk_z: i32,
        data: Vec<u8>
    },
    Kick {
        reason: String
    }
}

impl ClientboundPacket {
    pub fn player_pos_no_stance(position: Position) -> Self {
        Self::PlayerPositionLook {
            stance: position.y + 1.62,
            position
        }
    }

    pub fn from_chunk(chunk: &Chunk) -> Result<Self> {
        Ok(Self::Chunk {
            chunk_x: chunk.x,
            chunk_z: chunk.z,
            data: chunk.compressed()?
        })
    }

    /// serializes data to writer
    pub fn write<W: Write>(&self, writer: &mut PacketWriter<W>) -> Result<()> {
        match self {
            Self::KeepAlive => {
                writer.write_u8(0x00)?;
            }

            Self::Login { entity_id, world_seed, dimension } => {
                writer.write_u8(0x01)?;
                writer.write_i32(*entity_id)?;
                writer.write_string("")?;
                writer.write_i64(*world_seed)?;
                writer.write_u8(*dimension as u8)?;
            }

            Self::Handshake => {
                writer.write_u8(0x02)?;
                writer.write_string("-")?;
            }

            Self::ChatMessage { message } => {
                writer.write_u8(0x03)?;
                writer.write_string(message)?;
            }

            Self::SetSpawnPosition { x, y, z } => {
                writer.write_u8(0x06)?;
                writer.write_i32(*x)?;
                writer.write_i32(*y)?;
                writer.write_i32(*z)?;
            }

            Self::PlayerPositionLook { position, stance } => {
                writer.write_u8(0x0D)?;
                writer.write_f64(position.x)?;
                writer.write_f64(*stance)?;
                writer.write_f64(position.y)?;
                writer.write_f64(position.z)?;
                writer.write_f32(position.yaw)?;
                writer.write_f32(position.pitch)?;
                writer.write_bool(position.on_ground)?;
            }

            Self::SpawnPlayer { entity_id, username, position, current_item } => {
                writer.write_u8(0x14)?;
                writer.write_i32(*entity_id)?;
                writer.write_string(username)?;
                writer.write_i32((position.x * 32.0) as i32)?;
                writer.write_i32((position.y * 32.0) as i32)?;
                writer.write_i32((position.z * 32.0) as i32)?;
                writer.write_u8(math::angle_byte(position.yaw))?;
                writer.write_u8(math::angle_byte(position.pitch))?;
                writer.write_i16(*current_item)?;
            }

            Self::DestroyEntity { entity_id } => {
                writer.write_u8(0x1D)?;
                writer.write_i32(*entity_id)?;
            }

            Self::EntityTeleport { entity_id, position } => {
                writer.write_u8(0x22)?;
                writer.write_i32(*entity_id)?;
                writer.write_i32((position.x * 32.0) as i32)?;
                writer.write_i32((position.y * 32.0) as i32)?;
                writer.write_i32((position.z * 32.0) as i32)?;
                writer.write_u8(math::angle_byte(position.yaw))?;
                writer.write_u8(math::angle_byte(position.pitch))?;
            }

            Self::PlayerCrouch { entity_id, sneaking } => {
                writer.write_u8(0x28)?;
                writer.write_i32(*entity_id)?;
                writer.write_u8(0x00)?;
                writer.write_u8(if *sneaking { 0x02 } else { 0x00 })?;
                writer.write_u8(0x7F)?;
            }

            Self::PreChunk { chunk_x, chunk_z, mode } => {
                writer.write_u8(0x32)?;
                writer.write_i32(*chunk_x)?;
                writer.write_i32(*chunk_z)?;
                writer.write_bool(*mode)?;
            }

            Self::Chunk { chunk_x, chunk_z, data } => {
                writer.write_u8(0x33)?;
                writer.write_i32(chunk_x * 16)?;
                writer.write_i16(0)?;
                writer.write_i32(chunk_z * 16)?;
                writer.write_u8((SIZE_X - 1) as u8)?;
                writer.write_u8((SIZE_Y - 1) as u8)?;
                writer.write_u8((SIZE_Z - 1) as u8)?;
                writer.write_i32(data.len() as i32)?;
                writer.write_bytes(data)?;
            }

            Self::Kick { reason } => {
                writer.write_u8(0xFF)?;
                writer.write_string(reason)?;
            }
        }

        Ok(())
    }

    pub fn send<W: Write>(&self, stream: &mut W) -> Result<()> {
        let mut writer = PacketWriter::new(stream);

        self.write(&mut writer)
    }
}