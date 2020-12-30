extern crate piston_window;
extern crate rand;

use piston_window::*;
use piston_window::types::Color;
use crate::game::Game;
use crate::draw::u32_coordinate_conversion;

mod draw;
mod snake;
mod game;

fn main() {
    // [Red, Green, Blue, Alpha]
    const BACKGROUND_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
    const BLOCK_SIZE: f64 = 25.0;
    let (width, height) = (20, 20);

    let mut window: PistonWindow = WindowSettings::new("Snake.rs",
                                                       [u32_coordinate_conversion(width, BLOCK_SIZE), u32_coordinate_conversion(height, BLOCK_SIZE)])
        .exit_on_esc(true).build().unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() { game.key_pressed(key); }

        //Generates the game window
        window.draw_2d(&event,
                       |context, graphics, _| {
                           clear(BACKGROUND_COLOR, graphics);
                           game.draw(&context, graphics);
                       });

        event.update(|arg| { game.update(arg.dt); });
    }
}