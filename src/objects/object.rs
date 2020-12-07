use crate::core::vector::Vec4;
use crate::ray_tracing::intersection::Intersection;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::ray::Ray;
use std::fmt::Debug;

//Trait which holds necessary methods for an object
pub trait Object: Debug {
    //Returns the object material
    fn get_material(&self) -> &Material;

    //Intersects a given object with a ray
    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>>;

    //Finds the normal of an object at a given point
    fn normal(&self, world_point: &Vec4) -> Vec4;
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectEnum {
    Sphere,
    Plane,
}