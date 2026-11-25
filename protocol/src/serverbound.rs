pub mod login;
pub mod movement;
pub mod interact;
pub mod window;

use std::io::{Error, ErrorKind, Read, Result};

use crate::PacketReader;
use crate::codec::{Decode, Packet};
use crate::shared;

macro_rules! serverbound { // todo template!!!
    ($($name:ident($ty:ty)),* $(,)?) => { // name + type + varags* with ,
        #[derive(Debug, Clone)]
        pub enum Serverbound {
            $($name($ty)),*
        }

        impl Serverbound {
            pub fn id(&self) -> u8 {
                match self {
                    $(Self::$name(_) => <$ty as Packet>::ID),*
                }
            }

            pub fn decode<R: Read>(r: &mut PacketReader<R>) -> Result<Self> {
                match r.read_u8()? {
                    $(id if id == <$ty as Packet>::ID => Ok(Self::$name(<$ty>::decode(r)?)),)*

                    id => Err(Error::new(ErrorKind::InvalidData, format!("Unhandled packet 0x{id:02X}")))
                }
            }
        }

        $(
            impl From<$ty> for Serverbound {
                fn from(p: $ty) -> Self { Self::$name(p) }
            }

            impl TryFrom<Serverbound> for $ty {
                type Error = Serverbound;

                /// Unwraps into a possible Packet, basically turning the enum into the typed version.
                fn try_from(p: Serverbound) -> std::result::Result<Self, Serverbound> {
                    match p {
                        Serverbound::$name(p) => Ok(p),

                        other => Err(other)
                    }
                }
            }
        )*
    };
}

serverbound! {
    KeepAlive(shared::KeepAlive),
    Chat(shared::Chat),
    Kick(shared::Kick),
    Animation(shared::Animation),
    Transaction(shared::Transaction),
    CloseWindow(shared::CloseWindow),
    Handshake(login::Handshake),
    Login(login::Login),
    ServerListPing(login::ServerListPing),
    PlayerFlying(movement::PlayerFlying),
    PlayerPosition(movement::PlayerPosition),
    PlayerLook(movement::PlayerLook),
    PlayerPositionLook(movement::PlayerPositionLook),
    PlayerDigging(interact::PlayerDigging),
    BlockPlacement(interact::BlockPlacement),
    EntityAction(interact::EntityAction),
    UseEntity(interact::UseEntity),
    HeldItemChange(interact::HeldItemChange),
    WindowClick(window::WindowClick)
}