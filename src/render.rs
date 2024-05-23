use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::{Texture, WindowCanvas},
};

use crate::sprites::{self, direction_from_velocity};
use crate::structs::player::Player;

pub(crate) fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &Player,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let (frame_width, frame_height) = player.sprite.size();

    let current_frame = Rect::new(
        player.sprite.x + frame_width as i32 * player.current_frame,
        player.sprite.y + frame_height as i32 * sprites::direction_sprite_sheet_row(player.facing),
        frame_width,
        frame_height,
    );

    // Treat the center of the screen as the (0, 0) coordinate
    // move the sprite from world coordinates to screen coordinates
    let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);

    let screen_rect = Rect::from_center(
        screen_position,
        player.sprite.width(),
        player.sprite.height(),
    );

    canvas.copy(texture, current_frame, screen_rect)?;

    canvas.present();

    Ok(())
}
