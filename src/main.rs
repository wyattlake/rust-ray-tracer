use rust_ray_tracer::core::canvas::Canvas;
use rust_ray_tracer::core::color::*;
use rust_ray_tracer::core::matrix::Matrix4x4;
use rust_ray_tracer::core::sequence::Sequence;
use rust_ray_tracer::core::vector::Vec4;
use rust_ray_tracer::objects::plane::Plane;
use rust_ray_tracer::objects::sphere::Sphere;
use rust_ray_tracer::ray_tracing::camera::Camera;
use rust_ray_tracer::ray_tracing::lighting::*;
use rust_ray_tracer::ray_tracing::material::Material;
use rust_ray_tracer::ray_tracing::patterns::*;
use rust_ray_tracer::ray_tracing::scene::Scene;
use std::time::Instant;

fn main() {
    //Width and height of the scene
    const WIDTH: usize = 300;
    const HEIGHT: usize = 150;

    //Canvas where color is stored
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let mut material1 = Material::default();

    //Defining a checkerboard material
    material1.pattern = Some(Box::new(CheckerboardPattern::new(
        BLACK,
        WHITE,
        Matrix4x4::identity(),
    )));

    //Defining a reflective material
    let mut material2 = Material::default();
    material2.reflectivity = 0.5;

    //Setting up variables for an area light
    let corner = Vec4::new(-10.0, 10.0, -10.0, 1.0);
    let v1 = Vec4::new(2.0, 0.0, 0.0, 0.0);
    let v2 = Vec4::new(0.0, 2.0, 0.0, 0.0);
    let mut seq = Sequence::new(vec![1.0, 0.9, 0.3, 0.1, 0.5, 1.2]);
    let light = AreaLight::new(corner, v1, 8, v2, 8, WHITE);

    //Creates a new scene using the area light, a plane, and a sphere
    let scene: Scene = Scene {
        light_sources: vec![Box::new(light)],
        objects: vec![
            Box::new(Plane::new(Matrix4x4::identity(), Material::default())),
            Box::new(Sphere::new(Matrix4x4::translation(0.0, 1.0, 0.0), Material::default())),
        ],
    };

    //Creates a camera and defines its properties
    let mut camera = Camera::new(WIDTH, HEIGHT, 45.0);
    let start_pos = Vec4::new(0.0, 1.5, -7.0, 1.0);
    let end_pos = Vec4::new(0.0, 1.0, 3.0, 1.0);
    let up_vec = Vec4::new(0.0, 1.0, 0.0, 0.0);

    //Transforms the view according to the camera transformation
    camera.transform(Matrix4x4::view_transform(start_pos, end_pos, up_vec));

    println!("Render started...");
    let now = Instant::now();

    Camera::render(&camera, &scene, &mut canvas, &mut seq);

    let duration = now.elapsed();
    println!("Image successfully rendered");
    println!(
        "{} milliseconds elapsed.",
        duration.as_secs() * 1000 + u64::from(duration.subsec_millis())
    );
    Canvas::write_file(canvas, "result");
}
