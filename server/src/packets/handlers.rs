use std::io::Result;

use protocol::serverbound::movement::PlayerFlying;

use super::context::{ConnectionAction::{self, Continue}, ConnectionContext};
use super::registry::PacketRegistry;

type Handled = Result<ConnectionAction>; // lol

pub fn register(r: &mut PacketRegistry) {
    r.on(on_flying);
}

pub fn on_flying(ctx: &mut ConnectionContext, p: PlayerFlying) -> Handled {
    ctx.player.set_position(ctx.player.position().with_ground(p.on_ground));

    Ok(Continue)
}