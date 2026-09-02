use std::io::Result;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;

use protocol::{PacketReader, PacketWriter, clientbound::ClientboundPacket, serverbound};
use world::World;
use world::Position;
use entity::entity::Entity;
use entity::player::{Player, PlayerRegistry};
use crate::packets::{registry, ConnectionAction, ConnectionContext};

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

        ClientboundPacket::Login {
            entity_id: player.id(),
            world_seed: 0,
            dimension: 0
        }.send(&mut self.stream)?;

        self.send_spawn_chunks(5)?;
        ClientboundPacket::player_pos_no_stance(spawn_pos).send(&mut self.stream)?;

        ClientboundPacket::SetSpawnPosition { x: 0, y: 5, z: 0 }.send(&mut self.stream)?;

        Ok((self, player))
    }

    fn handle_handshake(&mut self) -> Result<()> {
        let packet = serverbound::read_handshake(&mut PacketReader::new(&mut self.stream))?;

        println!("Handshake from client: {packet:?}");

        ClientboundPacket::Handshake.send(&mut self.stream)
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

                ClientboundPacket::PreChunk { chunk_x: cx, chunk_z: cz, mode: true }.write(&mut writer)?;

                let chunk = world.chunk(cx, cz);

                ClientboundPacket::from_chunk(chunk)?.send(&mut self.stream)?;
            }
        }

        Ok(())
    }

    /// lifecycle? of a Player; Spawns them for everyone (and viceversa), then loop-handles the packets, then clean up on quit
    pub fn run(mut self, player: Arc<Player>) -> Result<()> {
        let spawn_player = ClientboundPacket::SpawnPlayer {
            entity_id: player.id(),
            username: player.username.clone(),
            position: player.position(),
            current_item: 0
        };

        let join = ClientboundPacket::ChatMessage { message: format!("§e{} joined", player.username) };

        self.players.for_each(|other| {
            if other.id() == player.id() { return; }

            let _ = other.send(|writer| spawn_player.write(writer));
            let _ = player.send(|writer| ClientboundPacket::SpawnPlayer {
                entity_id: other.id(),
                username: other.username.clone(),
                position: other.position(),
                current_item: 0
            }.write(writer));
        });

        self.players.broadcast(|w| join.write(w));

        let result = self.run_loop(&player);
        let destroy = ClientboundPacket::DestroyEntity { entity_id: player.id() };
        let leave = ClientboundPacket::ChatMessage { message: format!("§e{} left", player.username) };

        self.players.broadcast_except(player.id(), |w| {
            destroy.write(w)?;
            leave.write(w)
        });

        result
    }

    fn run_loop(&mut self, player: &Arc<Player>) -> Result<()> {
        loop {
            let packet_id = {
                PacketReader::new(&mut self.stream).read_u8()?
            };

            let mut ctx = ConnectionContext {
                stream: &mut self.stream,
                player,
                world: &self.world,
                players: &self.players,
            };

            match registry().handle(packet_id, &mut ctx)? {
                ConnectionAction::Continue => continue,
                ConnectionAction::Disconnect => break
            }
        }

        Ok(())
    }
}
