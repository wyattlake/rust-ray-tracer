use crate::core::matrix::Matrix4x4;
use crate::core::vector::Vec4;
use crate::objects::general::{Object, ObjectMethods};
use crate::ray_tracing::material::Material;
use crate::ray_tracing::intersection::Intersection;
use crate::ray_tracing::ray::Ray;
use crate::misc::utils::*;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Plane {
    transform: Matrix4x4,
    material: Material,
}

impl ObjectMethods for Plane {
    //Instantiates a Plane with a Rc and an identity Matrix as its transform
    fn new() -> Rc<Object> {
        Rc::new(
            Object::Plane(Plane {
                transform: Matrix4x4::identity(),
                material: Material::default()
            })
        )
    }

    //Instantiates a Plane with an identity Matrix as its transform 
    fn new_raw() -> Object {
        Object::Plane(Plane {
            transform: Matrix4x4::identity(),
            material: Material::default()
        })
    }

    //Applies a transformation to a Plane
    fn transform(&mut self, matrix: Matrix4x4) {
        self.transform = matrix * self.transform.clone();
    }

    //Returns the Plane transform
    fn get_transform(&self) -> &Matrix4x4 {
        &self.transform
    }

    //Returns the plane material
    fn get_material(&self) -> &Material {
        &self.material
    }

    //Sets the material of the plane
    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    //Gets a mutable reference to the material
    fn get_mut_material(&mut self) -> &mut Material {
        &mut self.material
    }

    //Intersects a ray with a plane
    fn intersect(object: &Rc<Object>, ray: &Ray) -> Option<Vec<Intersection>> {
        let transformed_ray = Ray::transform(ray, &((object.get_transform()).inverse().unwrap()));
        if transformed_ray.get_direction().1.abs() < EPSILON_BUMP {
            None 
        }
        else {
            Some(vec![Intersection::new(-ray.get_origin().1 / ray.get_direction().1, Rc::clone(&object))])
        }
    }

    //The normal of a plane is always a vector pointing directly upwards
    fn normal(_object: &Rc<Object>, _world_point: &Vec4) -> Vec4 {
        Vec4::new(0.0, 1.0, 0.0, 0.0)
    }
}

impl Clone for Plane {
    //Clones a given plane
    fn clone(&self) -> Plane {
        Plane {
            transform: self.transform.clone(),
            material: self.material.clone(),
        }
    }
}