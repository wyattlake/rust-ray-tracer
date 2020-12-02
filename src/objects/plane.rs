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
    pub fn new() -> Plane {
        Plane {
            transform: Matrix4x4::identity(),
            inverse: Matrix4x4::identity(),
            material: Material::default()
        }
    }

    //Applies a transformation to a Plane
    fn _transform(&mut self, matrix: Matrix4x4) {
        self.transform = matrix * self._get_transform();
    }

    //Returns the Plane transform
    fn _get_transform(&self) -> &Matrix4x4 {
        &self.transform
    }

    //Returns the Plane inverse
    fn _set_inverse(&mut self) {
        self.inverse = self.transform.inverse().unwrap();
    }

    //Sets the material of the plane
    fn _set_material(&mut self, material: Material) {
        self.material = material;
    }

    //Gets a mutable reference to the material
    fn _get_mut_material(&mut self) -> &mut Material {
        &mut self.material
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
            let i = Intersection::new(t, Ray::position(ray, t), &self.inverse, self.normal(&Ray::position(ray, t)), self.get_material());
            Some(vec![i])
        }
    }

    //The normal of a plane is always a vector pointing directly upwards
    fn normal(&self, _world_point: &Vec4) -> Vec4 {
        Vec4::new(0.0, 1.0, 0.0, 0.0)
    }
}