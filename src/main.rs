use rust_ray_tracer::core::canvas::Canvas;
use rust_ray_tracer::core::color::*;
use rust_ray_tracer::core::matrix::Matrix4x4;
use rust_ray_tracer::core::vector::Vec4;
use rust_ray_tracer::objects::plane::Plane;
use rust_ray_tracer::misc::axis::Axis;
use rust_ray_tracer::objects::cube::Cube;
use rust_ray_tracer::objects::cylinder::Cylinder;
use rust_ray_tracer::objects::sphere::Sphere;
use rust_ray_tracer::world::camera::Camera;
use rust_ray_tracer::world::lighting::*;
use rust_ray_tracer::materials::material::Material;
use rust_ray_tracer::world::scene::Scene;
use std::{default, time::Instant};

fn main() {
    //Width and height of the scene
    const WIDTH: usize = 100;
    const HEIGHT: usize = 100;

    //Canvas where color is stored
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    //Creating an AreaLight
    let corner = Vec4(-3.0, 5.0, -5.0, 1.0);
    let v1 = Vec4::new(0.5, 0.0, 0.0, 0.0);
    let v2 = Vec4::new(0.0, 0.5, 0.0, 0.0);
    let light = AreaLight::new(corner, v1, 5, v2, 5, Color(0.8, 0.8, 0.8));

    let pl = PointLight::new(Color::new(0.4, 0.4, 0.4), Vec4::new(5.0, 0.0, 20.0, 1.0));

    //Creating a plane
    let mut white_material = Material::default();
    white_material.ambient = 0.5;
    white_material.diffuse = 0.3;
    white_material.specular = 0.0;
    white_material.casts_shadows = false;

    let mut wm2 = Material::default();
    wm2.ambient = 0.6;
    wm2.specular = 0.0;

    let mut m1 = Material::default();
    m1.color = Color::new_255(255, 163, 64);
    m1.ambient = 0.5;
    m1.diffuse = 0.85;
    m1.reflectivity = 0.1;

    let mut m2 = Material::default();
    m2.color = Color::new_255(64, 169, 255);
    m2.ambient = 0.5;
    m2.reflectivity = 0.1;

    let mut lm = Material::default();
    lm.specular = 0.0;
    lm.color = Color::new(1.5, 1.5, 1.5);
    lm.ambient = 1.5;
    lm.casts_shadows = false;
    lm.diffuse = 0.0;

    let sphere1 = Sphere::new(Matrix4x4::translation(-0.3, 1.0, -0.5), m1.clone());
    let sphere2 = Sphere::new(Matrix4x4::translation(1.3, 0.5, -0.1) * Matrix4x4::scaling(0.5, 0.5, 0.5), m2.clone());
    let sphere3 = Sphere::new(Matrix4x4::translation(-1.2, 0.4, -1.7) * Matrix4x4::scaling(0.4, 0.4, 0.4), m2.clone());
    let cube1 = Cube::new(Matrix4x4::translation(0.0, 20.0, -15.0) * Matrix4x4::scaling(20.0, 20.0, 20.0), wm2.clone());
    
    //Creates a new scene using the area light, a plane, and a sphere
    let scene: Scene = Scene {
        light_sources: vec![
            Box::new(pl),
            Box::new(light),
        ],
        objects: vec![
            Box::new(sphere1),
            Box::new(sphere2),
            Box::new(sphere3),
            Box::new(cube1),
        ],
    };
    
    //Creates a camera and defines its properties
    let mut camera = Camera::new(WIDTH, HEIGHT, 45.0);
    let start_pos = Vec4::new(0.0, 1.6, -5.5, 1.0);
    let end_pos = Vec4::new(0.0, 1.0, 1.0, 1.0);
    let up_vec = Vec4::new(0.0, 1.0, 0.0, 0.0);

    //Transforms the view according to the camera transformation
    camera.transform(Matrix4x4::view_transform(start_pos, end_pos, up_vec));

    println!("Render started...");
    let now = Instant::now();

    Camera::render_supersampled(&camera, &scene, &mut canvas);

    let duration = now.elapsed();
    println!("Image successfully rendered");
    println!(
        "{} milliseconds elapsed.",
        duration.as_secs() * 1000 + u64::from(duration.subsec_millis())
    );
    Canvas::write_file(canvas, "image");
}
