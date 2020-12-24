use crate::core::matrix::Matrix4x4;
use crate::core::vector::Vec4;
use crate::objects::object::*;
use crate::materials::material::*;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::intersection::Intersection;
use std::any::Any;

#[derive(Debug, PartialEq, Clone)]
pub struct Group {
    pub transform: Matrix4x4,
    pub inverse: Matrix4x4,
    pub material: Material,
    pub objects: Vec<Box<dyn Object>>,
    parent_inverses: Vec<Matrix4x4>,
}

impl Group {
    //Instantiates a Group with an identity Matrix as its transform 
    pub fn new(transform: Matrix4x4, material: Material) -> Group {
        let group = Group {
            inverse: transform.inverse().unwrap(),
            transform,
            material,
            objects: vec![],
            parent_inverses: vec![],
        };
        group
    }

    //Instantiates a Group with an identity Matrix as its transform 
    pub fn default() -> Group {
        Group {
            transform: Matrix4x4::identity(),
            inverse: Matrix4x4::identity(), 
            material: Material::default(),
            objects: vec![],
            parent_inverses: vec![],
        }
    }
}

impl<'a> Object for Group {
    //Returns the group material
    fn get_material(&self) -> &Material {
        &self.material
    }

    //Returns the group matrix
    fn get_inverse(&self) -> &Matrix4x4 {
        &self.inverse
    }

    //Intersects a ray with a group
    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let transformed_ray = Ray::transform(ray, &self.inverse);
        let mut intersections: Vec<Intersection> = vec![];
        for object in &self.objects {
            let object_intersections = object.intersect(&transformed_ray);
            if object_intersections != None {
                for intersection in object_intersections.unwrap() {
                    intersections.push(intersection);
                }
            }
        }
        if intersections.len() > 0 {
            return Some(intersections);
        }
        None
    }

    //The normal of a group is always a vector pointing directly upwards
    fn normal(&self, _world_point: &Vec4) -> Vec4 {
        panic!("Cannot find the normal of a group");
    }

    fn get_parent_inverses(&self) -> &Vec<Matrix4x4> {
        &self.parent_inverses
    }

    fn push_parent_inverse(&mut self, inverse: Matrix4x4) {
        self.parent_inverses.push(inverse.clone());
        for object in &mut self.objects {
            object.push_parent_inverse(inverse.clone());
        }
    }

    fn add_to_group(mut self, group: &mut Group) {
        self.push_parent_inverse(group.get_inverse().clone());
        group.objects.push(Box::new(self));
    }

    fn eq(&self, other: &dyn Object) -> bool {
        other.as_any().downcast_ref::<Self>().map_or(false, |x| x == self)
    }

    fn as_any(&self) -> &dyn Any { self }
}