extern crate sdl2;

mod modules {
    pub mod lights;
    pub mod vehicle;
    pub mod view;
}

use modules::view::*;
use sdl2::keyboard::Keycode;
use sdl2::{event::Event, pixels::Color};
use std::time::Duration;

use crate::modules::vehicle::{Position, Vehicle};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Road Intersection", 900, 700)
        .position(6060, 30)
        // .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut view = View::new(&canvas);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(key), ..
                } => match key {
                    Keycode::Escape => break 'running,

                    Keycode::Up if view.vehicles.len() < 8 && view.cool_downs.bottom == 0 => {
                        view.vehicles.push(Vehicle::new(&view, Position::Bottom));
                        view.cool_downs.bottom = 10; // 1 second at 10 FPS
                    }

                    Keycode::Right if view.vehicles.len() < 8 && view.cool_downs.left == 0 => {
                        view.vehicles.push(Vehicle::new(&view, Position::Left));
                        view.cool_downs.left = 10;
                    }

                    Keycode::Down if view.vehicles.len() < 8 && view.cool_downs.top == 0 => {
                        view.vehicles.push(Vehicle::new(&view, Position::Top));
                        view.cool_downs.top = 10;
                    }

                    Keycode::Left if view.vehicles.len() < 8 && view.cool_downs.right == 0 => {
                        view.vehicles.push(Vehicle::new(&view, Position::Right));
                        view.cool_downs.right = 10;
                    }

                    _ => (),
                },

                _ => {}
            }
        }

        view.draw(&mut canvas);

        // for debugin
        for (_, r) in &view.decision_areas {
            canvas.set_draw_color(Color::RGB(66, 0, 0));
            canvas.fill_rect(*r).unwrap();
        }
        // for (_, rect) in &view.stop_lines {
        //     canvas.set_draw_color(Color::RGB(25, 155, 55));
        //     canvas.draw_rect(*rect).unwrap();
        // }

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

        let cloned_view = view.clone();

        for vehicle in &mut view.vehicles {
            decide_direction(vehicle, &cloned_view.decision_areas);

            match vehicle.start {
                Position::Top => {
                    if vehicle.can_move(&cloned_view.clone()) {
                        vehicle.y += 6
                    }
                }
                Position::Right => {
                    if vehicle.can_move(&cloned_view) {
                        vehicle.x -= 6
                    }
                }
                Position::Left => {
                    if vehicle.can_move(&cloned_view) {
                        vehicle.x += 6
                    }
                }
                Position::Bottom => {
                    if vehicle.can_move(&cloned_view) {
                        vehicle.y -= 6
                    }
                }
            }

            vehicle.draw(&mut canvas);
        }

        // dbg!(&view.vehicles.len());

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
