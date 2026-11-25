use std::io::Result;

use protocol::codec::Packet;
use protocol::serverbound::Serverbound;
use crate::packets::handlers;

use super::context::{ConnectionAction, ConnectionContext};

type Handler = Box<dyn Fn(&mut ConnectionContext, Serverbound) -> Result<ConnectionAction> + Send + Sync>;

/// wraps a closure, letting the bound imply the handlers lifetime type ig
fn boxed<F>(f: F) -> Handler where F: Fn(&mut ConnectionContext, Serverbound) -> Result<ConnectionAction> + Send + Sync + 'static {
    Box::new(f)
}

pub struct PacketRegistry {
    handlers: Vec<Option<Handler>>
}

impl PacketRegistry {
    pub fn empty() -> Self {
        Self {
            handlers: (0..256).map(|_| None).collect()
        }
    }

    pub fn vanilla() -> Self {
        let mut registry = Self::empty();

        handlers::register(&mut registry);
        registry
    }

    /// sets the handler for the packet
    pub fn on<P>(&mut self, f: impl Fn(&mut ConnectionContext, P) -> Result<ConnectionAction> + Send + Sync + 'static) -> &mut Self where P: Packet + TryFrom<Serverbound> {
        self.handlers[P::ID as usize] = Some(boxed(move |ctx, packet| match P::try_from(packet) {
            Ok(p) => f(ctx, p),

            Err(_) => Ok(ConnectionAction::Continue)
        }));

        self
    }

    pub fn dispatch(&self, ctx: &mut ConnectionContext, packet: Serverbound) -> Result<ConnectionAction> {
        match &self.handlers[packet.id() as usize] {
            Some(handler) => handler(ctx, packet),

            None => Ok(ConnectionAction::Continue)
        }
    }
}