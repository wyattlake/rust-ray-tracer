use rust_ray_tracer::core::canvas::Canvas;
use rust_ray_tracer::core::color::*;
use rust_ray_tracer::core::matrix::Matrix4x4;
use rust_ray_tracer::core::sequence::Sequence;
use rust_ray_tracer::core::vector::Vec4;
use rust_ray_tracer::objects::plane::Plane;
use rust_ray_tracer::objects::cube::Cube;
use rust_ray_tracer::world::camera::Camera;
use rust_ray_tracer::world::lighting::*;
use rust_ray_tracer::misc::axis::Axis;
use rust_ray_tracer::materials::material::Material;
use rust_ray_tracer::materials::patterns::*;
use rust_ray_tracer::world::scene::Scene;
use std::time::Instant;

fn main() {
    //Width and height of the scene
    const WIDTH: usize = 1500;
    const HEIGHT: usize = 750;

    //Canvas where color is stored
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let mut material1 = Material::default();

    //Defining a checkerboard material
    material1.pattern = Some(Box::new(CheckerboardPattern::new(WHITE, BLACK, Matrix4x4::identity())));
    material1.diffuse = 0.8;
    material1.casts_shadows = false;

    //Defining a reflective material
    let mut material2 = Material::default();
    material2.color = Color::new(0.3, 0.3, 0.3);
    material2.reflectivity = 0.9;
    material2.transparency = 0.99;
    material2.refractive_index = 1.1;

    let point_light = PointLight::new(Color(1.0, 1.0, 1.0), Vec4::new(-10.0, 10.0, -5.0, 1.0));

    let mut seq = Sequence::new(vec![1.0, 0.9, 0.3, 0.1, 0.5, 1.2]);

    //Creates a new scene using the area light, a plane, and a sphere
    let scene: Scene = Scene {
        light_sources: vec![Box::new(point_light)],
        objects: vec![
            Box::new(Plane::new(Matrix4x4::translation(0.0, 0.0, 8.0) * Matrix4x4::rotation(Axis::X, 90.0), material1.clone())),
            Box::new(Cube::new(
                Matrix4x4::translation(0.0, 1.0, 0.0) * Matrix4x4::rotation(Axis::Y, -30.0) * Matrix4x4::rotation(Axis::X, -45.0),
                material2,
            )),
        ],
    };
    
    //Creates a camera and defines its properties
    let mut camera = Camera::new(WIDTH, HEIGHT, 40.0);
    let start_pos = Vec4::new(0.0, 1.5, -7.0, 1.0);
    let end_pos = Vec4::new(0.0, 1.1, 3.0, 1.0);
    let up_vec = Vec4::new(0.0, 1.0, 0.0, 0.0);

    //Transforms the view according to the camera transformation
    camera.transform(Matrix4x4::view_transform(start_pos, end_pos, up_vec));

    println!("Render started...");
    let now = Instant::now();

    Camera::render_supersampled(&camera, &scene, &mut canvas, &mut seq);

    let duration = now.elapsed();
    println!("Image successfully rendered");
    println!(
        "{} milliseconds elapsed.",
        duration.as_secs() * 1000 + u64::from(duration.subsec_millis())
    );
    Canvas::write_file(canvas, "image");
}
