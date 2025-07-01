extern crate sdl2;

mod modules {
    pub mod ui;
    pub mod vehicle;
}

use modules::ui::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use std::time::Duration;

use crate::modules::vehicle::Vehicle;

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
        width,
        height,
        center,
        road_size,
        lights_margin,
        light_width,
        light_height,
    };

    view.vehicles
        .push(Vehicle::new(&view, modules::vehicle::Position::Right));

    view.vehicles
        .push(Vehicle::new(&view, modules::vehicle::Position::Top));

    view.vehicles
        .push(Vehicle::new(&view, modules::vehicle::Position::Bottom));

    view.vehicles
        .push(Vehicle::new(&view, modules::vehicle::Position::Left));

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        draw_ui(&mut canvas, &view);

        for veh in &view.vehicles {
            veh.draw(&mut canvas);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
