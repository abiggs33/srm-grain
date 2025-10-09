use std::f32::consts::PI;

use srmgrain::*;

const HEIGHT: usize = 1000;
const WIDTH: usize = 1000;

fn main() {
    let mut grain = Domain::new(WIDTH, HEIGHT);

    // basically enter a star shaped grain hole with a radius of 30
    let (x, y) = (grain.width as f32 / 2.0, grain.height as f32 / 2.0);
    let or = 150.0;
    let ir = 50.0;
    let points = 8;
    for i in 0..grain.height {
        for j in 0..grain.width {
            let (x, y) = (j as f32 - x, i as f32 - y);
            let dist = (x * x + y * y).sqrt();
            let theta = y.atan2(x);
            // find the actual point according to the star
            let target = ir + (or - ir) * 0.5 * (1.0 + ((theta + PI) * points as f32).cos());

            if dist < target {
                grain.cells[i][j].status = Status::None;
            }
        }
    }

    // run the algorithm to find how the grain will burn
    grain.distance_field();

    // use this tiny library to open a window to draw the grain into
    let mut window =
        minifb::Window::new("srm grain", WIDTH, HEIGHT, minifb::WindowOptions { ..Default::default() })
            .expect("failed to grab window handle");

    let mut time_threshold = f32::EPSILON;
    while !window.is_key_down(minifb::Key::Escape) {
        if window.is_key_down(minifb::Key::Q) {
            time_threshold += 1.0;
        }
        if window.is_key_down(minifb::Key::A) {
            time_threshold -= 1.0;
        }

        // collected a packed-color vector corresponding to black for fuel and
        // white for empty
        let buffer: Vec<u32> = grain
            .cells
            .iter()
            .flat_map(|inner| {
                inner.iter().map(|cell| if cell.time < time_threshold { 0xffffffff } else { 0xff000000 })
            })
            .collect();
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
