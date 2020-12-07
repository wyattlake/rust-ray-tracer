use crate::core::matrix::Matrix4x4;
use crate::core::vector::Vec4;
use crate::objects::object::*;
use crate::ray_tracing::material::Material;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::intersection::Intersection;
use crate::misc::utils::*;

#[derive(Debug)]
pub struct Plane {
    pub transform: Matrix4x4,
    pub inverse: Matrix4x4,
    pub material: Material,
}

impl Plane {
    //Instantiates a Plane with an identity Matrix as its transform 
    pub fn new(transform: Matrix4x4, material: Material) -> Plane {
        Plane {
            inverse: transform.inverse().unwrap(),
            transform,
            material,
        }
    }

    //Instantiates a Plane with an identity Matrix as its transform 
    pub fn default() -> Plane {
        Plane {
            transform: Matrix4x4::identity(),
            inverse: Matrix4x4::identity(), 
            material: Material::default()
        }
    }
}

impl Object for Plane {
    //Returns the plane material
    fn get_material(&self) -> &Material {
        &self.material
    }

    //Intersects a ray with a plane
    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let transformed_ray = Ray::transform(ray, &self.inverse);
        if transformed_ray.get_direction().1.abs() <= EPSILON_BUMP {
            None 
        }
        else {
            let t = -transformed_ray.get_origin().1 / transformed_ray.get_direction().1;
            let i = Intersection::new(t, Ray::position(ray, t), &self.inverse, self.normal(&Ray::position(ray, t)), self.get_material(), &ObjectEnum::Plane);
            Some(vec![i])
        }
    }

    //The normal of a plane is always a vector pointing directly upwards
    fn normal(&self, _world_point: &Vec4) -> Vec4 {
        Vec4::new(0.0, 1.0, 0.0, 0.0)
    }
}