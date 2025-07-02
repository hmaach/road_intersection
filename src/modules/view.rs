use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::modules::lights::*;
use crate::modules::vehicle::*;

pub struct View {
    pub vehicles: Vec<Vehicle>,
    pub green_light: GreenLight,
    pub width: u32,
    pub height: u32,
    pub center: Point,
    pub road_size: u32,
    pub light_width: i32,
    pub light_height: i32,
    pub lights_margin: i32,
    pub cool_downs: CoolDown,
    pub decision_areas: [(DecisionAreas, Rect); 4],
}

#[derive(Clone)]
pub enum DecisionAreas {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

impl View {
    pub fn new(canvas: &Canvas<Window>) -> Self {
        let (width, height) = canvas.output_size().expect("Failed to get canvas size");
        let center = Point::new((width / 2) as i32, (height / 2) as i32);
        let road_size = 40;
        let (lights_margin, light_width, light_height) = (5, 22, 35);
        let decision_areas: [(DecisionAreas, Rect); 4] = Self::decision_areas(&center, &road_size);

        Self {
            vehicles: Vec::new(),
            green_light: GreenLight::None,
            cool_downs: CoolDown {
                top: 0,
                right: 0,
                bottom: 0,
                left: 0,
            },
            width,
            height,
            center,
            road_size,
            lights_margin,
            light_width,
            light_height,
            decision_areas,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        // Clear background
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw roads and lights using view
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.draw_roads(canvas);

        draw_lights(canvas, self);
    }

    fn draw_roads(&self, canvas: &mut Canvas<Window>) {
        let w = self.width as i32;
        let h = self.height as i32;
        let cx = self.center.x;
        let cy = self.center.y;
        let r = self.road_size as i32;

        // Vertical lines
        canvas
            .draw_line(Point::new(cx, 0), Point::new(cx, h))
            .unwrap();
        canvas
            .draw_line(Point::new(cx - r, 0), Point::new(cx - r, h))
            .unwrap();
        canvas
            .draw_line(Point::new(cx + r, 0), Point::new(cx + r, h))
            .unwrap();

        // Horizontal lines
        canvas
            .draw_line(Point::new(0, cy), Point::new(w, cy))
            .unwrap();
        canvas
            .draw_line(Point::new(0, cy - r), Point::new(w, cy - r))
            .unwrap();
        canvas
            .draw_line(Point::new(0, cy + r), Point::new(w, cy + r))
            .unwrap();
    }

    fn decision_areas(center: &Point, road_size: &u32) -> [(DecisionAreas, Rect); 4] {
        [
            (
                DecisionAreas::TopLeft,
                Rect::new(
                    center.x - *road_size as i32,
                    center.y - *road_size as i32,
                    *road_size,
                    *road_size,
                ),
            ),
            (
                DecisionAreas::TopRight,
                Rect::new(
                    center.x,
                    center.y - *road_size as i32,
                    *road_size,
                    *road_size,
                ),
            ),
            (
                DecisionAreas::BottomLeft,
                Rect::new(
                    center.x - *road_size as i32,
                    center.y,
                    *road_size,
                    *road_size,
                ),
            ),
            (
                DecisionAreas::BottomRight,
                Rect::new(center.x, center.y, *road_size, *road_size),
            ),
        ]
    }
}

pub fn decide_direction(
    vehicle: &mut Vehicle,
    decision_areas: &[(DecisionAreas, sdl2::rect::Rect); 4],
) {
    if vehicle.decision_made || vehicle.direction == Direction::Straight {
        return;
    }

    for (decision, area) in decision_areas {
        if vehicle.is_in_area(area) {
            match (&vehicle.start, &vehicle.direction, decision) {
                // Turning LEFT (Lmovya)
                (Position::Top, Direction::Left, DecisionAreas::BottomLeft) => {
                    vehicle.start = Position::Left;
                }
                (Position::Right, Direction::Left, DecisionAreas::TopLeft) => {
                    vehicle.start = Position::Top;
                }
                (Position::Bottom, Direction::Left, DecisionAreas::TopRight) => {
                    vehicle.start = Position::Right;
                }
                (Position::Left, Direction::Left, DecisionAreas::BottomRight) => {
                    vehicle.start = Position::Bottom;
                }

                // Turning RIGHT (Lkhadra)
                (Position::Right, Direction::Right, DecisionAreas::TopRight) => {
                    vehicle.start = Position::Bottom;
                }
                (Position::Top, Direction::Right, DecisionAreas::TopLeft) => {
                    vehicle.start = Position::Right;
                }
                (Position::Bottom, Direction::Right, DecisionAreas::BottomRight) => {
                    vehicle.start = Position::Left;
                }
                (Position::Left, Direction::Right, DecisionAreas::BottomLeft) => {
                    vehicle.start = Position::Top;
                }

                _ => continue,
            }
            vehicle.decision_made = true;
        }
    }
}
