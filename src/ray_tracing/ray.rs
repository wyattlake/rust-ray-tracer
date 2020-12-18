use crate::core::vector::*;
use crate::ray_tracing::intersection::*;
use crate::core::matrix::Matrix4x4;
use crate::world::scene::*;

//A Ray has a origin (point) and a direction (vector)
#[derive(Debug, PartialEq, Clone)]
pub struct Ray {
    origin: Vec4,
    direction: Vec4,
}

impl Ray where {
    //Creates a new Ray
    pub fn new(origin: (f32, f32, f32), direction: (f32, f32, f32)) -> Ray {
        Ray {
            origin: Vec4::new(origin.0, origin.1, origin.2, 1.0),
            direction: Vec4::new(direction.0, direction.1, direction.2, 0.0),
        }
    }

    //Creates a new Ray
    pub fn new_from_vec(origin: Vec4, direction: Vec4) -> Ray {
        Ray {
            origin,
            direction,
        }
    }

    //Gets the origin of a ray
    pub fn get_origin(&self) -> &Vec4 {
        &self.origin
    }

    //Gets the direction of a ray
    pub fn get_direction(&self) -> &Vec4 {
        &self.direction
    }

    //Calculates the position of a ray
    pub fn position(ray: &Ray, t: f32) -> Vec4 {
        ray.get_origin() + (ray.get_direction() * t)
    }

        //Creates a new Ray transformed by a matrix
    pub fn transform(ray: &Ray, matrix: &Matrix4x4) -> Ray {
        Ray {
            origin: matrix * ray.get_origin(),
            direction: matrix * ray.get_direction(),
        }
    }

    //Lists ray intersections within a scene
    pub fn intersect_scene<'a>(scene: &'a Scene, ray: Ray) -> Vec<Intersection> {
        let objects = &scene.objects;
        let mut intersections: Vec<Intersection> = vec![];
        for object in objects {
            let object_intersections = object.intersect(&ray);
            if !object_intersections.is_none() {
                let unwrapped_intersections = object_intersections.unwrap();
                for x in unwrapped_intersections {
                    intersections.push(x);
                }
            }
        }
        intersections.sort_by(|a, b| b.t.partial_cmp(&a.t).unwrap());
        intersections.reverse();
        intersections
    }
}