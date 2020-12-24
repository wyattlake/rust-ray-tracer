use rust_ray_tracer::core::canvas::Canvas;
use rust_ray_tracer::core::color::*;
use rust_ray_tracer::core::matrix::Matrix4x4;
use rust_ray_tracer::core::vector::Vec4;
use rust_ray_tracer::objects::plane::Plane;
use rust_ray_tracer::objects::cone::Cone;
use rust_ray_tracer::objects::group::Group;
use rust_ray_tracer::objects::cube::Cube;
use rust_ray_tracer::objects::object::*;
use rust_ray_tracer::objects::triangle::Triangle;
use rust_ray_tracer::misc::axis::Axis;
use rust_ray_tracer::world::camera::Camera;
use rust_ray_tracer::world::lighting::*;
use rust_ray_tracer::materials::material::Material;
use rust_ray_tracer::objects::cylinder::Cylinder;
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

    let light = PointLight::new(Color(1.5, 1.5, 1.5), Vec4(-1.0, 2.0, 4.0, 1.0));

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
    plane_pattern.reflectivity = 0.3;

    let plane = Plane::new(Matrix4x4::identity(), plane_pattern);

    //Creating a red sphere
    let mut red_material = Material::default();
    red_material.color = Color(1.0, 0.0, 0.0);
    red_material.ambient = 0.1;
    red_material.specular = 0.0;
    red_material.diffuse = 0.6;
    red_material.reflectivity = 0.3;

    let red_sphere = Cube::new(Matrix4x4::translation(0.5, 0.5, 0.0) * Matrix4x4::scaling(0.5, 0.5, 0.5), red_material);

    //Creating a blue sphere
    let mut blue_material = Material::default();
    blue_material.color = Color(0.0, 0.5, 0.0);
    blue_material.ambient = 0.2;
    blue_material.specular = 0.0;
    blue_material.diffuse = 0.6;
    //blue_material.reflectivity = 0.3;

    let cone1 = Cone::new(Matrix4x4::translation(0.0, 0.0, 0.0) * Matrix4x4::scaling(0.3, 0.5, 0.3), blue_material.clone(), -1.0, 0.0, true);
    let cone2 = Cone::new(Matrix4x4::translation(0.0, 0.2, 0.0) * Matrix4x4::scaling(0.3, 0.5, 0.3), blue_material.clone(), -1.0, 0.0, true);
    let cone3 = Cone::new(Matrix4x4::translation(0.0, 0.4, 0.0) * Matrix4x4::scaling(0.3, 0.5, 0.3), blue_material, -1.0, 0.0, true);

    let mut group = Group::new(Matrix4x4::translation(0.0, 0.7, 0.0), Material::default());

    let mut trunk_material = Material::default();
    trunk_material.color = Color::new_255(145, 78, 45);
    trunk_material.ambient = 0.1;
    trunk_material.specular = 0.0;
    trunk_material.diffuse = 0.6;
    //trunk_material.reflectivity = 0.3;

    let trunk = Cylinder::new(Matrix4x4::scaling(0.1, 0.3, 0.1), trunk_material, 0.0, 1.0, true);

    cone1.add_to_group(&mut group);
    cone2.add_to_group(&mut group);
    cone3.add_to_group(&mut group);

    let mut mirror_material = Material::default();
    mirror_material.color = BLACK;
    mirror_material.reflectivity = 0.9;
    
    let mirror = Cube::new(Matrix4x4::translation(1.5, 0.9, 0.4) * Matrix4x4::rotation(Axis::Y, 0.0) * Matrix4x4::scaling(0.05, 1.0, 0.5), mirror_material);

    let triangle = Triangle::default();

    //Creates a new scene using the area light, a plane, and a sphere
    let scene: Scene = Scene {
        light_sources: vec![Box::new(light)],
        objects: vec![
            Box::new(light_cube),
            Box::new(plane),
            Box::new(group),
            Box::new(trunk),
            Box::new(mirror),
            Box::new(triangle),
        ],
    };
    
    //Creates a camera and defines its properties
    let mut camera = Camera::new(WIDTH, HEIGHT, 45.0);
    let start_pos = Vec4::new(-3.0, 1.0, 2.5, 1.0);
    let end_pos = Vec4::new(0.0, 0.5, 0.0, 1.0);
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
