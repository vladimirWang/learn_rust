extern crate piston_window;
extern crate rand;

use crate::piston_window::*;
use piston_window::types::Color;
mod draw;
mod snake;
mod game;
use crate::game::Game;
use crate::draw::to_coord_u32;

const BLACK_COLOR:Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (20, 20);

    let mut game = Game::new(width, height);
    let mut window:PistonWindow = WindowSettings::new(
        "snake",
        [to_coord_u32(width), to_coord_u32(height)]
    ).exit_on_esc(true).build().unwrap();
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, _| {
            clear(BLACK_COLOR, g);
            game.draw(&c, g);
        });
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}