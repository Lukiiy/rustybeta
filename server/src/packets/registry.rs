use std::io::{Error, ErrorKind, Result, Write};
use std::net::TcpStream;
use std::sync::OnceLock;

use protocol::{PacketReader, PacketWriter};
use super::context::{ConnectionAction, ConnectionContext};

/// Defines a packet that the clients will send to the server.
/// Needs ID and binary serialization & deserialization code.
pub trait ServerboundPacket: Sized {
    const ID: u8;

    fn read(reader: &mut PacketReader<&mut TcpStream>) -> Result<Self>;
    fn handle(self, ctx: &mut ConnectionContext) -> Result<ConnectionAction>;
}

/// Defines a packet that will be sent to clients.
/// Needs ID and binary serialization code.
pub trait ClientboundPacket {
    const ID: u8;

    fn write<W: Write>(&self, writer: &mut PacketWriter<W>) -> Result<()>;
}

type PacketHandlerFn = fn(&mut ConnectionContext) -> Result<ConnectionAction>;

pub struct PacketRegistry {
    handlers: [Option<PacketHandlerFn>; 256],
}

impl PacketRegistry {
    pub fn new() -> Self {
        Self {
            handlers: [None; 256],
        }
    }

    pub fn register<P: ServerboundPacket>(&mut self) {
        self.handlers[P::ID as usize] = Some(|ctx| {
            let packet = P::read(&mut PacketReader::new(&mut ctx.stream))?;

            packet.handle(ctx)
        });
    }

    pub fn handle(&self, id: u8, ctx: &mut ConnectionContext) -> Result<ConnectionAction> {
        if let Some(handler) = self.handlers[id as usize] {
            handler(ctx)
        } else {
            Err(Error::new(ErrorKind::InvalidData, format!("Unhandled packet 0x{id:02X}")))
        }
    }
}

pub fn registry() -> &'static PacketRegistry {
    static REGISTRY: OnceLock<PacketRegistry> = OnceLock::new();

    REGISTRY.get_or_init(|| {
        let mut r = PacketRegistry::new();

        r
    })
}