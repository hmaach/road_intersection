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

                    Keycode::Up if view.cool_downs.bottom == 0 => {
                        view.vehicles.push(Vehicle::new(&view, Position::Bottom));
                        view.cool_downs.bottom = 90; // 1.5 second at 90 FPS
                    }

                    Keycode::Right if view.cool_downs.left == 0 => {
                        view.vehicles.push(Vehicle::new(&view, Position::Left));
                        view.cool_downs.left = 90;
                    }

                    Keycode::Down if view.cool_downs.top == 0 => {
                        view.vehicles.push(Vehicle::new(&view, Position::Top));
                        view.cool_downs.top = 90;
                    }

                    Keycode::Left if view.cool_downs.right == 0 => {
                        view.vehicles.push(Vehicle::new(&view, Position::Right));
                        view.cool_downs.right = 90;
                    }

                    _ => (),
                },

                _ => {}
            }
        }

        view.draw(&mut canvas);

        // for debugging
        for (_, r) in &view.decision_areas {
            canvas.set_draw_color(Color::RGB(66, 0, 0));
            canvas.fill_rect(*r).unwrap();
        }
        // for (_, rect) in &view.stop_lines {
        //     canvas.set_draw_color(Color::RGB(25, 155, 55));
        //     canvas.draw_rect(*rect).unwrap();
        // }

        let mut vehicle_in_decision_area = false;

        for vehicle in &view.vehicles {
            for (_, area) in &view.decision_areas {
                if vehicle.is_in_area2(area) {
                    vehicle_in_decision_area = true;
                    break;
                }
            }
            if vehicle_in_decision_area {
                break;
            }
        }

        view.update_light_timing(vehicle_in_decision_area);

        // increment cooldown for each direction
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
                        vehicle.y += 1
                    }
                }
                Position::Right => {
                    if vehicle.can_move(&cloned_view) {
                        vehicle.x -= 1
                    }
                }
                Position::Left => {
                    if vehicle.can_move(&cloned_view) {
                        vehicle.x += 1
                    }
                }
                Position::Bottom => {
                    if vehicle.can_move(&cloned_view) {
                        vehicle.y -= 1
                    }
                }
            }

            vehicle.draw(&mut canvas);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
