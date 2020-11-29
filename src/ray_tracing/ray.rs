use crate::core::vector::*;
use crate::ray_tracing::intersection::*;
use crate::core::matrix::Matrix4x4;
use crate::objects::sphere::Sphere;
use crate::ray_tracing::scene::Scene;
use std::rc::Rc;

//A Ray has a origin (point) and a direction (vector)
pub struct Ray {
    origin: Vec4,
    direction: Vec4,
}

impl Ray where {
    //Creates a new Ray
    pub fn new(origin: (f64, f64, f64), direction: (f64, f64, f64)) -> Ray {
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
    pub fn position(ray: &Ray, t: &f64) -> Vec4 {
        ray.get_origin() + (ray.get_direction() * t)
    }

        //Creates a new Ray transformed by a matrix
    pub fn transform(ray: &Ray, matrix: &Matrix4x4) -> Ray {
        Ray {
            origin: matrix * ray.origin.clone(),
            direction: matrix * ray.direction.clone(),
        }
    }

    //Lists where a ray intersects with the unit sphere
    pub fn intersect(object: Rc<Sphere>, ray: &Ray) -> Option<Vec<Intersection>> {
        let transformed_ray = Ray::transform(ray, &((*object.get_transform()).inverse().unwrap()));
        let vector_to_unit_sphere = &transformed_ray.origin - Vec4::new(0.0, 0.0, 0.0, 1.0);
        let a = Vec4::dot(&transformed_ray.direction, &transformed_ray.direction);
        let b = 2.0 * Vec4::dot(&transformed_ray.direction, &vector_to_unit_sphere);
        let c = Vec4::dot(&vector_to_unit_sphere, &vector_to_unit_sphere) - 1.0;
        let discriminant = (b * b) - (4.0 * a * c);
        if discriminant >= 0.0 {
            Some(vec![(Intersection::new((- b - (discriminant.sqrt())) / (2.0 * a), Rc::clone(&object))), (Intersection::new((- b + (discriminant.sqrt())) / (2.0 * a), Rc::clone(&object)))])
        }
        else {
            None
        }
    }

    //Lists ray intersections within a scene
    pub fn intersect_scene(scene: &Scene, ray: &Ray) -> Vec<Intersection> {
        let objects = scene.get_objects();
        let mut intersections: Vec<Intersection> = vec![];
        for object in objects {
            let transformed_ray = Ray::transform(ray, &((*object.get_transform()).inverse().unwrap()));
            let vector_to_unit_sphere = &transformed_ray.origin - Vec4::new(0.0, 0.0, 0.0, 1.0);
            let a = Vec4::dot(&transformed_ray.direction, &transformed_ray.direction);
            let b = 2.0 * Vec4::dot(&transformed_ray.direction, &vector_to_unit_sphere);
            let c = Vec4::dot(&vector_to_unit_sphere, &vector_to_unit_sphere) - 1.0;
            let discriminant = (b * b) - (4.0 * a * c);
            if discriminant >= 0.0 {
                &intersections.push(Intersection::new((- b - (discriminant.sqrt())) / (2.0 * a), Rc::clone(&object)));
                &intersections.push(Intersection::new((- b + (discriminant.sqrt())) / (2.0 * a), Rc::clone(&object)));
            }
        }
        intersections.sort_by(|a, b| b.get_t().partial_cmp(&a.get_t()).unwrap());
        intersections.reverse();
        intersections
    }
}

impl Clone for Ray {
    //Clones a given Ray
    fn clone(&self) -> Ray {
        Ray {
            origin: self.origin.clone(),
            direction: self.direction.clone(),
        }
    }
}