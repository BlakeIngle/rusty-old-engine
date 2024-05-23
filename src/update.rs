use sdl2::rect::Point;

use crate::{enums::direction, sprites::direction_from_velocity, structs::player::Player};

pub(crate) fn update(player: &mut Player) {
    let direction = direction_from_velocity(&player.velocity);

    player.facing = match direction {
        Some(d) => d,
        None => player.facing,
    };

    player.position = player.position.offset(player.velocity.x, player.velocity.y);

    // update sprite frames
    match player.velocity {
        p if p.x == 0 && p.y == 0 => {
            // not moving
            // do not move frames
        }
        _ => {
            // moving
            player.current_frame = (player.current_frame + 1) % 3;
            // 3 frames total each animation
        }
    }
}
