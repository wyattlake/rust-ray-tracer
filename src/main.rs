use rust_ray_tracer::core::canvas::Canvas;
use rust_ray_tracer::core::color::*;
use rust_ray_tracer::core::matrix::Matrix4x4;
use rust_ray_tracer::core::vector::Vec4;
use rust_ray_tracer::objects::plane::Plane;
use rust_ray_tracer::objects::group::Group;
use rust_ray_tracer::misc::axis::Axis;
use rust_ray_tracer::world::camera::Camera;
use rust_ray_tracer::world::lighting::*;
use rust_ray_tracer::materials::material::Material;
use rust_ray_tracer::objects::parser::Parser;
use std::fs::File;
use rust_ray_tracer::world::scene::Scene;
use std::time::Instant;

fn main() {
    //Width and height of the scene
    const WIDTH: usize = 300;
    const HEIGHT: usize = 150;

    //Canvas where color is stored
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    // //Creating an AreaLight
    // let corner = Vec4::new(-1.0, 2.0, 4.0, 1.0);
    // let v1 = Vec4::new(2.0, 0.0, 0.0, 0.0);
    // let v2 = Vec4::new(0.0, 2.0, 0.0, 0.0);
    // let light = AreaLight::new(corner, v1, 10, v2, 10, Color(1.5, 1.5, 1.5));

    let light = PointLight::new(Color(1.0, 1.0, 1.0), Vec4(-10.0, 5.0, 2.5, 1.0));

    //Creating a plane
    let mut plane_pattern = Material::default();
    plane_pattern.color = Color(1.0, 1.0, 1.0);
    plane_pattern.ambient = 0.1;
    plane_pattern.diffuse = 0.67;
    plane_pattern.specular = 0.0;
    plane_pattern.reflectivity = 0.3;

    let plane = Plane::new(Matrix4x4::identity(), plane_pattern);

    let mut teapot_material = Material::default();
    teapot_material.specular = 0.0;

    let teapot = File::open("src/models/teapot.obj").unwrap();
    let mut group = Group::new(Matrix4x4::rotation(Axis::Y, -90.0) * Matrix4x4::scaling(0.5, 0.5, 0.5), teapot_material);
    let parsed = Parser::parse_obj(teapot);
    parsed.convert_to_group(&mut group);

    //Creates a new scene using the area light, a plane, and a sphere
    let scene: Scene = Scene {
        light_sources: vec![Box::new(light)],
        objects: vec![
            Box::new(plane),
            Box::new(group),
        ],
    };
    
    //Creates a camera and defines its properties
    let mut camera = Camera::new(WIDTH, HEIGHT, 45.0);
    let start_pos = Vec4::new(-10.0, 3.0, 2.5, 1.0);
    let end_pos = Vec4::new(0.0, 1.0, 0.0, 1.0);
    let up_vec = Vec4::new(0.0, 1.0, 0.0, 0.0);

    //Transforms the view according to the camera transformation
    camera.transform(Matrix4x4::view_transform(start_pos, end_pos, up_vec));

    println!("Render started...");
    let now = Instant::now();

    Camera::render(&camera, &scene, &mut canvas);

    let duration = now.elapsed();
    println!("Image successfully rendered");
    println!(
        "{} milliseconds elapsed.",
        duration.as_secs() * 1000 + u64::from(duration.subsec_millis())
    );
    Canvas::write_file(canvas, "image");
}
