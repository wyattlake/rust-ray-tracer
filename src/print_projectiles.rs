use crate::vector::*;
use crate::canvas::*;
use crate::color::*;

struct Coordinates (i32, i32);

pub fn launch_projectile() {
    let mut canvas = Canvas::new(800, 800);
    let gravity = Vec4::new(0.0, -0.005, 0.0, 0.0);
    let wind = Vec4::new(-0.0005, 0.0, 0.0, 0.0);
    let mut position = Vec4::new(0.0, 0.000001, 0.0, 0.0);
    let mut velocity = Vec4::new(2.0, 0.5, 0.0, 0.0);
    //Repeats until the object hits 0
    while position.1 > 0.0 {
        velocity.normalize();
        let new_values = tick(&position, &gravity, &wind, &velocity);
        position = new_values.0;
        velocity = new_values.1;
        if &position.1 > &0.0 {
            let coords = convert_pos(&position, &canvas);
            canvas.set(Color::new(1.0, 0.0, 0.0), coords.0 as usize, coords.1 as usize);
        }
    }
    println!("Object landed at x:{}, y:{}, z:{}", position.0, position.1, position.2);
    Canvas::write_file(canvas, "test");
}

//Converts the projectile position to image coordinates
fn convert_pos(position: &Vec4, canvas: &Canvas) -> Coordinates {
    let new_x = (position.0 * 2.0).round() as i32;
    let new_y = (position.1 * 20.0).round() as i32;
    Coordinates(new_x, canvas.height as i32 - new_y)
}

//Tick finds the new position and velocity
fn tick(position: &Vec4, gravity: &Vec4, wind: &Vec4, velocity: &Vec4) -> (Vec4, Vec4) {
    let new_position = position + velocity;
    let new_velocity = (velocity + gravity + wind).clone();
    (new_position, new_velocity)
}