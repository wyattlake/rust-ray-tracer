use rust_ray_tracer::vector::Vec4;
use rust_ray_tracer::ray::Ray;
use rust_ray_tracer::intersection::Intersection;
use rust_ray_tracer::matrix::Matrix4x4;
use rust_ray_tracer::color::Color;
use rust_ray_tracer::canvas::Canvas;
use rust_ray_tracer::sphere::Sphere;
use rust_ray_tracer::axis::Axis;
use rust_ray_tracer::lighting::*;
use std::rc::Rc;

fn main() {
    //Origin from which all rays are cast
    let ray_origin = Vec4::new(0.0, 0.0, -5.0, 1.0);

    //Creates a new sphere, changes the color, and stores it as a new Rc
    let mut sphere = Sphere::new_raw();
    &sphere.transform(Matrix4x4::scaling(1.0, 0.3, 1.0));
    &sphere.transform(Matrix4x4::rotation(Axis::Z, 30.0));
    &sphere.transform(Matrix4x4::rotation(Axis::Y, -40.0));
    &sphere.mut_material_ref().set_color(Color::new(0.0, 1.0, 1.0));
    let sphere = Rc::new(sphere);

    //Creates a point light
    let light = PointLight::new(Color::new(1.0, 1.0, 1.0), Vec4::new(-10.0, 10.0, -10.0, 1.0));

    //Z coordinate of the wall
    let wall_z = 10.0;

    //Wide enough to capture sphere with some edges
    let wall_size = 7.0;

    //Canvas dimensions
    let canvas_w = 100.0;
    let canvas_h = 100.0;

    let mut canvas = Canvas::new(canvas_w as usize, canvas_h as usize);

    //Size of individual pixels in the world units
    let pixel_size = wall_size / canvas_h;

    //Hald of the wall
    let bound = wall_size / 2.0;

    //This loop points a vector towards the wall at each pixel
    for y in 0..(canvas_h as i32) {
        //Gets the y position in world units
        let y_pos = bound - (pixel_size * (y as f64));

        for x in 0..(canvas_h as i32) {
            //Gets the x position in world units
            let x_pos = - bound + (pixel_size * (x as f64));

            //Sets the target position for the ray
            let target_pos = Vec4::new(x_pos, y_pos, wall_z, 1.0);

            //Finds a vector from the ray origin to the target position
            let vector = (target_pos - &ray_origin).normalize();

            //Creates a new from the origin and calculated vector
            let ray = Ray::new_from_vec(ray_origin.clone(), vector);

            //Finds the intersections of the ray with the sphere
            let result = Ray::intersect(Rc::clone(&sphere), &ray);

            if result != None {
                let mut unwrapped_vec = result.unwrap();
                let i1 = unwrapped_vec.remove(0);
                let i2 = unwrapped_vec.remove(0); 
                //Finds which intersection is visible
                let visible_intersection = Intersection::hit(&[i1, i2]);
                if visible_intersection != None {
                    let hit = visible_intersection.unwrap();
                    //Finds the point at which the ray intersected the sphere
                    let point = Ray::position(&ray, &hit.get_t());
                    //Finds the normal vector
                    let normal = Vec4::normal(&hit.object, &point);
                    //Finds the eye vector
                    let eye = &ray.direction.negate();
                    let color = lighting(hit.object.material_ref(), &point, &light, &eye, &normal);

                    //Paints the Pixel if there is a visible intersection
                    canvas.set(color.clone(), x, y);
                }
            }
        }
    }
    println!("Image successfully rendered");
    Canvas::write_file(canvas, "image");
}