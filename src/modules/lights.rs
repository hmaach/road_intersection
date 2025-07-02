use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::modules::view::*;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum GreenLight {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

pub fn draw_lights(canvas: &mut Canvas<Window>, view: &View) {
    let road_size = view.road_size as i32;
    let light_w = view.light_width;
    let light_h = view.light_height;
    let margin = view.lights_margin;

    // box dimensions
    let box_w = light_w + 10;
    let box_h = light_h + 10;

    let lights = [
        (
            GreenLight::TopLeft,
            Rect::new(
                view.center.x - road_size - box_w - margin,
                view.center.y - road_size - box_h - margin,
                box_w as u32,
                box_h as u32,
            ),
        ),
        (
            GreenLight::TopRight,
            Rect::new(
                view.center.x + road_size + margin,
                view.center.y - road_size - box_h - margin,
                box_w as u32,
                box_h as u32,
            ),
        ),
        (
            GreenLight::BottomRight,
            Rect::new(
                view.center.x + road_size + margin,
                view.center.y + road_size + margin,
                box_w as u32,
                box_h as u32,
            ),
        ),
        (
            GreenLight::BottomLeft,
            Rect::new(
                view.center.x - road_size - box_w - margin,
                view.center.y + road_size + margin,
                box_w as u32,
                box_h as u32,
            ),
        ),
    ];

    for (position, box_rect) in lights {
        // Draw box background
        canvas.set_draw_color(Color::RGB(30, 30, 30)); // dark gray box
        canvas.fill_rect(box_rect).unwrap();

        let light_rect = Rect::new(
            box_rect.x() + 5,
            box_rect.y() + 5,
            light_w as u32,
            light_h as u32,
        );

        let is_green = view.green_light == position;
        let color = if is_green {
            Color::RGB(0, 255, 0)
        } else {
            Color::RGB(180, 0, 0)
        };

        canvas.set_draw_color(color);
        canvas.fill_rect(light_rect).unwrap();
    }
}
