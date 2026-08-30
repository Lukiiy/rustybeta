use std::io::Result;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;
use std::io::Error;
use std::io::ErrorKind;

use protocol::{PacketReader, PacketWriter, clientbound};
use world::World;
use world::Position;

pub struct Connection {
    stream: TcpStream,
    world: Arc<Mutex<World>>
}

impl Connection {
    pub fn new(stream: TcpStream, world: Arc<Mutex<World>>) -> Self {
        Self { stream, world }
    }

    pub fn handle(mut self) -> Result<()> {
        self.handle_handshake()?;
        self.handle_login()?;
        self.send_spawn_chunks(5)?;

        clientbound::write_player_pos_nostance(&mut PacketWriter::new(&mut self.stream), Position { x: 0.0, y: 5.0, z: 0.0, yaw: 0.0, pitch: 0.0, on_ground: false })?;
        clientbound::set_spawn_pos(&mut PacketWriter::new(&mut self.stream), 0, 5, 0)?;

        self.run_loop()
    }

    fn handle_handshake(&mut self) -> Result<()> {
        let packet = serverbound::read_handshake(&mut PacketReader::new(&mut self.stream))?;

        println!("Handshake from client: {packet:?}");
        clientbound::write_handshake(&mut PacketWriter::new(&mut self.stream))
    }

    fn handle_login(&mut self) -> Result<()> {
        let packet = serverbound::read_login(&mut PacketReader::new(&mut self.stream))?;

        println!("Login request: username={}", packet.username);

        clientbound::write_login(&mut PacketWriter::new(&mut self.stream), 0, 0, 0)
    }

    fn send_spawn_chunks(&mut self, radius: i32) -> Result<()> {
        let mut world = self.world.lock().unwrap();

        for cx in -radius..=radius {
            for cz in -radius..=radius {
                let mut writer = PacketWriter::new(&mut self.stream);

                clientbound::send_prechunk(&mut writer, cx, cz, true)?;

                let chunk = world.chunk(cx, cz);

                clientbound::send_chunk(&mut writer, chunk)?;
            }
        }

        Ok(())
    }

    fn run_loop(&mut self) -> Result<()> {
        loop {
            let mut reader = PacketReader::new(&mut self.stream);
            let packet_id = reader.read_u8()?;

            match packet_id {
                0x00 => {} // keep alive

                0x0A => { // player
                    reader.read_bool()?;
                }

                0x0B => { // player Position
                    reader.read_f64()?;
                    reader.read_f64()?;
                    reader.read_f64()?;
                    reader.read_f64()?;
                    reader.read_bool()?;
                }

                0x0C => { // player look
                    reader.read_f32()?;
                    reader.read_f32()?;
                    reader.read_bool()?;
                }

                0x0D => { // player full pos
                    reader.read_f64()?;
                    reader.read_f64()?;
                    reader.read_f64()?;
                    reader.read_f64()?;
                    reader.read_f32()?;
                    reader.read_f32()?;
                    reader.read_bool()?;
                }

                0x03 => { // chat/cmd
                    let message = reader.read_string()?;

                    if let Some(command) = message.strip_prefix('/') { // TODO
                        println!("{} issued command: {command}", player.username);

                        continue;
                    }

                    println!("<{}> {}", player.username, message);
                    self.players.broadcast(|w| clientbound::write_chatmsg(w, &format!("<{}> {}", player.username, message)));
                }

                0x12 => { // arm swing
                    reader.read_i32()?;
                    reader.read_u8()?;
                }

                0x65 => { // inventory close
                    reader.read_u8()?;
                }

                0x0E => { // digging? status, x, y, z, face
                    reader.read_u8()?;
                    reader.read_i32()?;
                    reader.read_u8()?;
                    reader.read_i32()?;
                    reader.read_u8()?;
                }

                0x0F => { // block placement
                    reader.read_i32()?;
                    reader.read_u8()?;
                    reader.read_i32()?;
                    reader.read_u8()?;

                    if reader.read_i16()? >= 0 {
                        reader.read_u8()?;
                        reader.read_i16()?;
                    }
                }

                0x13 => { // eid, action
                    reader.read_i32()?;
                    reader.read_u8()?;
                }

                0x07 => { // use entity
                    reader.read_i32()?;
                    reader.read_i32()?;
                    reader.read_bool()?;
                }

                0x10 => { // held item change
                    reader.read_i16()?;
                }

                0x66 => { // window click
                    reader.read_u8()?;
                    reader.read_i16()?;
                    reader.read_u8()?;
                    reader.read_i16()?;
                    reader.read_bool()?;

                    if reader.read_i16()? != -1 {
                        reader.read_u8()?;
                        reader.read_i16()?;
                    }
                }

                0x6A => { // transaction
                    reader.read_u8()?;
                    reader.read_i16()?;
                    reader.read_bool()?;
                }

                0xFF => { // disconnect/kick
                    println!("Last player disconnect: {}", reader.read_string()?);

                    return Ok(());
                }

                other => {
                    return Err(Error::new(ErrorKind::InvalidData, format!("unhandled packet 0x{other:02X}")));
                }
            }
        }
    }
}