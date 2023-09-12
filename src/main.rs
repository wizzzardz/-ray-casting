//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

mod casting;

mod map;

mod utils;

use ggez::{
    event,
    glam::*,
    graphics::{self, Color, ImageFormat},
    input::keyboard::{KeyCode, KeyInput, KeyMods},
    mint,
    winit::dpi::LogicalSize,
    Context, GameError, GameResult,
};

use ray_casting::{player::*, types::*};
use std::env;
use utils::Config;

struct MainState {
    player: Player,
    circle: graphics::Mesh,
    game_config: Config,
}

impl MainState {
    fn new(ctx: &mut Context, config: Config) -> GameResult<MainState> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2(0., 0.),
            10.0,
            2.0,
            Color::WHITE,
        )?;
        let player = Player::new(50.0, 50.0);
        Ok(MainState {
            player,
            circle,
            game_config: config,
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let k_ctx = &ctx.keyboard;
        if k_ctx.is_key_pressed(KeyCode::D) {
            self.player.walk(Vector { x: 1.0, y: 0.0 });
        } else if k_ctx.is_key_pressed(KeyCode::A) {
            self.player.walk(Vector { x: -1.0, y: 0.0 });
        }
        if k_ctx.is_key_pressed(KeyCode::W) {
            self.player.walk(Vector { x: 0.0, y: -1.0 });
        } else if k_ctx.is_key_pressed(KeyCode::S) {
            self.player.walk(Vector { x: 0.0, y: 1.0 });
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        let color = Color {
            r: 255.,
            g: 0.,
            b: 0.,
            a: 1.,
        };
        let pixels = (0..(self.game_config.screen_width * self.game_config.screen_height))
            .flat_map(|_| {
                let (r, g, b, a) = color.to_rgba();
                [r, g, b, a]
            })
            .collect::<Vec<_>>();
        let screen = graphics::Image::from_pixels(
            ctx,
            &pixels,
            graphics::ImageFormat::Rgba8UnormSrgb,
            self.game_config.screen_width as u32,
            self.game_config.screen_height as u32,
        );
        canvas.draw(
            &screen,
            Vec2::new(self.player.position.x as f32, self.player.position.y as f32),
        );

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let args: Vec<String> = env::args().collect();
    if (args.len() != 3) {
        return Err(GameError::ConfigError(
            "Invalid number of arguments".to_string(),
        ));
    }
    let game_config = Config::from_args(args);
    let cb = ggez::ContextBuilder::new("wizzzards", "blank");
    let (mut ctx, event_loop) = cb.build()?;
    ctx.gfx.window().set_inner_size(LogicalSize::new(
        game_config.screen_width,
        game_config.screen_height,
    ));
    let state = MainState::new(&mut ctx, game_config)?;
    event::run(ctx, event_loop, state)
}
