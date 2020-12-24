use crate::core::matrix::Matrix4x4;
use crate::core::vector::Vec4;
use crate::objects::object::*;
use crate::materials::material::*;
use crate::ray_tracing::ray::Ray;
use crate::objects::group::Group;
use crate::ray_tracing::intersection::Intersection;
use crate::misc::utils::*;
use std::any::Any;

#[derive(Debug, PartialEq, Clone)]
pub struct Plane {
    pub transform: Matrix4x4,
    pub inverse: Matrix4x4,
    pub material: Material,
    pub parent_inverses: Vec<Matrix4x4>,
}

impl Plane {
    //Instantiates a Plane with an identity Matrix as its transform 
    pub fn new(transform: Matrix4x4, material: Material) -> Plane {
        Plane {
            inverse: transform.inverse().unwrap(),
            transform,
            material,
            parent_inverses: vec![],
        }
    }

    //Instantiates a Plane with an identity Matrix as its transform 
    pub fn default() -> Plane {
        Plane {
            transform: Matrix4x4::identity(),
            inverse: Matrix4x4::identity(), 
            material: Material::default(),
            parent_inverses: vec![],
        }
    }
}

impl Object for Plane {
    //Returns the plane material
    fn get_material(&self) -> &Material {
        &self.material
    }

    //Returns the plane matrix
    fn get_inverse(&self) -> &Matrix4x4 {
        &self.inverse
    }

    //Intersects a ray with a plane
    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let transformed_ray = Ray::transform(ray, &self.inverse);
        if transformed_ray.origin.1.abs() < EPSILON_BUMP {
            None 
        }
        else {
            let t = -&transformed_ray.origin.1 / &transformed_ray.direction.1;
            let i = Intersection::new(t, Ray::position(&ray, t), self.normal(&Ray::position(&ray, t)), self);
            Some(vec![i])
        }
    }

    //The normal of a plane is always a vector pointing directly upwards
    fn normal(&self, _world_point: &Vec4) -> Vec4 {
        let mut result = &self.inverse.transpose() * Vec4::new(0.0, 1.0, 0.0, 0.0);
        result.3 = 0.0;
        normal_to_world(&self.parent_inverses, &result.normalize())
    }

    fn get_parent_inverses(&self) -> &Vec<Matrix4x4> {
        &self.parent_inverses
    }

    fn push_parent_inverse(&mut self, inverse: Matrix4x4) {
        self.parent_inverses.push(inverse);
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