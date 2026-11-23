use std::io::Result;
use server::Server;
use server::ServerConfig;
use world::FlatGenerator;
use world::World;

fn main() -> Result<()> {
    println!("Hello, world!");

    let config = ServerConfig::default();
    let world = World::new(FlatGenerator {
        height: 4,
        block: 1
    });

    let server = Server::new(config, world);

    server.run()
}
