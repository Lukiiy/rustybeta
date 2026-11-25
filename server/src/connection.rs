use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::Mutex;

use protocol::clientbound;
use protocol::clientbound::entity::DestroyEntity;
use protocol::clientbound::entity::PlayerPositionLook;
use protocol::clientbound::entity::SpawnPlayer;
use protocol::clientbound::login::Handshake;
use protocol::clientbound::login::Login;
use protocol::clientbound::world::ChunkData;
use protocol::clientbound::world::PreChunk;
use protocol::clientbound::world::SetSpawnPosition;
use protocol::serverbound::Serverbound;
use protocol::shared::Chat;
use protocol::shared::Kick;
use protocol::{PacketReader};
use world::World;
use world::Position;
use entity::entity::Entity;
use entity::player::{Player, PlayerRegistry};
use crate::packets::PacketRegistry;
use crate::packets::{ConnectionAction, ConnectionContext};

const MOTD: &str = "A b1.7.3 server";
const MAX_PLAYERS: u32 = 20;

pub struct Connection {
    reader: BufReader<TcpStream>,
    stream: TcpStream,
    world: Arc<Mutex<World>>,
    players: PlayerRegistry,
    packets: Arc<PacketRegistry>
}

impl Connection {
    pub fn new(stream: TcpStream, world: Arc<Mutex<World>>, players: PlayerRegistry, packets: Arc<PacketRegistry>) -> Result<Self> {
        Ok(Self {
            reader: BufReader::new(stream.try_clone()?),
            stream,
            world,
            players,
            packets
        })
    }

    /// Initialize a player's connection. handshakes & logins & sends initial chunks;
    /// Returns a runnable Player
    pub fn handle(mut self) -> Result<(Self, Arc<Player>)> {
        self.handle_handshake()?;

        let username = self.read_login()?;
        let spawn_pos = Position { x: 0.0, y: 5.0, z: 0.0, yaw: 0.0, pitch: 0.0, on_ground: false };
        let player = Arc::new(Player::new(Entity::new(spawn_pos), username, self.stream.try_clone()?));

        clientbound::send(&mut self.stream, Login {
            id: player.id(),
            seed: 0,
            dimension: 0
        })?;

        self.send_spawn_chunks(5)?;
        clientbound::send(&mut self.stream, PlayerPositionLook::at(spawn_pos))?;
        clientbound::send(&mut self.stream, SetSpawnPosition {
            x: 0,
            y: 5,
            z: 0
        })?;

        Ok((self, player))
    }

    fn read_packet(&mut self) -> Result<Serverbound> {
        Serverbound::decode(&mut PacketReader::new(&mut self.reader))
    }

    fn handle_handshake(&mut self) -> Result<()> {
        match self.read_packet()? {
            Serverbound::Handshake(packet) => {
                println!("Pending {packet:?}");

                clientbound::send(&mut self.stream, Handshake)
            }

            // legacy (1.4-1.6) server list ping: answer with a Kick carrying the status, then hang up
            Serverbound::ServerListPing(_) => {
                let mut online = 0;

                self.players.for_each(|_| online += 1);

                let status = format!("§1\0{}\0{}\0{MOTD}\0{online}\0{MAX_PLAYERS}", 127, "b1.7.3");

                clientbound::send(&mut self.stream, Kick { reason: status })?;

                Err(Error::new(ErrorKind::ConnectionAborted, "server list ping"))
            }

            other => Err(unexpected("handshake", &other))
        }
    }

    fn read_login(&mut self) -> Result<String> {
        match self.read_packet()? {
            Serverbound::Login(packet) => {
                println!("login from {}", packet.username);

                Ok(packet.username)
            }

            other => Err(unexpected("login", &other))
        }
    }

    fn send_spawn_chunks(&mut self, radius: i32) -> Result<()> {
        let mut world = self.world.lock().unwrap();

        for cx in -radius..=radius {
            for cz in -radius..=radius {
                clientbound::send(&mut self.stream, PreChunk { chunk_x: cx, chunk_z: cz, mode: true })?;
                clientbound::send(&mut self.stream, ChunkData::from_chunk(world.chunk(cx, cz))?)?;
            }
        }

        Ok(())
    }

    /// lifecycle? of a Player; Spawns them for everyone (and viceversa), then loop-handles the packets, then clean up on quit
    pub fn run(mut self, player: Arc<Player>) -> Result<()> {
        let spawn = |p: &Player| SpawnPlayer {
            id: p.id(),
            username: p.username.clone(),
            position: p.position(),
            current_item: 0
        };

        self.players.for_each(|other| {
            if other.id() == player.id() { return; }

            let _ = other.send(spawn(player.as_ref()));
            let _ = player.send(spawn(other.as_ref()));
        });

        self.players.broadcast(Chat { message: format!("§e{} joined", player.username) });

        let result = self.run_loop(&player);

        self.players.broadcast_except(player.id(), DestroyEntity { id: player.id() });
        self.players.broadcast_except(player.id(), Chat { message: format!("§e{} left", player.username) });

        result
    }

    fn run_loop(&mut self, player: &Arc<Player>) -> Result<()> {
        loop {
            let packet = self.read_packet()?;
            let mut ctx = ConnectionContext {
                player,
                world: &self.world,
                players: &self.players
            };

            if let ConnectionAction::Disconnect = self.packets.dispatch(&mut ctx, packet)? {
                return Ok(());
            }
        }
    }
}

fn unexpected(state: &str, packet: &Serverbound) -> Error {
    Error::new(ErrorKind::InvalidData, format!("Expected {state}, got {packet:?}"))
}