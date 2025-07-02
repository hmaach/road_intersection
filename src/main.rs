extern crate sdl2;

mod modules {
    pub mod lights;
    pub mod ui;
    pub mod vehicle;
}

use modules::ui::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;

use crate::modules::{
    lights::GreenLight,
    vehicle::{CoolDown, Position, Vehicle},
};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Road Intersection", 900, 700)
        .position(1010, 30)
        // .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Get canvas dimensions
    let (width, height) = canvas.output_size().expect("Failed to get canvas size");

    let center = Point::new((width / 2) as i32, (height / 2) as i32);
    let road_size = 40;
    let lights_margin = 5;
    let light_width = 22;
    let light_height = 35;

    let mut view = View {
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
    };

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::Escape => break 'running,

                    Keycode::Up if view.vehicles.len() < 8 && view.cool_downs.bottom == 0 => {
                        view.vehicles.push(Vehicle::new(&view, Position::Bottom));
                        view.cool_downs.bottom = 60; // 1 second at 60 FPS
                    }

                    Keycode::Right if view.vehicles.len() < 8 && view.cool_downs.left == 0 => {
                        view.vehicles.push(Vehicle::new(&view, Position::Left));
                        view.cool_downs.left = 60;
                    }

                    Keycode::Down if view.vehicles.len() < 8 && view.cool_downs.top == 0 => {
                        view.vehicles.push(Vehicle::new(&view, Position::Top));
                        view.cool_downs.top = 60;
                    }

                    Keycode::Left if view.vehicles.len() < 8 && view.cool_downs.right == 0 => {
                        view.vehicles.push(Vehicle::new(&view, Position::Right));
                        view.cool_downs.right = 60;
                    }

                    _ => (),
                },

                _ => {}
            }
        }

        view.draw(&mut canvas);

        if view.cool_downs.top > 0 {
            view.cool_downs.top -= 1;
        }
        if view.cool_downs.right > 0 {
            view.cool_downs.right -= 1;
        }
        if view.cool_downs.bottom > 0 {
            view.cool_downs.bottom -= 1;
        }
        if view.cool_downs.left > 0 {
            view.cool_downs.left -= 1;
        }

        // check if a car reached the end
        view.vehicles.retain(|vehicle| match vehicle.start {
            Position::Top => vehicle.y <= view.height as i32, // bottom reached
            Position::Bottom => vehicle.y + vehicle.height as i32 >= 0, // top reached
            Position::Left => vehicle.x <= view.width as i32, // Right reached
            Position::Right => vehicle.x + vehicle.width as i32 >= 0, // Left reached
        });

        for vehicle in &mut view.vehicles {
            match vehicle.start {
                Position::Top => vehicle.y += 1,
                Position::Right => vehicle.x -= 1,
                Position::Left => vehicle.x += 1,
                Position::Bottom => vehicle.y -= 1,
            }
            vehicle.draw(&mut canvas);
        }

        // dbg!(&view.vehicles.len());

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
