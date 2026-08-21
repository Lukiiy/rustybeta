use std::io;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::Mutex;

use crate::connection::Connection;
use world::World;
use world::FlatGenerator;
use entity::player::PlayerRegistry;


#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub bind_addr: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self { bind_addr: "0.0.0.0:25565".into() }
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

    pub fn run(&self) -> io::Result<()> {
        let listener = TcpListener::bind(&self.config.bind_addr)?;

        println!("b1.7.3 server listening on {}", self.config.bind_addr);

        for stream in listener.incoming().flatten() {
            self.handle_connection(stream);
        }

        Ok(())
    }

    fn handle_connection(&self, stream: TcpStream) {
        let ip = stream.peer_addr().ok().map(|p| p.ip()).unwrap();

        println!("Connection from {ip}");

        let mut connection = Connection::new(stream, self.world.clone(), self.players.clone());

        match connection.handle() {
            Ok(player) => {
                self.players.register(player.clone());

                if let Err(e) = connection.run(player.clone()) {
                    eprintln!("Connection {ip} failed; reason: {e}");
                }

                self.players.unregister(player.id());
            }

            Err(e) => eprintln!("Connection {ip} failed during login: {e}")
        }

        println!("Connection from {ip} closed");
    }

}