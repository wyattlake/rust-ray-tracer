use crate::core::matrix::Matrix4x4;
use crate::core::vector::Vec4;
use crate::ray_tracing::material::Material;
use crate::objects::object::*;
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::intersection::Intersection;

//A sphere has a transform trait which keeps track of its transformations
#[derive(Debug)]
pub struct Sphere {
    pub transform: Matrix4x4,
    pub inverse: Matrix4x4,
    pub material: Material,
}

impl Sphere {
    //Instantiates a sphere with an identity Matrix as its transform 
    pub fn default() -> Sphere {
        Sphere {
            transform: Matrix4x4::identity(),
            inverse: Matrix4x4::identity(), 
            material: Material::default()
        }
    }

    pub fn new(transform: Matrix4x4, material: Material) -> Sphere {
        Sphere {
            inverse: transform.inverse().unwrap(), 
            transform,
            material,
        }
    }
}

impl Object for Sphere {
    //Returns the sphere material
    fn get_material(&self) -> &Material {
        &self.material
    }

    //Intersects a ray with a sphere
    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let transformed_ray = Ray::transform(ray, &self.inverse);
        let vector_to_unit_sphere = transformed_ray.get_origin() - Vec4::new(0.0, 0.0, 0.0, 1.0);
        let a = Vec4::dot(&transformed_ray.get_direction(), &transformed_ray.get_direction());
        let b = 2.0 * Vec4::dot(&transformed_ray.get_direction(), &vector_to_unit_sphere);
        let c = Vec4::dot(&vector_to_unit_sphere, &vector_to_unit_sphere) - 1.0;
        let discriminant = (b * b) - (4.0 * a * c);

        //If the discriminant is less than zero then the point is imaginary
        if discriminant >= 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let i1 = Intersection::new(t1, Ray::position(ray, t1), &self.inverse, self.normal(&Ray::position(ray, t1)), self.get_material());
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            let i2 = Intersection::new(t2, Ray::position(ray, t2), &self.inverse, self.normal(&Ray::position(ray, t2)), self.get_material());
            Some(vec![i1, i2])
        }
        else {
            None
        }
    }

    //Finds the normal of a given point on a sphere
    fn normal(&self, world_point: &Vec4) -> Vec4 {
        //Applies inverse transformations to the point
        let object_point = &self.inverse * world_point;
        let object_normal = object_point - Vec4::new(0.0, 0.0, 0.0, 1.0);
        //Computes the world normal
        let mut world_normal = &self.inverse.transpose() * object_normal; 
        world_normal.3 = 0.0;
        world_normal.normalize()
    }
}