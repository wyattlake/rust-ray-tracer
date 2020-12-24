use crate::core::vector::Vec4;
use crate::core::matrix::Matrix4x4;
use crate::ray_tracing::intersection::Intersection;
use crate::materials::material::*;
use crate::ray_tracing::ray::Ray;
use crate::objects::group::Group;
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

    //Adds a given object to a group
    fn add_to_group(self, group: &mut Group);

    //Modifiers for the object's parent inverse list
    fn get_parent_inverses(&self) -> &Vec<Matrix4x4>;
    fn push_parent_inverse(&mut self, inverse: Matrix4x4);

    //Methods used to allow PartialEq between objects
    fn eq(&self, other: &dyn Object) -> bool;
    fn as_any(&self) -> &dyn Any;
}

pub fn world_to_object(list: &Vec<Matrix4x4>, point: &Vec4) -> Vec4 {
    let mut transformed_point = point.clone();
    let mut reversed = list.clone();
    reversed.reverse();
    for inverse in reversed {
        transformed_point = inverse * transformed_point;        
    }
    transformed_point
}

pub fn normal_to_world(list: &Vec<Matrix4x4>, normal: &Vec4) -> Vec4 {
    let mut transformed_normal = normal.clone();
    for inverse in list {
        transformed_normal = inverse.transpose() * transformed_normal;
        transformed_normal.3 = 0.0;
        transformed_normal = transformed_normal.normalize();
    }
    transformed_normal.normalize()
}

//PartialEq for trait objects
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