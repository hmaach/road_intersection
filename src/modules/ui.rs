use sdl2::pixels::Color;
use sdl2::rect::Point;
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
    pub road_size: i32,
    pub light_width: i32,
    pub light_height: i32,
    pub lights_margin: i32,
    pub cool_downs: CoolDown,
}

impl View {
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
        let r = self.road_size;

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
}
