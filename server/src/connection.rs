use std::io::Result;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;

use protocol::{PacketReader, PacketWriter, clientbound, serverbound};
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

        self.players.broadcast(|w| {
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
