use std::io::Result;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;
use std::io::Error;
use std::io::ErrorKind;

use protocol::{PacketReader, PacketWriter, clientbound, serverbound};
use world::World;
use world::Position;
use entity::entity::Entity;
use entity::player::{Player, PlayerRegistry};

pub struct Connection {
    stream: TcpStream,
    world: Arc<Mutex<World>>,
    players: PlayerRegistry
}

impl Connection {
    pub fn new(stream: TcpStream, world: Arc<Mutex<World>>, players: PlayerRegistry) -> Self {
        Self { stream, world, players }
    }

    /// Initialize a player's connection. handshakes & logins & sends initial chunks;
    /// Returns a runnable Player
    pub fn handle(mut self) -> Result<(Self, Arc<Player>)> {
        self.handle_handshake()?;

        let username = self.read_login()?;
        let spawn_pos = Position { x: 0.0, y: 5.0, z: 0.0, yaw: 0.0, pitch: 0.0, on_ground: false };
        let player = Arc::new(Player::new(Entity::new(spawn_pos), username, self.stream.try_clone()?));

        clientbound::write_login(&mut PacketWriter::new(&mut self.stream), player.id(), 0, 0)?;
        self.send_spawn_chunks(5)?;
        clientbound::write_player_pos_nostance(&mut PacketWriter::new(&mut self.stream), spawn_pos)?;
        clientbound::set_spawn_pos(&mut PacketWriter::new(&mut self.stream), 0, 5, 0)?;

        Ok((self, player))
    }

    fn handle_handshake(&mut self) -> Result<()> {
        let packet = serverbound::read_handshake(&mut PacketReader::new(&mut self.stream))?;

        println!("Handshake from client: {packet:?}");
        clientbound::write_handshake(&mut PacketWriter::new(&mut self.stream))
    }

    fn read_login(&mut self) -> Result<String> {
        let packet = serverbound::read_login(&mut PacketReader::new(&mut self.stream))?;

        println!("Login request: username={}", packet.username);

        Ok(packet.username)
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

    /// lifecycle? of a Player; Spawns them for everyone (and viceversa), then loop-handles the packets, then clean up on quit
    pub fn run(mut self, player: Arc<Player>) -> Result<()> {
        self.players.for_each(|other| {
            if other.id() == player.id() { return; }

            let _ = other.send(|writer| clientbound::write_spawnplayer(writer, player.id(), &player.username, player.position(), 0));
            let _ = player.send(|writer| clientbound::write_spawnplayer(writer, other.id(), &other.username, other.position(), 0));
        });

        self.players.broadcast_except(player.id(), |w| {
            clientbound::write_chatmsg(w, &format!("§e{} joined", player.username))
        });

        let result = self.run_loop(&player);

        self.players.broadcast_except(player.id(), |w| {
            clientbound::destroy_entity(w, player.id())?;
            clientbound::write_chatmsg(w, &format!("§e{} left", player.username))
        });

        result
    }

    fn run_loop(&mut self, player: &Arc<Player>) -> Result<()> {
        loop {
            let mut reader = PacketReader::new(&mut self.stream);
            let packet_id = reader.read_u8()?;

            match packet_id {
                0x00 => {} // keep alive

                0x0A => { // player
                    reader.read_bool()?;
                }

                0x0B => { // player position
                    let x = reader.read_f64()?;
                    let y = reader.read_f64()?;
                    reader.read_f64()?; // stance
                    let z = reader.read_f64()?;
                    let on_ground = reader.read_bool()?;
                    let position = player.position().with_ground(on_ground);

                    self.update_position(player, Position { x, y, z, ..position });
                }

                0x0C => { // player look
                    let yaw = reader.read_f32()?;
                    let pitch = reader.read_f32()?;
                    let on_ground = reader.read_bool()?;
                    let position = player.position();

                    self.update_position(player, position.with_look(yaw, pitch).with_ground(on_ground));
                }

                0x0D => { // player position & look
                    let x = reader.read_f64()?;
                    let y = reader.read_f64()?;
                    reader.read_f64()?; // stance
                    let z = reader.read_f64()?;
                    let yaw = reader.read_f32()?;
                    let pitch = reader.read_f32()?;
                    let on_ground = reader.read_bool()?;

                    self.update_position(player, Position { x, y, z, yaw, pitch, on_ground });
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
                    println!("{} disconnected: {}", player.username, reader.read_string()?);

                    return Ok(());
                }

                other => {
                    return Err(Error::new(ErrorKind::InvalidData, format!("unhandled packet 0x{other:02X}")));
                }
            }
        }
    }

    fn update_position(&self, player: &Arc<Player>, position: Position) {
        player.set_position(position);

        // TODO: this sends a whole new position to sync, instead of updating via relative-pos packets
        self.players.broadcast_except(player.id(), |w| {
            clientbound::entity_teleport(w, player.id(), position)
        });
    }
}
