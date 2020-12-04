use rust_ray_tracer::core::vector::Vec4;
use rust_ray_tracer::core::matrix::Matrix4x4;
use rust_ray_tracer::core::color::*;
use rust_ray_tracer::objects::sphere::Sphere;
use rust_ray_tracer::objects::plane::Plane;
use rust_ray_tracer::core::canvas::Canvas;
use rust_ray_tracer::ray_tracing::scene::Scene;
use rust_ray_tracer::ray_tracing::patterns::*;
use rust_ray_tracer::ray_tracing::camera::Camera;
use rust_ray_tracer::ray_tracing::lighting::PointLight;
use rust_ray_tracer::ray_tracing::material::Material;
use std::time::Instant;

fn main() {
    const WIDTH: usize = 300;
    const HEIGHT: usize  = 150; 
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let mut material1 = Material::default();
    material1.pattern = Some(Box::new(CheckerboardPattern::new(BLACK, WHITE, Matrix4x4::identity())));

    let mut material2 = Material::default();
    material2.reflectivity = 0.5;

    let scene: Scene = Scene {
        light_sources: vec![
            Box::new(PointLight::new(Color::new(1.0, 1.0, 1.0), Vec4::new(-10.0, 10.0, -10.0, 1.0)))
        ],
        objects: vec![
            Box::new(Sphere::new(Matrix4x4::translation(0.0, 1.0, 0.0), material2)),
            Box::new(Plane::new(Matrix4x4::identity(), material1))
        ],
    };

    let mut camera = Camera::new(WIDTH, HEIGHT, 45.0);
    let start_pos = Vec4::new(0.0, 1.5, -7.0, 1.0);
    let end_pos = Vec4::new(0.0, 1.0, 3.0, 1.0);
    let up_vec = Vec4::new(0.0, 1.0, 0.0, 0.0);

    camera.transform(Matrix4x4::view_transform(start_pos, end_pos, up_vec));

    println!("Render started...");
    let now = Instant::now();

    Camera::render(&camera, scene, &mut canvas);

    let duration = now.elapsed();
    println!("Image successfully rendered");
    println!(
        "{} milliseconds elapsed.",
        duration.as_secs() * 1000 + u64::from(duration.subsec_millis())
    );
    Canvas::write_file(canvas, "result");
}
