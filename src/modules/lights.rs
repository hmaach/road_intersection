use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::modules::ui::*;

#[derive(PartialEq)]
pub enum GreenLight {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
    None,
}

pub fn draw_lights(canvas: &mut Canvas<Window>, view: &View) {
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

        canvas.set_draw_color(if is_green { Color::GREEN } else { Color::RED });

        canvas.fill_rect(rect).unwrap();
    }
}
