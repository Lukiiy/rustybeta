pub mod entity;
pub mod login;
pub mod world;

use std::io::{Result, Write};

use crate::PacketWriter;
use crate::codec::{Encode, Packet};
use crate::shared;

macro_rules! clientbound { // todo template!!!
    ($($name:ident($ty:ty)),* $(,)?) => { // name + type + varags* with ,
        #[derive(Debug, Clone)]
        pub enum Clientbound {
            $($name($ty)),*
        }

        impl Clientbound {
            pub fn id(&self) -> u8 {
                match self {
                    $(Self::$name(_) => <$ty as Packet>::ID),*
                }
            }

            pub fn write<W: Write>(&self, w: &mut PacketWriter<W>) -> Result<()> {
                w.write_u8(self.id())?;

                match self {
                    $(Self::$name(p) => p.encode(w)),*
                }
            }
        }

        $(impl From<$ty> for Clientbound {
            fn from(p: $ty) -> Self { Self::$name(p) }
        })*
    };
}

clientbound! {
    KeepAlive(shared::KeepAlive),
    Chat(shared::Chat),
    Kick(shared::Kick),
    Animation(shared::Animation),
    Handshake(login::Handshake),
    Login(login::Login),
    SetSpawnPosition(world::SetSpawnPosition),
    PreChunk(world::PreChunk),
    ChunkData(world::ChunkData),
    PlayerPositionLook(entity::PlayerPositionLook),
    SpawnPlayer(entity::SpawnPlayer),
    DestroyEntity(entity::DestroyEntity),
    EntityTeleport(entity::EntityTeleport),
    EntityMove(entity::EntityMove),
    EntityCrouch(entity::EntityCrouch)
}

impl Clientbound {
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(32);

        self.write(&mut PacketWriter::new(&mut buf))?;

        Ok(buf)
    }
}

pub fn send<W: Write>(stream: &mut W, packet: impl Into<Clientbound>) -> Result<()> {
    stream.write_all(&packet.into().to_bytes()?)
}