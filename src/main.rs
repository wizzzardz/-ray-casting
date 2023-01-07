//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

mod casting;

mod map;

use ggez::{
    event,
    glam::*,
    graphics::{self, Color},
    input::keyboard::{KeyCode, KeyInput, KeyMods},
    winit::dpi::LogicalSize,
    Context, GameResult,
};
use ray_casting::{player::*, types::*};

struct MainState {
    player: Player,
    circle: graphics::Mesh,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2(0., 0.),
            10.0,
            2.0,
            Color::WHITE,
        )?;
        let player = Player::new(50.0, 50.0);
        Ok(MainState { player, circle })
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

        canvas.draw(
            &self.circle,
            Vec2::new(self.player.position.x as f32, self.player.position.y as f32),
        );

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("wizzzards", "ggez");
    let (mut ctx, event_loop) = cb.build()?;
    ctx.gfx
        .window()
        .set_inner_size(LogicalSize::new(1920.0, 1080.0));
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
