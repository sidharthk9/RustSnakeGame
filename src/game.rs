use piston_window::*;
use piston_window::types::Color;
use rand::{thread_rng, Rng};

use crate::snake::{Direction, Snake};
use crate::draw::{draw_block, draw_rectangle};


pub struct Game {
    snake: Snake,
    food_state: bool,
    food_x: i32,
    food_y: i32,
    width: i32,
    height: i32,
    fail_state: bool,
    waiting_time: f64,
}

impl Game {

    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new_snake(2, 2),
            waiting_time: 0.0,
            food_state: true,
            food_x: 5,
            food_y: 5,
            width,
            height,
            fail_state: false,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.fail_state { return; }

        let dir_mapping = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(self.snake.head_direction())
        };

        if dir_mapping.unwrap() == self.snake.head_direction().opposite() { return; }

        self.update_snake(dir_mapping);
    }

    pub fn draw(&self, context: &Context, gfx: &mut G2d) {
        // [Red, Green, Blue, Alpha]
        const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
        const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
        const FAILURE_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

        self.snake.draw_body(context, gfx);

        if self.food_state {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, context, gfx);
        }

        //Border consists of four conjoined rectangles. Heh.
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, context, gfx);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, context, gfx);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, context, gfx);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, context, gfx);

        if self.fail_state {
            draw_rectangle(FAILURE_COLOR, 0, 0, self.width, self.height, context, gfx);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        const MOVING_PERIOD: f64 = 0.1;
        const RESTART_TIME: f64 = 1.0;

        self.waiting_time += delta_time;

        if self.fail_state {
            if self.waiting_time > RESTART_TIME { self.restart(); }
            return;
        }

        if !self.food_state { self.add_food(); }

        if self.waiting_time > MOVING_PERIOD { self.update_snake(None); }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_state && self.food_x == head_x && self.food_y == head_y {
            self.food_state = false;
            self.snake.restore_tail();
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();
        let mut new_x = rng.gen_range(1, self.width - 1);
        let mut new_y = rng.gen_range(1, self.height - 1);
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1, self.width - 1);
            new_y = rng.gen_range(1, self.height - 1);
        }
        self.food_x = new_x;
        self.food_y = new_y;
        self.food_state = true;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else { self.fail_state = true; }

        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new_snake(2, 2);
        self.waiting_time = 0.0;
        self.food_state = true;
        self.food_x = 6;
        self.food_y = 4;
        self.fail_state = false;
    }
}