use rust_ray_tracer::core::canvas::Canvas;
use rust_ray_tracer::core::color::*;
use rust_ray_tracer::core::matrix::Matrix4x4;
use rust_ray_tracer::core::vector::Vec4;
use rust_ray_tracer::objects::plane::Plane;
use rust_ray_tracer::objects::group::Group;
use rust_ray_tracer::misc::axis::Axis;
use rust_ray_tracer::objects::cube::Cube;
use rust_ray_tracer::objects::cylinder::Cylinder;
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
    const HEIGHT: usize = 300;

    //Canvas where color is stored
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    //Creating an AreaLight
    let corner = Vec4(-0.25, 0.99, -1.0, 1.0);
    let v1 = Vec4::new(0.5, 0.0, 0.0, 0.0);
    let v2 = Vec4::new(0.0, 0.0, 0.5, 0.0);
    let back_light = AreaLight::new(corner, v1, 10, v2, 10, Color(0.7, 0.7, 0.65));

    //let light = PointLight::new(Color(0.1, 0.1, 0.05), Vec4(0.0, 0.0, -2.0, 1.0));
    //let front_light = PointLight::new(Color(0.1, 0.1, 0.05), Vec4(0.0, 1.0, -5.0, 1.0));
    //let front_light = PointLight::new(Color(1.1, 1.1, 1.05), Vec4(0.0, 5.0, -10.0, 1.0));

    //Creating a plane
    let mut white_material = Material::default();
    white_material.ambient = 0.3;
    white_material.diffuse = 0.9;
    white_material.specular = 0.0;

    let floor = Plane::new(Matrix4x4::identity(), white_material.clone());
    let ceiling = Plane::new(Matrix4x4::translation(0.0, 2.0, 0.0), white_material.clone());

    let mut red_material = Material::default();
    red_material.ambient = 0.2;
    red_material.diffuse = 0.9;
    red_material.specular = 0.0;
    red_material.color = Color::new_255(237, 19, 41);

    let mut green_material = Material::default();
    green_material.ambient = 0.2;
    green_material.diffuse = 0.9;
    green_material.specular = 0.0;
    green_material.color = Color::new_255(67, 209, 56);

    let left_wall = Cube::new(Matrix4x4::translation(2.0, 0.0, 0.0) * Matrix4x4::rotation(Axis::Z, 90.0) * Matrix4x4::scaling(5.0, 1.0, 5.0), green_material);
    let right_wall = Cube::new(Matrix4x4::translation(-2.0, 0.0, 0.0) * Matrix4x4::rotation(Axis::Z, 90.0) * Matrix4x4::scaling(5.0, 1.0, 5.0), red_material);
    let back_wall = Cube::new(Matrix4x4::translation(0.0, 1.0, 1.0), white_material);

    let mut box_material = Material::default();
    box_material.ambient = 0.3;
    box_material.diffuse = 0.9;
    box_material.specular = 0.0;
    box_material.color = WHITE;
    box_material.environment_lighting = 0.5;

    //let box1 = Cube::new(Matrix4x4::rotation(Axis::Y, -20.0) * Matrix4x4::translation(-0.9, 0.5, -1.3) * Matrix4x4::scaling(0.25, 0.5, 0.25), box_material.clone());
    let box1 = Cylinder::new(Matrix4x4::translation(0.0, 1.0, -1.3) * Matrix4x4::scaling(0.5, 1.0, 0.5), box_material.clone(), -1.0, 0.0, true);
    // let box2 = Cube::new(Matrix4x4::rotation(Axis::Y, 30.0) * Matrix4x4::translation(1.3, 0.23, -1.6) * Matrix4x4::scaling(0.23, 0.23, 0.23), box_material);

    // let mut teapot_material = Material::default();
    // teapot_material.specular = 0.0;

    // let teapot = File::open("src/models/smooth_teapot.obj").unwrap();
    // let mut group = Group::new(Matrix4x4::rotation(Axis::X, -90.0) * Matrix4x4::translation(0.0, 1.3, 0.0) * Matrix4x4::scaling(0.05, 0.05, 0.05), box_material);
    // let parsed = Parser::parse_obj(teapot);
    // parsed.convert_to_group(&mut group);

    //let cube = Cube::new(Matrix4x4::scaling(0.447175, 0.70498, 1.0), Material::default());

    //Creates a new scene using the area light, a plane, and a sphere
    let scene: Scene = Scene {
        light_sources: vec![
            //Box::new(front_light),
            Box::new(back_light),
        ],
        objects: vec![
            Box::new(floor),
            Box::new(ceiling),
            Box::new(left_wall),
            Box::new(right_wall),
            Box::new(back_wall),
            //Box::new(group),
            Box::new(box1),
        ],
    };
    //-5
    //0
    
    //Creates a camera and defines its properties
    let mut camera = Camera::new(WIDTH, HEIGHT, 45.0);
    let start_pos = Vec4::new(0.0, 1.0, -5.0, 1.0);
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
