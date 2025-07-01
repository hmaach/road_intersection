use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::modules::vehicle::Vehicle;

#[derive(PartialEq)]
pub enum GreenLight {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
    None,
}

pub struct View {
    pub vehicles: Vec<Vehicle>,
    pub green_light: GreenLight,
    pub width: u32,
    pub height: u32,
    pub center: Point,
    pub road_size: i32,
    pub light_width: i32,
    pub light_height: i32,
    pub lights_margin: i32,
}

pub fn draw_ui(canvas: &mut Canvas<Window>, view: &View) {
    // Clear background
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // Draw roads and lights using view
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    draw_roads(canvas, &view);

    draw_lights(canvas, &view);
}

fn draw_roads(canvas: &mut Canvas<Window>, view: &View) {
    let w = view.width as i32;
    let h = view.height as i32;
    let cx = view.center.x;
    let cy = view.center.y;
    let r = view.road_size;

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

fn draw_lights(canvas: &mut Canvas<Window>, view: &View) {
    let lights = [
        (
            GreenLight::TopLeft,
            Rect::new(
                view.center.x - view.road_size - view.light_width - view.lights_margin,
                view.center.y - view.road_size - view.light_height - view.lights_margin,
                view.light_width as u32,
                view.light_height as u32,
            ),
        ),
        (
            GreenLight::TopRight,
            Rect::new(
                view.center.x + view.road_size + view.lights_margin,
                view.center.y - view.road_size - view.light_height - view.lights_margin,
                view.light_width as u32,
                view.light_height as u32,
            ),
        ),
        (
            GreenLight::BottomRight,
            Rect::new(
                view.center.x + view.road_size + view.lights_margin,
                view.center.y + view.road_size + view.lights_margin,
                view.light_width as u32,
                view.light_height as u32,
            ),
        ),
        (
            GreenLight::BottomLeft,
            Rect::new(
                view.center.x - view.road_size - view.light_width - view.lights_margin,
                view.center.y + view.road_size + view.lights_margin,
                view.light_width as u32,
                view.light_height as u32,
            ),
        ),
    ];

    for (position, rect) in lights {
        let is_green = view.green_light == position;

        canvas.set_draw_color(if is_green {
            Color::RGB(0, 255, 0) // Green
        } else {
            Color::RGB(255, 0, 0) // Red
        });

        canvas.fill_rect(rect).unwrap();
    }
}
