use std::io::Result;
use server::Server;
use server::ServerConfig;

fn main() -> Result<()> {
    println!("Hello, world!");

    let config = ServerConfig::default();
    let server = Server::new(config);

    server.run()
}
