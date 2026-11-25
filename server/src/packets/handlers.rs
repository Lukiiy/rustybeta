use std::io::Result;

use protocol::clientbound::entity::EntityCrouch;
use protocol::serverbound::movement::*;
use protocol::serverbound::interact::EntityAction;
use protocol::shared::{Animation, Chat, Kick};
use world::Position;

use super::context::{ConnectionAction::{self, Continue, Disconnect}, ConnectionContext};
use super::registry::PacketRegistry;

type Handled = Result<ConnectionAction>; // lol

pub fn register(r: &mut PacketRegistry) { // TODO add other systems, implement their packets, add them here, yada yada yada
    r.on(on_flying).on(on_position).on(on_look).on(on_position_look).on(on_chat).on(on_animation).on(on_entity_action).on(on_disconnect);
}

pub fn on_flying(ctx: &mut ConnectionContext, p: PlayerFlying) -> Handled {
    ctx.player.set_position(ctx.player.position().with_ground(p.on_ground));

    Ok(Continue)
}

pub fn on_position(ctx: &mut ConnectionContext, p: PlayerPosition) -> Handled {
    ctx.update_position(Position {
        x: p.x,
        y: p.y,
        z: p.z,
        ..ctx.player.position().with_ground(p.on_ground)
    });

    Ok(Continue)
}

pub fn on_look(ctx: &mut ConnectionContext, p: PlayerLook) -> Handled {
    ctx.update_position(ctx.player.position().with_look(p.yaw, p.pitch).with_ground(p.on_ground));

    Ok(Continue)
}

pub fn on_position_look(ctx: &mut ConnectionContext, p: PlayerPositionLook) -> Handled {
    ctx.update_position(Position { x: p.x, y: p.y, z: p.z, yaw: p.yaw, pitch: p.pitch, on_ground: p.on_ground });

    Ok(Continue)
}

pub fn on_chat(ctx: &mut ConnectionContext, p: Chat) -> Handled {
    let name = &ctx.player.username;

    if let Some(command) = p.message.strip_prefix('/') {
        println!("{name} issued command: {command}");
    } else {
        println!("<{name}> {}", p.message);

        ctx.players.broadcast(Chat { message: format!("<{name}> {}", p.message) });
    }

    Ok(Continue)
}

pub fn on_animation(ctx: &mut ConnectionContext, p: Animation) -> Handled {
    let id = ctx.player.id();

    ctx.players.broadcast_except(id, Animation { id, animate: p.animate });

    Ok(Continue)
}

pub fn on_entity_action(ctx: &mut ConnectionContext, p: EntityAction) -> Handled {
    let sneaking = match p.action {
        1 => true,
        2 => false,
        _ => return Ok(Continue)
    };

    let id = ctx.player.id();

    ctx.players.broadcast_except(id, EntityCrouch { id, sneaking });

    Ok(Continue)
}

pub fn on_disconnect(ctx: &mut ConnectionContext, p: Kick) -> Handled {
    println!("{} disconnected: {}", ctx.player.username, p.reason);

    Ok(Disconnect)
}