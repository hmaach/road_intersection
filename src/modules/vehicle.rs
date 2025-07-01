use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::modules::ui::View;

#[derive(PartialEq)]
pub enum Position {
    Top,
    Right,
    Bottom,
    Left,
}

pub enum Direction {
    Right,
    Left,
}

pub struct Vehicle {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub color: Color,
    pub start: Position,
    pub direction: Direction,
}

impl Vehicle {
    pub fn new(view: &View, start: Position) -> Self {
        let mut width: i32 = 15;
        let mut height: i32 = 30;
        let (x, y) = match start {
            Position::Top => {
                let x = view.center.x - view.road_size / 2 - width / 2;
                let y = 0;
                (x, y)
            }
            Position::Right => {
                let x = view.width as i32 - height;
                let y = view.height as i32 / 2 - view.road_size / 2 - width / 2;
                std::mem::swap(&mut width, &mut height);
                (x, y)
            }
            Position::Bottom => {
                let x = view.center.x + view.road_size / 2 - width / 2;
                let y = view.height as i32 - height;
                (x, y)
            }
            Position::Left => {
                let x = 0;
                let y = view.height as i32 / 2 + view.road_size / 2 - width / 2;
                std::mem::swap(&mut width, &mut height);
                (x, y)
            }
        };

        let direction = if rand::thread_rng().gen_bool(0.5) {
            Direction::Right
        } else {
            Direction::Left
        };

        let color = match direction {
            Direction::Right => Color::RGB(0, 0, 255),
            Direction::Left => Color::RGB(255, 255, 0),
        };

        Self {
            x,
            y,
            width: width as u32,
            height: height as u32,
            color,
            start,
            direction,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(self.color);
        canvas
            .fill_rect(Rect::new(self.x, self.y, self.width, self.height))
            .unwrap();
    }
}
