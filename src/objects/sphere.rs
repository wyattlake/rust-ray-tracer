use crate::core::matrix::Matrix4x4;
use crate::core::vector::Vec4;
use crate::ray_tracing::material::Material;
use crate::objects::general::{ObjectMethods, Object};
use crate::ray_tracing::ray::Ray;
use crate::ray_tracing::intersection::Intersection;
use std::rc::Rc;

//A sphere has a transform trait which keeps track of its transformations
#[derive(Debug, PartialEq)]
pub struct Sphere {
    transform: Matrix4x4,
    material: Material,
}

impl Sphere {
    pub fn intersect(object: Rc<Object>, ray: &Ray) -> Option<Vec<Intersection>> {
        let transformed_ray = Ray::transform(ray, &((object.get_transform()).inverse().unwrap()));
        let vector_to_unit_sphere = transformed_ray.get_origin() - Vec4::new(0.0, 0.0, 0.0, 1.0);
        let a = Vec4::dot(&transformed_ray.get_direction(), &transformed_ray.get_direction());
        let b = 2.0 * Vec4::dot(&transformed_ray.get_direction(), &vector_to_unit_sphere);
        let c = Vec4::dot(&vector_to_unit_sphere, &vector_to_unit_sphere) - 1.0;
        let discriminant = (b * b) - (4.0 * a * c);
        if discriminant >= 0.0 {
            Some(vec![(Intersection::new((- b - (discriminant.sqrt())) / (2.0 * a), Rc::clone(&object))), (Intersection::new((- b + (discriminant.sqrt())) / (2.0 * a), Rc::clone(&object)))])
        }
        else {
            None
        }
    }

    //Finds the normal of a given point on a sphere
    pub fn normal(object: &Rc<Object>, world_point: &Vec4) -> Vec4 {
        let object_point = (object.get_transform()).inverse().unwrap() * world_point;
        let object_normal = object_point - Vec4::new(0.0, 0.0, 0.0, 1.0);
        let mut world_normal = (object.get_transform()).inverse().unwrap().transpose() * object_normal; 
        world_normal.3 = 0.0;
        world_normal.normalize()
    }
}

impl ObjectMethods for Sphere {
    //Instantiates a Sphere with a Rc and an identity Matrix as its transform
    fn new() -> Rc<Object> {
        Rc::new(
            Object::Sphere(Sphere {
                transform: Matrix4x4::identity(),
                material: Material::default()
            })
        )
    }

    //Instantiates a sphere with an identity Matrix as its transform 
    fn new_raw() -> Object {
        Object::Sphere(Sphere {
            transform: Matrix4x4::identity(),
            material: Material::default()
        })
    }

    //Applies a transformation to a sphere
    fn transform(&mut self, matrix: Matrix4x4) {
        self.transform = matrix * self.transform.clone();
    }

    //Returns the sphere transform
    fn get_transform(&self) -> &Matrix4x4 {
        &self.transform
    }

    //Returns the sphere material
    fn get_material(&self) -> &Material {
        &self.material
    }

    //Sets the material of the sphere
    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    //Gets a mutable reference to the material
    fn get_mut_material(&mut self) -> &mut Material {
        &mut self.material
    }
}

impl Clone for Sphere {
    //Clones a given sphere
    fn clone(&self) -> Sphere {
        Sphere {
            transform: self.transform.clone(),
            material: self.material.clone(),
        }
    }
}