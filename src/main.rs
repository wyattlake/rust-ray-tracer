use rust_ray_tracer::core::vector::Vec4;
use rust_ray_tracer::core::matrix::Matrix4x4;
use rust_ray_tracer::core::color::*;
use rust_ray_tracer::misc::axis::Axis;
use rust_ray_tracer::objects::sphere::Sphere;
use rust_ray_tracer::objects::plane::Plane;
use rust_ray_tracer::core::canvas::Canvas;
use rust_ray_tracer::ray_tracing::scene::Scene;
use rust_ray_tracer::ray_tracing::camera::Camera;
use rust_ray_tracer::ray_tracing::patterns::*;
use rust_ray_tracer::ray_tracing::lighting::PointLight;
use rust_ray_tracer::objects::general::ObjectMethods;
use std::rc::Rc;

fn main() {
    let mut canvas = Canvas::new(1000, 500);

    let mut scene = Scene::new();

    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_color, Vec4::new(-10.0, 10.0, -10.0, 1.0));
    &scene.add_light(light);

    let green = Color::new(0.1, 1.0, 0.5);
    let yellow_green = Color::new(0.5, 1.0, 0.1);
    let tan = Color::new(1.0, 0.9, 0.9);

    let mut pattern = RingPattern::new(Color::new(0.18, 0.63, 0.19), Color::new(0.22, 0.81, 0.28));
    &pattern.transform(Matrix4x4::scaling(0.23, 0.23, 0.23) * Matrix4x4::rotation(Axis::Z, 70.0) * Matrix4x4::rotation(Axis::X, -60.0));

    let mut pattern2 = GradientPattern::new(Color::new(0.96, 0.32, 0.12), Color::new(0.98, 0.87, 0.2));
    &pattern2.transform(Matrix4x4::scaling(1.5, 1.0, 1.0) * Matrix4x4::rotation(Axis::Y, -25.0));

    let mut pattern3 = CheckerboardPattern::new(Color::new(0.3, 0.3, 0.3), Color::new(0.75, 0.75, 0.75));
    &pattern3.transform(Matrix4x4::scaling(1.0, 1.0, 1.0));

    let mut floor1_raw = Plane::new_raw();
    &floor1_raw.get_mut_material().set_color(tan.clone());
    &floor1_raw.get_mut_material().set_specular(0.0);
    &floor1_raw.get_mut_material().set_pattern(pattern3);
    let floor1 = Rc::new(floor1_raw);
    &scene.add_object(floor1);

    let mut sphere1_raw = Sphere::new_raw();
    &sphere1_raw.get_mut_material().set_color(green);
    &sphere1_raw.transform(Matrix4x4::translation(-0.5, 1.0, 0.5));
    &sphere1_raw.get_mut_material().set_pattern(pattern);
    let sphere1 = Rc::new(sphere1_raw);
    &scene.add_object(sphere1);

    let mut sphere2_raw = Sphere::new_raw();
    &sphere2_raw.get_mut_material().set_color(yellow_green);
    &sphere2_raw.transform(Matrix4x4::translation(1.5, 0.5, -0.5) * Matrix4x4::scaling(0.5, 0.5, 0.5));
    &sphere2_raw.get_mut_material().set_pattern(pattern2);
    let sphere1 = Rc::new(sphere2_raw);
    &scene.add_object(sphere1);

    let mut camera = Camera::new(1000, 500, 45.0);
    let start_pos = Vec4::new(0.0, 1.5, -7.0, 1.0);
    let end_pos = Vec4::new(0.0, 1.0, 0.0, 1.0);
    let up_vec = Vec4::new(0.0, 1.0, 0.0, 0.0);

    camera.transform(Matrix4x4::view_transform(start_pos, end_pos, up_vec));

    Camera::render(&camera, &scene, &mut canvas);

    println!("Image successfully rendered");
    Canvas::write_file(canvas, "image");
}