mod enums;
mod render;
mod sprites;
mod structs;
mod update;

use crate::enums::direction::Direction;
use crate::structs::player;

use self::player::*;

use sdl2::{
    event::Event,
    image::LoadTexture,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
};
use specs::prelude::Component;
use specs::prelude::VecStorage;
use specs_derive::Component;
use std::time::Duration;

const PLAYER_MOVEMENT_SPEED: i32 = 5;
const TARGET_FPS: u32 = 30;

#[derive(Component, Debug)]
#[storage(VecStorage)] // tf?
struct Position(Point);

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    x_speed: i32,
    y_speed: i32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Sprite {
    spritesheet: usize,
    region: Rect,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct SpriteAnimation {
    name: String,
    frames: Vec<Sprite>,
}

impl SpriteAnimation {
    fn new() -> Self {
        SpriteAnimation {
            name: "".to_owned(),
            frames: Vec::new(),
        }
    }

    fn name(mut self, name: &str) -> Self {
        self.name = name.to_owned();
        self
    }

    fn add_frame(mut self, spritesheet: usize, region: Rect) -> Self {
        self.frames.push(Sprite {
            spritesheet,
            region,
        });
        self
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct AnimationPlayer {
    ellapsed_time: f32,
    animation_duration: f32,
    playback_speed: f32,
    paused: bool,
}

impl AnimationPlayer {
    fn new(animation_duration: f32) -> Self {
        AnimationPlayer {
            animation_duration,
            ellapsed_time: 0.0,
            playback_speed: 1.0,
            paused: false,
        }
    }

    fn set_duration(mut self, animation_duration: f32) -> Self {
        self.animation_duration = animation_duration;
        self
    }

    fn set_speed(&mut self, new_speed: f32) {
        self.playback_speed = new_speed;
    }

    fn step(&mut self, delta_time: f32) {
        match self.paused {
            false => {
                self.ellapsed_time = (self.ellapsed_time + (delta_time * self.playback_speed))
                    % self.animation_duration;
            }
            true => {}
        }
    }

    fn pause(&mut self) {
        self.paused = true;
    }

    fn play(&mut self) {
        self.paused = false;
    }

    fn stop(&mut self) {
        self.paused = true;
        self.ellapsed_time = 0.0;
    }

    fn current_frame(self) -> u32 {
        (self.animation_duration / self.ellapsed_time) as u32
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct AnimatedSprite {
    animation_player: AnimationPlayer,
    animations: Vec<SpriteAnimation>,
}

impl AnimatedSprite {
    fn new() -> Self {
        AnimatedSprite {
            animation_player: AnimationPlayer::new(1.0),
            animations: Vec::new(),
        }
    }

    fn set_duration(mut self, animation_duration: f32) -> Self {
        self.animation_player.set_speed(animation_duration);
        self
    }

    fn add_animation(mut self, new_animation: SpriteAnimation) -> Self {
        self.animations.push(new_animation);
        self
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    // let texture_creator = canvas.texture_creator();
    // let texture = texture_creator.load_texture("assets/bardo.png")?;

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .load_texture("src/assets/bardo.png")
        .unwrap();

    // let position = Point::new(0, 0);
    // // src position in the spritesheet
    // let sprite = Rect::new(0, 0, 26, 36);

    let mut player = Player {
        position: Point::new(0, 0),
        sprite: Rect::new(0, 0, 26, 36),
        velocity: Point::new(0, 0),
        current_frame: 0,
        facing: Direction::Down,
    };

    let idle_animation = SpriteAnimation::new().name("idle");
    // .add_frame(usize::from(texture), Rect::new(0, 0, 26, 36));

    let animated_sprite = AnimatedSprite::new()
        .set_duration(2.0)
        .add_animation(idle_animation);

    'running: loop {
        canvas.set_draw_color(Color::RGB(255, 0, 255));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    player.velocity = player.velocity.offset(-PLAYER_MOVEMENT_SPEED, 0);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                    player.velocity = player.velocity.offset(PLAYER_MOVEMENT_SPEED, 0);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                    player.velocity = player.velocity.offset(0, -PLAYER_MOVEMENT_SPEED);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    player.velocity = player.velocity.offset(0, PLAYER_MOVEMENT_SPEED);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    player.velocity = player.velocity.offset(PLAYER_MOVEMENT_SPEED, 0);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                    player.velocity = player.velocity.offset(-PLAYER_MOVEMENT_SPEED, 0);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                    player.velocity = player.velocity.offset(0, PLAYER_MOVEMENT_SPEED);
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    player.velocity = player.velocity.offset(0, -PLAYER_MOVEMENT_SPEED);
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        // Update
        update::update(&mut player);

        // Render
        render::render(&mut canvas, Color::RGB(255, 0, 255), &texture, &player).ok();

        // Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / TARGET_FPS));
    }

    Ok(())
}
