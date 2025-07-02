use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::modules::{
    lights::GreenLight,
    view::{View, decision_area_to_light},
};

#[derive(PartialEq, Debug, Clone)]
pub enum Position {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Direction {
    Straight,
    Right,
    Left,
}

#[derive(Debug, Clone)]
pub struct Vehicle {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub color: Color,
    pub start: Position,
    pub decision_made: bool,
    pub direction: Direction,
}

#[derive(Debug, Clone)]
pub struct CoolDown {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

impl Vehicle {
    pub fn new(view: &View, start: Position) -> Self {
        let width: i32 = 25;
        let height: i32 = 25;
        let (x, y) = match start {
            Position::Top => {
                let x = view.center.x - view.road_size as i32 / 2 - width / 2;
                let y = 0;
                (x, y)
            }
            Position::Right => {
                let x = view.width as i32 - height;
                let y = view.height as i32 / 2 - view.road_size as i32 / 2 - width / 2;
                (x, y)
            }
            Position::Bottom => {
                let x = view.center.x + view.road_size as i32 / 2 - width / 2;
                let y = view.height as i32 - height;
                (x, y)
            }
            Position::Left => {
                let x = 0;
                let y = view.height as i32 / 2 + view.road_size as i32 / 2 - width / 2;
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
            Direction::Right => Color::CYAN,
            Direction::Left => Color::MAGENTA,
        };

        Self {
            x,
            y,
            width: width as u32,
            height: height as u32,
            decision_made: false,
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

    pub fn can_move(&self, view: &View) -> bool {
        let car_rect = Rect::new(self.x, self.y, self.width, self.height);
        let margin = 13;

        for (my_light, rect) in &view.stop_lines {
            if *my_light == view.green_light {
                continue;
            }
            if rect.has_intersection(car_rect) {
                return false;
            }
        }

        for other in &view.vehicles {
            if other.x == self.x && other.y == self.y {
                continue; // skip self
            }

            // Expand the other car's rectangle with margin
            let other_rect_with_margin = Rect::new(
                other.x - margin,
                other.y - margin,
                other.width + margin as u32 * 2,
                other.height + margin as u32 * 2,
            );

            if other_rect_with_margin.has_intersection(car_rect) {
                return false;
            }
        }

        true
    }

    pub fn is_in_area(&self, area: &Rect) -> bool {
        let car_center_x = self.x + (self.width as i32) / 2;
        let car_center_y = self.y + (self.height as i32) / 2;
        let area_center_x = area.x() + (area.width() as i32) / 2;
        let area_center_y = area.y() + (area.height() as i32) / 2;

        let small_car = Rect::new(car_center_x, car_center_y, 3, 3);
        let small_area = Rect::new(area_center_x, area_center_y, 3, 3);

        small_area.has_intersection(small_car)
    }
}
