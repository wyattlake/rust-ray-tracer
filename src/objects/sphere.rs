use crate::core::matrix::Matrix4x4;
use crate::core::vector::Vec4;
use crate::objects::object::*;
use crate::ray_tracing::intersection::Intersection;
use crate::materials::material::*;
use crate::objects::group::Group;
use crate::ray_tracing::ray::Ray;
use std::any::Any;

//A sphere has a transform trait which keeps track of its transformations
#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    pub transform: Matrix4x4,
    pub inverse: Matrix4x4,
    pub material: Material,
    pub parent_inverses: Vec<Matrix4x4>,
}

impl Sphere {
    //Instantiates a sphere with an identity Matrix as its transform
    pub fn default() -> Sphere {
        Sphere {
            transform: Matrix4x4::identity(),
            inverse: Matrix4x4::identity(),
            material: Material::default(),
            parent_inverses: vec![],
        }
    }

    //Instantiates a sphere with an identity Matrix as its transform
    pub fn glass() -> Sphere {
        let mut material = Material::default();
        material.refractive_index = 1.5;
        material.transparency = 1.0;
        Sphere {
            transform: Matrix4x4::identity(),
            inverse: Matrix4x4::identity(),
            material: material,
            parent_inverses: vec![],
        }
    }

    pub fn new(transform: Matrix4x4, material: Material) -> Sphere {
        Sphere {
            inverse: transform.inverse().unwrap(),
            transform,
            material,
            parent_inverses: vec![],
        }
    }
}

impl Object for Sphere {
    //Returns the sphere material
    fn get_material(&self) -> &Material {
        &self.material
    }

    //Returns the plane material
    fn get_inverse(&self) -> &Matrix4x4 {
        &self.inverse
    }

    //Intersects a ray with a sphere
    fn intersect(&self, ray: &Ray) -> Option<Vec<Intersection>> {
        let transformed_ray = Ray::transform(ray, &self.inverse);
        let vector_to_unit_sphere = &transformed_ray.origin - Vec4::new(0.0, 0.0, 0.0, 1.0);
        let a = Vec4::dot(
            &transformed_ray.direction,
            &transformed_ray.direction,
        );
        let b = 2.0 * Vec4::dot(&transformed_ray.direction, &vector_to_unit_sphere);
        let c = Vec4::dot(&vector_to_unit_sphere, &vector_to_unit_sphere) - 1.0;
        let discriminant = (b * b) - (4.0 * a * c);

        //If the discriminant is less than zero then the point is imaginary
        if discriminant >= 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let i1 = Intersection::new(
                t1,
                Ray::position(&ray, t1),
                self.normal(&Ray::position(&ray, t1)),
                self,
            );
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            let i2 = Intersection::new(
                t2,
                Ray::position(&ray, t2),
                self.normal(&Ray::position(&ray, t2)),
                self,
            );
            Some(vec![i1, i2])
        } else {
            None
        }
    }

    //Finds the normal of a given point on a sphere
    fn normal(&self, world_point: &Vec4) -> Vec4 {
        println!("world_point: {:?}", world_point);
        //Applies inverse transformations to the point
        let group_point = world_to_object(&self.parent_inverses, world_point);
        println!("group_point: {:?}", group_point);
        let object_point = &self.inverse * group_point;
        let object_normal = object_point - Vec4::new(0.0, 0.0, 0.0, 1.0);
        //Computes the world normal
        let mut world_normal = &self.inverse.transpose() * object_normal;
        world_normal.3 = 0.0;
        let world_normal = world_normal.normalize();
        println!();
        normal_to_world(&self.parent_inverses, &world_normal)
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
