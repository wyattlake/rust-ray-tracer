use crate::core::vector::Vec4;
use crate::core::matrix::Matrix4x4;
use crate::ray_tracing::intersection::Intersection;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::ray::Ray;
use std::fmt::Debug;
use std::any::Any;

//Trait which holds necessary methods for an object
pub trait Object: Debug + ObjectClone {
    //Returns the object material
    fn get_material(&self) -> &Material;

    fn get_inverse(&self) -> &Matrix4x4;
    
    //Intersects a given object with a ray
    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>>;

    //Finds the normal of an object at a given point
    fn normal(&self, world_point: &Vec4) -> Vec4;

    //Methods used to allow PartialEq between objects
    fn eq(&self, other: &dyn Object) -> bool;
    fn as_any(&self) -> &dyn Any;
}

impl<'a, 'b> PartialEq<dyn Object+'b> for dyn Object+'a {
    fn eq(&self, other: &(dyn Object+'b)) -> bool {
        Object::eq(self, other)
    }
}

pub trait ObjectClone {
    fn clone_box(&self) -> Box<dyn Object>;
}

impl<T> ObjectClone for T where T: 'static + Object + Clone, {
    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Object> {
    fn clone(&self) -> Box<dyn Object> {
        self.clone_box()
    }
}