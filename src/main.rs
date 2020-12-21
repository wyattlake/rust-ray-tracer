use rust_ray_tracer::core::canvas::Canvas;
use rust_ray_tracer::core::color::*;
use rust_ray_tracer::core::matrix::Matrix4x4;
use rust_ray_tracer::core::vector::Vec4;
use rust_ray_tracer::objects::plane::Plane;
use rust_ray_tracer::objects::cube::Cube;
use rust_ray_tracer::objects::sphere::Sphere;
use rust_ray_tracer::world::camera::Camera;
use rust_ray_tracer::world::lighting::*;
use rust_ray_tracer::materials::material::Material;
use rust_ray_tracer::world::scene::Scene;
use std::time::Instant;

fn main() {
    //Width and height of the scene
    const WIDTH: usize = 1000;
    const HEIGHT: usize = 500;

    //Canvas where color is stored
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    //Creating an AreaLight
    let corner = Vec4::new(-1.0, 2.0, 4.0, 1.0);
    let v1 = Vec4::new(2.0, 0.0, 0.0, 0.0);
    let v2 = Vec4::new(0.0, 2.0, 0.0, 0.0);
    let light = AreaLight::new(corner, v1, 5, v2, 5, Color(1.5, 1.5, 1.5));

    //Surrounding the light in a cube
    let mut light_cube_material = Material::default();
    light_cube_material.color = Color(1.5, 1.5, 1.5);
    light_cube_material.ambient = 1.0;
    light_cube_material.diffuse = 0.0;
    light_cube_material.specular = 0.0;
    light_cube_material.casts_shadows = false;

    let light_cube = Cube::new(Matrix4x4::translation(0.0, 3.0, 4.0) * Matrix4x4::scaling(1.0, 1.0, 0.1), light_cube_material);

    //Creating a plane
    let mut plane_pattern = Material::default();
    plane_pattern.color = Color(1.0, 1.0, 1.0);
    plane_pattern.ambient = 0.1;
    plane_pattern.diffuse = 0.67;
    plane_pattern.specular = 0.0;

    let plane = Plane::new(Matrix4x4::identity(), plane_pattern);

    //Creating a red sphere
    let mut red_material = Material::default();
    red_material.color = Color(1.0, 0.0, 0.0);
    red_material.ambient = 0.1;
    red_material.specular = 0.0;
    red_material.diffuse = 0.6;
    red_material.reflectivity = 0.3;

    let red_sphere = Sphere::new(Matrix4x4::translation(0.5, 0.5, 0.0) * Matrix4x4::scaling(0.5, 0.5, 0.5), red_material);

    //Creating a red sphere
    let mut blue_material = Material::default();
    blue_material.color = Color(0.5, 0.5, 1.0);
    blue_material.ambient = 0.1;
    blue_material.specular = 0.0;
    blue_material.diffuse = 0.6;
    blue_material.reflectivity = 0.3;

    let blue_sphere = Sphere::new(Matrix4x4::translation(-0.25, 0.33, 0.0) * Matrix4x4::scaling(0.33, 0.33, 0.33), blue_material);

    //Creating a red sphere
    let mut glass_material = Material::default();
    glass_material.color = Color(0.01, 0.01, 0.01);
    glass_material.ambient = 0.1;
    glass_material.specular = 0.0;
    glass_material.diffuse = 0.6;
    glass_material.reflectivity = 0.3;
    let cube = Cube::new(Matrix4x4::translation(0.2, 0.43, -0.1) * Matrix4x4::scaling(0.2, 0.43, 0.06), glass_material);

    //Creates a new scene using the area light, a plane, and a sphere
    let scene: Scene = Scene {
        light_sources: vec![Box::new(light)],
        objects: vec![
            Box::new(light_cube),
            Box::new(plane),
            // Box::new(red_sphere),
            // Box::new(blue_sphere),
            Box::new(cube)
        ],
    };
    
    //Creates a camera and defines its properties
    let mut camera = Camera::new(WIDTH, HEIGHT, 40.0);
    let start_pos = Vec4::new(-3.0, 1.0, 2.5, 1.0);
    let end_pos = Vec4::new(0.0, 0.5, 0.0, 1.0);
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
