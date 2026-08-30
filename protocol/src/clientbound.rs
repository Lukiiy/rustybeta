use std::io::{Write, Result};

use super::PacketWriter;
use world::Chunk;
use world::{SIZE_X, SIZE_Y, SIZE_Z};
use world::Position;
use utils::math;

pub fn write_handshake<W: Write>(writer: &mut PacketWriter<W>) -> Result<()> {
    writer.write_u8(0x02)?;
    writer.write_string("-")?; // ignore auth ig
    writer.flush()
}

pub fn write_login<W: Write>(writer: &mut PacketWriter<W>, entity_id: i32, world_seed: i64, dimension: i8) -> Result<()> {
    writer.write_u8(0x01)?;
    writer.write_i32(entity_id)?;
    writer.write_string("")?; // unused, yet i did not look into why; maybe i should, maybbe not
    writer.write_i64(world_seed)?;
    writer.write_u8(dimension as u8)?;

    writer.flush()
}

pub fn write_player_pos<W: Write>(writer: &mut PacketWriter<W>, position: Position, stance: f64) -> Result<()> {
    writer.write_u8(0x0D)?;
    writer.write_f64(position.x)?;
    writer.write_f64(stance)?;
    writer.write_f64(position.y)?;
    writer.write_f64(position.z)?;
    writer.write_f32(position.yaw)?;
    writer.write_f32(position.pitch)?;
    writer.write_bool(position.on_ground)?;

    writer.flush()
}

pub fn write_player_pos_nostance<W: Write>(writer: &mut PacketWriter<W>, position: Position) -> Result<()> {
    write_player_pos(writer, position, position.y + 1.62)
}

pub fn set_spawn_pos<W: Write>(writer: &mut PacketWriter<W>, x: i32, y: i32, z: i32) -> Result<()> {
    writer.write_u8(0x06)?;
    writer.write_i32(x)?;
    writer.write_i32(y)?;
    writer.write_i32(z)?;

    writer.flush()
}

pub fn send_prechunk<W: Write>(writer: &mut PacketWriter<W>, chunk_x: i32, chunk_z: i32, mode: bool) -> Result<()> {
    writer.write_u8(0x32)?;
    writer.write_i32(chunk_x)?;
    writer.write_i32(chunk_z)?;
    writer.write_bool(mode)?;

    writer.flush()
}

pub fn send_chunk<W: Write>(writer: &mut PacketWriter<W>, chunk: &Chunk) -> Result<()> {
    let data = chunk.compressed()?;

    writer.write_u8(0x33)?;
    writer.write_i32(chunk.x * 16)?;
    writer.write_i16(0)?;
    writer.write_i32(chunk.z * 16)?;
    writer.write_u8((SIZE_X - 1) as u8)?;
    writer.write_u8((SIZE_Y - 1) as u8)?;
    writer.write_u8((SIZE_Z - 1) as u8)?;
    writer.write_i32(data.len() as i32)?;

    writer.write_bytes(&data)?;
    writer.flush()
}

pub fn write_keepalive<W: Write>(writer: &mut PacketWriter<W>) -> Result<()> {
    writer.write_u8(0x00)?;

    writer.flush()
}

/// spawn player
pub fn write_spawnplayer<W: Write>(writer: &mut PacketWriter<W>, entity_id: i32, username: &str, position: Position, current_item: i16) -> Result<()> {
    writer.write_u8(0x14)?;
    writer.write_i32(entity_id)?;
    writer.write_string(username)?;
    writer.write_i32((position.x * 32.0) as i32)?;
    writer.write_i32((position.y * 32.0) as i32)?;
    writer.write_i32((position.z * 32.0) as i32)?;
    writer.write_u8(math::angle_byte(position.yaw))?;
    writer.write_u8(math::angle_byte(position.pitch))?;
    writer.write_i16(current_item)?;

    writer.flush()
}

/// destroy entity
pub fn destroy_entity<W: Write>(writer: &mut PacketWriter<W>, entity_id: i32) -> Result<()> {
    writer.write_u8(0x1D)?;
    writer.write_i32(entity_id)?;

    writer.flush()
}

/// entity pos update
pub fn entity_teleport<W: Write>(writer: &mut PacketWriter<W>, entity_id: i32, position: Position) -> Result<()> {
    writer.write_u8(0x22)?;
    writer.write_i32(entity_id)?;
    writer.write_i32((position.x * 32.0) as i32)?;
    writer.write_i32((position.y * 32.0) as i32)?;
    writer.write_i32((position.z * 32.0) as i32)?;
    writer.write_u8(math::angle_byte(position.yaw))?;
    writer.write_u8(math::angle_byte(position.pitch))?;

    writer.flush()
}

pub fn write_chatmsg<W: Write>(writer: &mut PacketWriter<W>, message: &str) -> Result<()> {
    writer.write_u8(0x03)?;
    writer.write_string(message)?;

    writer.flush()
}

pub fn write_kick<W: Write>(writer: &mut PacketWriter<W>, reason: &str) -> Result<()> {
    writer.write_u8(0xFF)?;
    writer.write_string(reason)?;

    writer.flush()
}