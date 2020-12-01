use crate::core::vector::*;

//Launches a projectile with set environment conditions
pub fn _launch_projectile() {
    let gravity = Vec4::new(0.0, -0.1, 0.0, 0.0);
    let wind = Vec4::new(-0.01, 0.0, 0.0, 0.0);
    let mut position = Vec4::new(0.0, 1.0, 0.0, 0.0);
    let mut velocity = Vec4::new(1.0, 1.0, 0.0, 0.0);
    //Repeats until the object hits 0
    while position.1 > 0.0 {
        velocity.normalize();
        let new_values = _tick(&position, &gravity, &wind, &velocity);
        position = new_values.0;
        velocity = new_values.1;
    }
    println!("Object landed at x:{}, y:{}, z:{}", position.0, position.1, position.2)
}

//Tick finds the new position and velocity
fn _tick(position: &Vec4, gravity: &Vec4, wind: &Vec4, velocity: &Vec4) -> (Vec4, Vec4) {
    let new_position = position + velocity;
    let new_velocity = velocity + gravity + wind;
    (new_position, new_velocity)
}