use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::modules::ui::View;

#[derive(PartialEq, Debug)]
pub enum Position {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(PartialEq, Debug)]
pub enum Direction {
    Straight,
    Right,
    Left,
}

#[derive(Debug)]
pub struct Vehicle {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub color: Color,
    pub start: Position,
    pub direction: Direction,
}

pub struct CoolDown {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
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

        let direction = match rand::thread_rng().gen_range(0..=2) {
            0 => Direction::Left,
            1 => Direction::Straight,
            _ => Direction::Right,
        };

        let color = match direction {
            Direction::Straight => Color::RGB(255, 165, 0),
            Direction::Right => Color::GREEN,
            Direction::Left => Color::MAGENTA,
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
