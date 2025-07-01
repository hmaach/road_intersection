use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn draw_background(canvas: &mut Canvas<Window>) {
    // the color of the background
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // draw the roads iw white
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    draw_roads(canvas);
}

fn draw_roads(canvas: &mut Canvas<Window>) {
    let (width, height) = canvas.output_size().unwrap();
    let center_x = (width / 2) as i32;
    let center_y = (height / 2) as i32;
    let road_size = 40;

    canvas
        .draw_line(Point::new(center_x, 0), Point::new(center_x, height as i32))
        .unwrap();

    canvas
        .draw_line(
            Point::new(center_x - road_size, 0),
            Point::new(center_x - road_size, height as i32),
        )
        .unwrap();

    canvas
        .draw_line(
            Point::new(center_x + road_size, 0),
            Point::new(center_x + road_size, height as i32),
        )
        .unwrap();

    canvas
        .draw_line(Point::new(0, center_y), Point::new(width as i32, center_y))
        .unwrap();

    canvas
        .draw_line(
            Point::new(0, center_y - road_size),
            Point::new(width as i32, center_y - road_size),
        )
        .unwrap();

    canvas
        .draw_line(
            Point::new(0, center_y + road_size),
            Point::new(width as i32, center_y + road_size),
        )
        .unwrap();
}
