use crate::matrix::*;
use crate::vector::*;
use crate::canvas::*;
use crate::color::*;
use crate::axis::Axis;

pub fn draw() {
    let mut canvas = Canvas::new(100, 100);
    let mut point = Vec4::new(0.0, 1.0, 0.0, 1.0);
    for _ in 1..13 {
        let coords = convert_pos(point.0, point.1);
        canvas.set(Color::new(1.0, 1.0, 1.0), coords.0, coords.1);
        point = Matrix4x4::rotation(Axis::Z, 30.0) * point;
    }
    Canvas::write_file(canvas, "result");
}

fn convert_pos(x: f64, y: f64) -> (i32, i32) {
    let new_x = ((x * 35.0) + 50.0).round() as i32;
    let new_y = ((y * 35.0) + 50.0).round() as i32;
    println!("Printing pixel (x: {}, y: {})", &new_x, &new_y);
    (new_x, new_y)
}