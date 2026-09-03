use std::io::{Result, ErrorKind};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use crate::connection::Connection;
use world::World;
use world::FlatGenerator;
use entity::player::PlayerRegistry;
use protocol::clientbound;

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub bind_addr: String,
    pub readwrite_timeout: Duration
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_addr: "0.0.0.0:25565".into(),
            readwrite_timeout: Duration::from_secs(30)
        }
    }
}

pub struct Server {
    config: Arc<ServerConfig>,
    world: Arc<Mutex<World>>,
    players: PlayerRegistry
}

impl Server {
    pub fn new(config: ServerConfig) -> Self {
        let world = World::new(FlatGenerator {
            height: 4,
            block: 1
        });

        Self {
            config: Arc::new(config),
            world: Arc::new(Mutex::new(world)),
            players: PlayerRegistry::new()
        }
    }

    pub fn run(&self) -> Result<()> {
        let listener = match TcpListener::bind(&self.config.bind_addr) {
            Ok(listener) => listener,
            Err(e) if e.kind() == ErrorKind::AddrInUse => {
                eprintln!("Port {} is already in use!", self.config.bind_addr);

                return Err(e);
            }
            Err(e) => return Err(e)
        };

        println!("b1.7.3 server listening on {}", self.config.bind_addr);
        self.spawn_tick_thread();

        for stream in listener.incoming().flatten() {
            let world = self.world.clone();
            let players = self.players.clone();
            let config = self.config.clone();

            thread::spawn(move || Self::handle_connection(stream, world, players, config));
        }

        Ok(())
    }

    fn spawn_tick_thread(&self) {
        let players = self.players.clone();

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(1)); // TODO - Hey! This isn't 20 ticks... Not even A TICK!

                players.broadcast(|w| clientbound::write_keepalive(w));
            }
        });
    }

    fn handle_connection(stream: TcpStream, world: Arc<Mutex<World>>, players: PlayerRegistry, config: Arc<ServerConfig>) {
        let ip = stream.peer_addr().ok().map(|p| p.ip()).unwrap();

        println!("Connection from {ip}");

        if let Err(e) = stream.set_read_timeout(Some(config.readwrite_timeout)) {
            eprintln!("↳ failed to set read timeout: {e}");
        }
        if let Err(e) = stream.set_write_timeout(Some(config.readwrite_timeout)) {
            eprintln!("↳ failed to set write timeout: {e}");
        }
        if let Err(e) = stream.set_nodelay(false) {
            eprintln!("↳ failed to set tcp nodelay: {e}");
        }

        let connection = Connection::new(stream, world, players.clone());

        match connection.handle() {
            Ok((connection, player)) => {
                let username = player.username.clone();

                players.register(player.clone());

                if let Err(e) = connection.run(player.clone()) {
                    eprintln!("Connection {ip} ({username}) failed; reason: {e}");
                }

                players.unregister(player.id());
            }

            Err(e) => eprintln!("Connection {ip} failed during login: {e}")
        }

        println!("Connection from {ip} closed");
    }

}