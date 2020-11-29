use rust_ray_tracer::core::vector::Vec4;
use rust_ray_tracer::core::matrix::Matrix4x4;
use rust_ray_tracer::core::color::Color;
use rust_ray_tracer::objects::sphere::Sphere;
use rust_ray_tracer::core::canvas::Canvas;
use rust_ray_tracer::ray_tracing::scene::Scene;
use rust_ray_tracer::ray_tracing::camera::Camera;
use rust_ray_tracer::ray_tracing::lighting::PointLight;
use rust_ray_tracer::misc::axis::Axis;
use rust_ray_tracer::objects::general::ObjectMethods;
use std::rc::Rc;

fn main() {
    let mut canvas = Canvas::new(100, 50);

    let mut scene = Scene::new();

    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_color, Vec4::new(-10.0, 10.0, -10.0, 1.0));
    &scene.add_light(light);

    let green = Color::new(0.1, 1.0, 0.5);
    let yellow_green = Color::new(0.5, 1.0, 0.1);
    let yellow = Color::new(1.0, 0.8, 0.1);
    let tan = Color::new(1.0, 0.9, 0.9);

    let mut floor1_raw = Sphere::new_raw();
    &floor1_raw.get_mut_material().set_color(tan.clone());
    &floor1_raw.get_mut_material().set_specular(0.0);
    &floor1_raw.transform(Matrix4x4::scaling(10.0, 0.01, 10.0));
    let floor1 = Rc::new(floor1_raw);
    &scene.add_object(floor1);

    let mut left_wall_raw = Sphere::new_raw();
    &left_wall_raw.get_mut_material().set_color(tan.clone());
    &left_wall_raw.get_mut_material().set_specular(0.0);
    &left_wall_raw.transform(Matrix4x4::translation(0.0, 0.0, 5.0) * Matrix4x4::rotation(Axis::Y, -45.0) * Matrix4x4::rotation(Axis::X, 90.0) * Matrix4x4::scaling(10.0, 0.01, 10.0));
    let left_wall = Rc::new(left_wall_raw);
    &scene.add_object(left_wall);

    let mut right_wall_raw = Sphere::new_raw();
    &right_wall_raw.get_mut_material().set_color(tan.clone());
    &right_wall_raw.get_mut_material().set_specular(0.0);
    &right_wall_raw.transform(Matrix4x4::translation(0.0, 0.0, 5.0) * Matrix4x4::rotation(Axis::Y, 45.0) * Matrix4x4::rotation(Axis::X, 90.0) * Matrix4x4::scaling(10.0, 0.01, 10.0));
    let right_wall = Rc::new(right_wall_raw);
    &scene.add_object(right_wall);

    let mut sphere1_raw = Sphere::new_raw();
    &sphere1_raw.get_mut_material().set_color(green);
    &sphere1_raw.transform(Matrix4x4::translation(-0.5, 1.0, 0.5));
    let sphere1 = Rc::new(sphere1_raw);
    &scene.add_object(sphere1);

    let mut sphere2_raw = Sphere::new_raw();
    &sphere2_raw.get_mut_material().set_color(yellow_green);
    &sphere2_raw.transform(Matrix4x4::translation(1.5, 0.5, -0.5) * Matrix4x4::scaling(0.5, 0.5, 0.5));
    let sphere1 = Rc::new(sphere2_raw);
    &scene.add_object(sphere1);

    let mut sphere3_raw = Sphere::new_raw();
    &sphere3_raw.get_mut_material().set_color(yellow);
    &sphere3_raw.transform(Matrix4x4::translation(-1.6, 0.33, -0.75) * Matrix4x4::scaling(0.33, 0.33, 0.33));
    let sphere1 = Rc::new(sphere3_raw);
    &scene.add_object(sphere1);

    let mut camera = Camera::new(100, 50, 45.0);
    let start_pos = Vec4::new(0.0, 1.5, -7.0, 1.0);
    let end_pos = Vec4::new(0.0, 1.0, 0.0, 1.0);
    let up_vec = Vec4::new(0.0, 1.0, 0.0, 0.0);

    camera.transform(Matrix4x4::view_transform(start_pos, end_pos, up_vec));

    Camera::render(&camera, &scene, &mut canvas);

    println!("Image successfully rendered");
    Canvas::write_file(canvas, "image");
}